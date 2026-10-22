use clap::{Args, Parser, Subcommand, ValueEnum};
use qsc::interpret::{CircuitEntryPoint, CircuitGenerationMethod, Interpreter, SimType};
use qsc::PauliNoise;
use qsc_circuit::TracerConfig;
use qsc_data_structures::{language_features::LanguageFeatures, target::TargetCapabilityFlags};
use qsc_eval::output::CursorReceiver;
use qsc_project::Manifest;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    name = "qsharp",
    version,
    about = "A fast, lightweight, standalone command-line compiler and runner for Q#",
    long_about = "A standalone CLI for Microsoft Q# built on the official Rust-based QDK (qsc) engine."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    run_args: RunArgs,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a Q# program (default action)
    Run(RunArgs),

    /// Generate and render a quantum circuit diagram for the Q# program
    Circuit(CircuitArgs),

    /// Perform physical and logical quantum resource estimation
    Estimate(EstimateArgs),
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum SimBackend {
    /// Full state-vector simulation (default)
    Sparse,
    /// Fast Clifford / Stabilizer simulation for Clifford circuits
    Stabilizer,
}

#[derive(Args, Debug, Clone)]
struct RunArgs {
    /// Path to one or more Q# (.qs) files to compile and execute
    #[arg(value_name = "FILES")]
    files: Vec<PathBuf>,

    /// Specific operation or expression to execute (e.g. -e "MyOperation()")
    #[arg(short = 'e', long = "entry", value_name = "EXPR")]
    entry: Option<String>,

    /// Number of times to simulate the program
    #[arg(short = 's', long = "shots", default_value = "1", value_name = "N")]
    shots: usize,

    /// Seed for random number generation (ensures deterministic output)
    #[arg(long = "seed", value_name = "U64")]
    seed: Option<u64>,

    /// Simulator backend (sparse or stabilizer)
    #[arg(long = "sim", value_enum, default_value_t = SimBackend::Sparse)]
    sim: SimBackend,

    /// Pauli noise probabilities: px,py,pz (e.g. --noise 0.01,0.01,0.01)
    #[arg(long = "noise", value_name = "PX,PY,PZ")]
    noise: Option<String>,
}

#[derive(Args, Debug, Clone)]
struct CircuitArgs {
    /// Path to one or more Q# (.qs) files
    #[arg(value_name = "FILES")]
    files: Vec<PathBuf>,

    /// Specific operation or expression to generate circuit for
    #[arg(short = 'e', long = "entry", value_name = "EXPR")]
    entry: Option<String>,

    /// Output circuit as raw JSON representation instead of ASCII diagram
    #[arg(long = "json")]
    json: bool,
}

#[derive(Args, Debug, Clone)]
struct EstimateArgs {
    /// Path to one or more Q# (.qs) files
    #[arg(value_name = "FILES")]
    files: Vec<PathBuf>,

    /// Specific operation or expression to estimate resources for
    #[arg(short = 'e', long = "entry", value_name = "EXPR")]
    entry: Option<String>,

    /// Estimation parameters JSON string or path to JSON file
    #[arg(long = "params", value_name = "JSON_OR_PATH")]
    params: Option<String>,

    /// Output raw JSON estimation data instead of human-readable summary
    #[arg(long = "json")]
    json: bool,
}

fn parse_noise(s: &str) -> Result<PauliNoise, String> {
    let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
    if parts.len() != 3 {
        return Err("Noise must be in format 'px,py,pz' (e.g. --noise 0.01,0.01,0.01)".to_string());
    }
    let px: f64 = parts[0]
        .parse()
        .map_err(|_| format!("Invalid px value: '{}'", parts[0]))?;
    let py: f64 = parts[1]
        .parse()
        .map_err(|_| format!("Invalid py value: '{}'", parts[1]))?;
    let pz: f64 = parts[2]
        .parse()
        .map_err(|_| format!("Invalid pz value: '{}'", parts[2]))?;

    PauliNoise::from_probabilities(px, py, pz)
}

/// Checks if a Q# source file contains an entry point:
/// either `@EntryPoint()` attribute or `operation Main(` / `function Main(`.
fn file_has_main_or_entry(content: &str) -> bool {
    let has_entry_attr = content.contains("@EntryPoint");
    let has_main_op = content.lines().any(|line| {
        let trimmed = line.trim();
        (trimmed.starts_with("operation Main") || trimmed.starts_with("function Main"))
            && (trimmed.contains('(') || trimmed.contains(':'))
    });
    has_entry_attr || has_main_op
}

