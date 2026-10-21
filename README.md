# `qsharp` - Standalone Modern Q# CLI

A fast, lightweight, standalone command-line compiler and runner for the [Q# Quantum Programming Language](https://github.com/microsoft/qdk), built with Rust.

## Features
- **Zero Dependencies**: Pure native binary with no need for .NET SDK, C# runtime, Python, or VS Code.
- **Modern QDK Engine**: Directly links against Microsoft's official Rust-based `qsc` compiler and simulator.
- **Instant Execution**: Sub-second compile-and-run cycles.

## Quick Start

### Build & Install
```bash
cargo build --release
cargo install --path .
```

### Run a Q# Program
```bash
qsharp program.qs
```

### Example
```qsharp
@EntryPoint()
operation Main() : Result {
    Message("Hello Quantum World!");
    use q = Qubit();
    H(q);
    let r = M(q);
    Message($"Measured: {r}");
    Reset(q);
    return r;
}
```

```bash
$ qsharp program.qs
Hello Quantum World!
Measured: One

Result: One
```

## Documentation
- [CONTEXT.md](CONTEXT.md): Background and problem statement.
- [GOAL.md](GOAL.md): Project vision and objectives.
- [TASKS.md](TASKS.md): Development roadmap.
