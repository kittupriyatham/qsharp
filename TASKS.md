# Roadmap & Tasks: `qsharp` Standalone CLI

## Phase 1: Core Functionality (Current)
- [x] Basic execution of single `.qs` files with `@EntryPoint()`.
- [x] Integration with Microsoft's `qsc` crate directly from GitHub.
- [x] Standard library loading (`package_store_with_stdlib`).
- [x] Streaming output from `Message(...)` and capturing return values.

## Phase 2: CLI UX & Argument Parsing
- [x] Add `clap` for rich command-line argument parsing:
  - `qsharp [run] <file.qs>` (default action).
  - `--shots <N>` / `-s <N>`: Run simulations across multiple shots with histogram summary.
  - `--entry <operation>` / `-e <op>`: Specify which operation to execute (defaults to `@EntryPoint()` or `Main()`).
  - `--version` / `-V`: Print `qsharp` version and linked `qsc` commit.
  - `--help` / `-h`: Formatted help text and examples.

## Phase 3: Advanced Capabilities
- [x] Multi-file and Project Support:
  - Support passing multiple `.qs` files (`qsharp run file1.qs file2.qs`).
  - Detect and parse `qsharp.json` project manifests when present.
- [x] Simulation Configs:
  - Noise model flags (`--noise px,py,pz`).
  - Seed control (`--seed <u64>`).
  - Simulator types (`--sim sparse|stabilizer`).
- [x] Resource Estimation & Circuits:
  - `qsharp estimate <file.qs>`: Output logical and physical qubit resource counts in terminal.
  - `qsharp circuit <file.qs>`: Output ASCII/text or JSON quantum circuit diagram.

## Phase 4: CI/CD & Releases
- [x] Add GitHub Actions release workflow building for Windows, Ubuntu, and macOS.
- [x] Configure WiX Windows installer packaging generating `qsharp-windows-x64.msi` (with PATH integration).
- [x] Add automated GitHub Release notes generator summarizing new features per release tag (`generate_release_notes: true`).
- [x] Add pre-merge CI validation workflow (cross-platform format check, clippy `-D warnings`, test suite).
- [x] Add PR template (`.github/pull_request_template.md`) and Conventional Commits guidelines in `CONTRIBUTING.md`.