/// Discovers candidate .qs files in the current directory having a Main operation / @EntryPoint
fn find_main_files_in_dir(dir: &Path) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("qs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if file_has_main_or_entry(&content) {
                        candidates.push(path);
                    }
                }
            }
        }
    }
    candidates
}

/// Discovers all `.qs` files in a directory and any `src` subdirectories.
fn find_all_qs_files_in_dir(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let src_dir = dir.join("src");
    let target_dir = if src_dir.is_dir() { &src_dir } else { dir };

    if let Ok(entries) = fs::read_dir(target_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("qs") {
                files.push(path);
            }
        }
    }
    files
}

/// Resolves target source files given CLI inputs and/or manifest discovery.
fn resolve_target_files(
    input_files: &[PathBuf],
    is_explicit_run_cmd: bool,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;

    if !input_files.is_empty() {
        if is_explicit_run_cmd {
            eprintln!(
                "warning: running '{}' explicitly; it is intended to execute the Main operation / @EntryPoint.",
                input_files[0].display()
            );
        }
        for f in input_files {
            if !f.exists() {
                eprintln!("Error: File '{}' not found.", f.display());
                std::process::exit(1);
            }
        }
        return Ok(input_files.to_vec());
    }

    // Check for qsharp.json in current directory or parent directory
    let manifest_path = current_dir.join("qsharp.json");
    if manifest_path.is_file() {
        let manifest_content = fs::read_to_string(&manifest_path)?;
        let manifest: Manifest = serde_json::from_str(&manifest_content)
            .map_err(|e| format!("Failed to parse qsharp.json: {e}"))?;

        if !manifest.files.is_empty() {
            let files: Vec<PathBuf> = manifest
                .files
                .into_iter()
                .map(|f| current_dir.join(f))
                .collect();
            println!(
                "info: Discovered 'qsharp.json' manifest with {} source file(s).",
                files.len()
            );
            return Ok(files);
        } else {
            // Find files in src/ or current_dir
            let files = find_all_qs_files_in_dir(&current_dir);
            if !files.is_empty() {
                println!(
                    "info: Discovered 'qsharp.json' manifest; loaded {} source file(s).",
                    files.len()
                );
                return Ok(files);
            }
        }
    }

    // Inspect directory for single vs multiple main candidate files
    let candidates = find_main_files_in_dir(&current_dir);
    match candidates.len() {
        0 => {
            eprintln!(
                "Error: No Q# file provided and no .qs file containing a Main operation or @EntryPoint() found in '{}'.\nUsage: qsharp [OPTIONS] <FILE> or qsharp run [OPTIONS] <FILE>\nRun 'qsharp --help' for details.",
                current_dir.display()
            );
            std::process::exit(1);
        }
        1 => {
            let auto_path = candidates.into_iter().next().unwrap();
            println!(
                "info: No file specified; automatically selected '{}' containing Main operation.",
                auto_path.display()
            );
            Ok(vec![auto_path])
        }
        _ => {
            eprintln!(
                "Error: Multiple Q# files containing Main operations or @EntryPoint() found in '{}':",
                current_dir.display()
            );
            for c in &candidates {
                eprintln!("  - {}", c.display());
            }
            eprintln!("Please specify which file to run: qsharp run <FILE>");
            std::process::exit(1);
        }
    }
}

/// Initializes an interpreter and compiles the provided files into it.
fn init_interpreter_with_sources(
    files: &[PathBuf],
) -> Result<Interpreter, Box<dyn std::error::Error>> {
    let capabilities = TargetCapabilityFlags::all();
    let (std_id, store) = qsc::compile::package_store_with_stdlib(capabilities);

    let mut sources_vec = Vec::new();
    for file_path in files {
        let content = fs::read_to_string(file_path)?;
        sources_vec.push((
            file_path.to_string_lossy().to_string().into(),
            content.into(),
        ));
    }

    let source_map = qsc::SourceMap::new(sources_vec, None);

    let interpreter = Interpreter::new(
        source_map,
        qsc::PackageType::Lib,
        capabilities,
        LanguageFeatures::default(),
        store,
        &[(std_id, None)],
        Default::default(),
    )
    .map_err(|errors| {
        format!(
            "Compilation errors:\n{}",
            errors
                .iter()
                .map(|e| format!("  - {:?}", e))
                .collect::<Vec<_>>()
                .join("\n")
        )
    })?;

    Ok(interpreter)
}

