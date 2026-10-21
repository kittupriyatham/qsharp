# Context: Standalone Modern Q# CLI (`qsharp`)

## Background
In 2023–2024, Microsoft completely re-architected the Quantum Development Kit (QDK) from the ground up in Rust (hosted at [microsoft/qdk](https://github.com/microsoft/qdk)). This modern engine replaced the legacy C# / .NET SDK (`Microsoft.Quantum.Sdk 0.28.x` on `.NET 6.0`) with a high-performance, instant compiler and simulator.

## The Missing Piece
Despite the modern rewrite:
1. Microsoft **did not ship a standalone command-line executable** (like `rustc`, `python`, `cargo`, or `dotnet`) for running `.qs` files directly.
2. The modern toolchain is distributed strictly as:
   - A VS Code extension (GUI button clicks)
   - Python bindings (`qdk` library via PyPI)
   - WebAssembly / Browser playgrounds
3. Developers wanting a simple, fast `qsharp run file.qs` CLI tool without .NET or Python wrappers were left without an official solution.

## Solution Architecture
This repository (`qsharp`) creates a lightweight, zero-overhead native Rust CLI binary that directly links against Microsoft's official `qsc` (Q# compiler), `qsc_eval`, and `qsc_data_structures` crates:

- **Source:** Directly targets `https://github.com/microsoft/qdk.git`.
- **Engine:** Uses Microsoft's official `qsc::interpret::Interpreter` and standard library bundle (`qsc::compile::package_store_with_stdlib`).
- **I/O:** Implements `qsc_eval::output::CursorReceiver` to capture and stream `Message(...)` outputs in real time.
- **Portability:** Builds as a single static binary with no external runtimes (.NET, Python, etc.) required.
