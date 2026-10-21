use std::env;
use std::fs;
use std::io::Cursor;
use qsc::interpret::Interpreter;
use qsc_data_structures::{language_features::LanguageFeatures, target::TargetCapabilityFlags};
use qsc_eval::output::CursorReceiver;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: qsharp <file.qs>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let source = fs::read_to_string(file_path)?;

    let capabilities = TargetCapabilityFlags::all();
    let (std_id, store) = qsc::compile::package_store_with_stdlib(capabilities);

    let mut interpreter = Interpreter::new(
        qsc::SourceMap::default(),
        qsc::PackageType::Lib,
        capabilities,
        LanguageFeatures::default(),
        store,
        &[(std_id, None)],
        Default::default(),
    )
    .map_err(|e| format!("Initialization error: {:?}", e))?;

    let mut cursor = Cursor::new(Vec::<u8>::new());
    let mut receiver = CursorReceiver::new(&mut cursor);

    let res = interpreter.eval_fragments(&mut receiver, &source);
    let output = receiver.dump();
    if !output.is_empty() {
        print!("{}", output);
    }

    if let Err(errors) = res {
        eprintln!("Errors in Q# source: {:?}", errors);
        std::process::exit(1);
    }

    let run_res = interpreter.run(
        &mut receiver,
        Some("Main()"),
        None,
        None,
        None,
        None,
        Default::default(),
    );

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

    Ok(())
}