fn execute_run(
    run_args: RunArgs,
    is_explicit_run_cmd: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let files = resolve_target_files(&run_args.files, is_explicit_run_cmd)?;
    let mut interpreter = init_interpreter_with_sources(&files)?;

    let pauli_noise = match run_args.noise {
        Some(ref n) => Some(parse_noise(n)?),
        None => None,
    };

    let sim_type = match run_args.sim {
        SimBackend::Sparse => SimType::Sparse,
        SimBackend::Stabilizer => SimType::Clifford(64),
    };

    let mut cursor = Cursor::new(Vec::<u8>::new());
    let mut receiver = CursorReceiver::new(&mut cursor);

    // Determine entry expression
    let entry_expr_owned: String;
    let entry_expr = if let Some(ref e) = run_args.entry {
        e.as_str()
    } else {
        // Find if any file has an explicit Main or @EntryPoint
        let mut main_file_stem = None;
        for file in &files {
            if let Ok(content) = fs::read_to_string(file) {
                if file_has_main_or_entry(&content) {
                    if let Some(stem) = file.file_stem().and_then(|s| s.to_str()) {
                        main_file_stem = Some(stem.to_string());
                        break;
                    }
                }
            }
        }
        entry_expr_owned = match main_file_stem {
            Some(stem) => format!("{}.Main()", stem),
            None => "Main()".to_string(),
        };
        entry_expr_owned.as_str()
    };
    let shots = run_args.shots.max(1);

    if shots == 1 {
        // Try candidate entry_expr, fallback to "Main()" if namespaced resolution failed
        let run_res = interpreter
            .run(
                &mut receiver,
                Some(entry_expr),
                pauli_noise,
                None,
                None,
                run_args.seed,
                sim_type,
            )
            .or_else(|_| {
                interpreter.run(
                    &mut receiver,
                    Some("Main()"),
                    pauli_noise,
                    None,
                    None,
                    run_args.seed,
                    sim_type,
                )
            });

        let run_output = receiver.dump();
        if !run_output.is_empty() {
            print!("{}", run_output);
        }

        match run_res {
            Ok(val) => {
                if val != qsc_eval::val::Value::unit() {
                    println!("\nResult: {}", val);
                }
            }
            Err(err) => {
                eprintln!("\nRuntime error: {:?}", err);
                std::process::exit(1);
            }
        }
    } else {
        // Multi-shot execution with histogram aggregation
        let mut histogram: BTreeMap<String, usize> = BTreeMap::new();
        println!(
            "Running {} shots with simulator {:?}...\n",
            shots, run_args.sim
        );

        for shot in 0..shots {
            let seed = run_args.seed.map(|s| s.wrapping_add(shot as u64));
            let run_res = interpreter
                .run(
                    &mut receiver,
                    Some(entry_expr),
                    pauli_noise,
                    None,
                    None,
                    seed,
                    sim_type,
                )
                .or_else(|_| {
                    interpreter.run(
                        &mut receiver,
                        Some("Main()"),
                        pauli_noise,
                        None,
                        None,
                        seed,
                        sim_type,
                    )
                });

            // Print any Message(...) outputs only on the first shot to avoid flooding stdout
            let run_output = receiver.dump();
            if shot == 0 && !run_output.is_empty() {
                println!("--- Output from Shot 1 ---");
                print!("{}", run_output);
                println!("--------------------------\n");
            }

            match run_res {
                Ok(val) => {
                    let key = if val == qsc_eval::val::Value::unit() {
                        "()".to_string()
                    } else {
                        val.to_string()
                    };
                    *histogram.entry(key).or_insert(0) += 1;
                }
                Err(err) => {
                    eprintln!("Runtime error on shot {}: {:?}", shot + 1, err);
                    std::process::exit(1);
                }
            }
        }

        println!("=== Shot Summary ({} shots) ===", shots);
        for (val, count) in &histogram {
            let percentage = (*count as f64 / shots as f64) * 100.0;
            println!("{:>10} : {:>6} shots ({:>6.2}%)", val, count, percentage);
        }
    }

    Ok(())
}

