# Project Goal: `qsharp` Standalone CLI

## Vision
To provide the quantum developer community with a first-class, lightning-fast, zero-dependency standalone command-line compiler and runner for modern Q# programs.

## Key Objectives
1. **Developer Simplicity**: Enable users to run any Q# file with a single command:
   ```bash
   qsharp <file.qs>
   ```
2. **Zero Dependencies**: Pure standalone native binary (Windows `.exe`, Linux, macOS) with no requirement for .NET SDK, Python, or VS Code.
3. **Full Language Support**: Support the entire modern Q# language specification, standard library, and preview features (e.g. `Std.Diagnostics`, `Std.Intrinsic`, `Std.Math`).
4. **Distribution Ready**:
   - Publish to Crates.io (`cargo install qsharp`) or GitHub Releases.
   - Cross-platform automated binary builds via GitHub Actions (Windows x64/arm64, Linux x64/arm64, macOS x64/arm64).
