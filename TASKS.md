# Roadmap & Tasks: `qsharp` Standalone CLI

## Phase 1: Core Functionality (Current)
- [x] Basic execution of single `.qs` files with `@EntryPoint()`.
- [x] Integration with Microsoft's `qsc` crate directly from GitHub.
- [x] Standard library loading (`package_store_with_stdlib`).
- [x] Streaming output from `Message(...)` and capturing return values.

## Phase 2: CLI UX & Argument Parsing
- [ ] Add `clap` for rich command-line argument parsing:
  - `qsharp run <file.qs>` (default action).
  - `--shots <N>` / `-s <N>`: Run simulations across multiple shots with histogram summary.
  - `--entry <operation>` / `-e <op>`: Specify which operation to execute (defaults to `@EntryPoint()` or `Main()`).
  - `--version` / `-V`: Print `qsharp` version and linked `qsc` commit.
  - `--help` / `-h`: Formatted help text and examples.

## Phase 3: Advanced Capabilities
- [ ] Multi-file and Project Support:
  - Support passing multiple `.qs` files (`qsharp run file1.qs file2.qs`).
  - Detect and parse `qsharp.json` project manifests when present.
- [ ] Simulation Configs:
  - Noise model flags (`--noise <model>`).
  - Seed control (`--seed <u64>`).
  - Simulator types (`--sim sparse|stabilizer`).
- [ ] Resource Estimation & Circuits:
  - `qsharp estimate <file.qs>`: Output logical and physical qubit resource counts in terminal.
  - `qsharp circuit <file.qs>`: Output ASCII/text or JSON quantum circuit diagram.

## Phase 4: CI/CD & Releases
- [ ] Add GitHub Actions CI workflow to test across Windows, Ubuntu, and macOS.
- [ ] Set up automated release workflow attaching pre-built standalone binaries on git tag.
- [ ] Publish documentation and installation guide (`cargo install`, Homebrew formula, winget).