fn execute_circuit(args: CircuitArgs) -> Result<(), Box<dyn std::error::Error>> {
    let files = resolve_target_files(&args.files, false)?;
    let mut interpreter = init_interpreter_with_sources(&files)?;

    let entry = match args.entry {
        Some(ref expr) => CircuitEntryPoint::EntryExpr(expr.clone()),
        None => CircuitEntryPoint::EntryExpr("Main()".to_string()),
    };

    let tracer_config = TracerConfig {
        max_operations: TracerConfig::DEFAULT_MAX_OPERATIONS,
        max_loop_iterations: None,
        source_locations: false,
        group_by_scope: false,
        prune_classical_qubits: false,
    };

    let circuit = interpreter
        .circuit(entry, CircuitGenerationMethod::Simulate, tracer_config)
        .map_err(|e| format!("Circuit generation error: {:?}", e))?;

    if args.json {
        let json_str = serde_json::to_string_pretty(&circuit)?;
        println!("{}", json_str);
    } else {
        println!("{}", circuit);
    }

    Ok(())
}

fn execute_estimate(args: EstimateArgs) -> Result<(), Box<dyn std::error::Error>> {
    let files = resolve_target_files(&args.files, false)?;
    let mut interpreter = init_interpreter_with_sources(&files)?;

    let expr = args.entry.as_deref().unwrap_or("Main()");

    let params_str = match args.params {
        Some(ref p) => {
            if Path::new(p).exists() {
                fs::read_to_string(p)?
            } else {
                p.clone()
            }
        }
        None => String::new(),
    };

    let result_json = resource_estimator::estimate_expr(&mut interpreter, expr, &params_str)
        .map_err(|e| format!("Resource estimation error: {:?}", e))?;

    if args.json {
        println!("{}", result_json);
    } else {
        // Pretty formatted summary table
        if let Ok(parsed) = serde_json::from_str::<JsonValue>(&result_json) {
            if let Some(first) = parsed.as_array().and_then(|arr| arr.first()) {
                println!("=== Quantum Resource Estimation ===");
                if let Some(status) = first.get("status").and_then(|s| s.as_str()) {
                    println!("Status: {}", status);
                }
                if let Some(phys) = first.get("physicalCountsFormatted") {
                    println!("\n[Physical Resource Estimates]");
                    if let Some(v) = phys.get("runtime").and_then(|x| x.as_str()) {
                        println!("  Runtime:               {}", v);
                    }
                    if let Some(v) = phys.get("rqops").and_then(|x| x.as_str()) {
                        println!("  rQOPS:                 {}", v);
                    }
                    if let Some(v) = phys.get("physicalQubits").and_then(|x| x.as_str()) {
                        println!("  Physical Qubits:       {}", v);
                    }
                    if let Some(v) = phys
                        .get("algorithmicLogicalQubits")
                        .and_then(|x| x.as_str())
                    {
                        println!("  Algorithmic Qubits:    {}", v);
                    }
                    if let Some(v) = phys.get("algorithmicLogicalDepth").and_then(|x| x.as_str()) {
                        println!("  Algorithmic Depth:     {}", v);
                    }
                    if let Some(v) = phys.get("numTstates").and_then(|x| x.as_str()) {
                        println!("  Number of T states:    {}", v);
                    }
                    if let Some(v) = phys.get("numTfactories").and_then(|x| x.as_str()) {
                        println!("  T Factories:           {}", v);
                    }
                }
                if let Some(logical) = first.get("logicalCounts") {
                    println!("\n[Pre-Layout Logical Resources]");
                    if let Some(v) = logical.get("numQubits").and_then(|x| x.as_u64()) {
                        println!("  Logical Qubits:        {}", v);
                    }
                    if let Some(t) = logical.get("tCount").and_then(|x| x.as_u64()) {
                        println!("  T Gates:               {}", t);
                    }
                    if let Some(r) = logical.get("rotationCount").and_then(|x| x.as_u64()) {
                        println!("  Rotation Gates:        {}", r);
                    }
                    if let Some(m) = logical.get("measurementCount").and_then(|x| x.as_u64()) {
                        println!("  Measurements:          {}", m);
                    }
                }
                println!("\n(Use --json for complete raw estimation dataset)");
                return Ok(());
            }
        }
        println!("{}", result_json);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run(args)) => execute_run(args, true),
        Some(Commands::Circuit(args)) => execute_circuit(args),
        Some(Commands::Estimate(args)) => execute_estimate(args),
        None => execute_run(cli.run_args, false),
    }
}
