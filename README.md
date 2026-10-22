# `qsharp` - Standalone Modern Q# CLI

A fast, lightweight, standalone command-line compiler and runner for the [Q# Quantum Programming Language](https://github.com/microsoft/qdk), built with Rust.

## Features
- **Zero Dependencies**: Pure native binary with no need for .NET SDK, C# runtime, Python, or VS Code.
- **Modern QDK Engine**: Directly links against Microsoft's official Rust-based `qsc` and `resource_estimator` engines.
- **Instant Execution**: Sub-second compile-and-run cycles.
- **Multi-Shot & Histograms**: Fast simulation over N shots with frequency aggregation (`-s 1000`).
- **Circuit Generation**: Render ASCII text or JSON circuit diagrams (`qsharp circuit <files>`).
- **Resource Estimation**: Calculate physical qubits, runtime, and logical gate breakdown (`qsharp estimate <files>`).
- **Multi-File & Manifests**: Support passing multiple `.qs` files or auto-discovering `qsharp.json` projects.
- **Noise & Backend Control**: Support for Pauli noise models (`--noise px,py,pz`), RNG seeds (`--seed`), and Clifford/Stabilizer simulation (`--sim stabilizer`).

## Installation

### Windows (`.msi` Installer)
Download the latest `qsharp-windows-x64.msi` installer from [Releases](https://github.com/potlurikrishnapriyatham/qsharp/releases).
Run the installer to automatically place `qsharp.exe` in `Program Files` and add it to your system `PATH`.

Once installed, open any command prompt or PowerShell and verify:
```bash
qsharp --version
```

### Linux & macOS
Pre-compiled executable archives are available on the [Releases](https://github.com/potlurikrishnapriyatham/qsharp/releases) page for each platform.

## CLI Usage

### 1. Run Quantum Programs
```bash
# Run single file (or multiple files)
qsharp run program.qs
qsharp run helper.qs caller.qs

# Run with 1,000 shots and histogram summary
qsharp run -s 1000 program.qs

# Deterministic seed & Clifford stabilizer simulation
qsharp run --sim stabilizer --seed 42 program.qs

# Pauli noise channel (px, py, pz)
qsharp run --noise 0.01,0.01,0.01 program.qs
```

### 2. Quantum Circuit Diagrams
```bash
# ASCII circuit diagram in the terminal
qsharp circuit program.qs

# Raw JSON circuit structure
qsharp circuit program.qs --json
```

### 3. Resource Estimation
```bash
# Formatted table of physical and pre-layout logical resources
qsharp estimate program.qs

# Specify custom qubit architecture parameters
qsharp estimate program.qs --params custom_qubit.json

# Complete raw JSON estimation report
qsharp estimate program.qs --json
```

## Documentation
- [CONTEXT.md](CONTEXT.md): Background and problem statement.
- [GOAL.md](GOAL.md): Project vision and objectives.
- [TASKS.md](TASKS.md): Development roadmap.
