# Contributing to `qsharp`

Thank you for your interest in contributing to `qsharp`! We welcome contributions of all kinds—bug fixes, features, documentation improvements, tests, and ideas.

---

## Getting Started

### Prerequisites
- [Rust toolchain](https://rustup.rs/) (Rust 1.75+ or newer recommended).
- Git.

### Building Locally
```bash
git clone https://github.com/your-username/qsharp.git
cd qsharp
cargo build
```

### Running Tests and Verifying
```bash
cargo run -- test.qs
```

---

## How to Contribute

1. **Check the Roadmap:** Read [TASKS.md](TASKS.md) to see planned features and open issues.
2. **Fork and Branch:** Create a feature branch off `main`:
   ```bash
   git checkout -b feature/my-new-feature
   ```
3. **Commit Convention:** Write clear, concise commit messages.
4. **Submit a Pull Request (PR):** Describe your changes and reference any related issues.

---

## Code Style & Guidelines
- Run `cargo fmt` before submitting.
- Run `cargo clippy` and ensure no new warnings are introduced.
- Strive for zero external runtime dependencies.

---

## License
By contributing to `qsharp`, you agree that your contributions will be licensed under the MIT License.
