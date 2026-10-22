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

## Commit Message Convention

This project adheres to the [Conventional Commits](https://www.conventionalcommits.org/) specification (`<type>(<scope>): <subject>`):

### Types
- `feat`: A new feature or capability (e.g., `feat(circuit): add ASCII quantum circuit diagrams`)
- `fix`: A bug fix (e.g., `fix(parser): resolve multi-file implicit namespaces`)
- `docs`: Documentation updates (e.g., `docs(readme): add MSI installation instructions`)
- `style`: Changes that do not affect code logic (formatting, whitespace)
- `refactor`: Code changes that neither fix a bug nor add a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `ci`: Changes to CI/CD workflows and scripts (e.g., `.github/workflows/`)
- `chore`: Maintenance tasks, dependency bumps, or tool configurations

### Formatting Rules
- Use the imperative present tense: "add" not "added", "fix" not "fixed".
- Keep the first line under 72 characters.
- Reference issue or PR numbers in the body or footer (e.g. `Fixes #12`).

---

## Pull Request Guidelines

1. **Check the Roadmap:** Read [TASKS.md](TASKS.md) to see planned features and open issues.
2. **Create a Branch:**
   ```bash
   git checkout -b feat/my-new-feature
   # or
   git checkout -b fix/issue-description
   ```
3. **Verify Locally:**
   - Format: `cargo fmt --check`
   - Lint: `cargo clippy --all-targets`
   - Test: `cargo test`
4. **Use PR Template:** When opening a PR, fill out the checklist and description in the pull request template.

---

## Code Style & Standards
- Keep zero runtime external dependencies (.NET, Python, etc.).
- Ensure all native builds pass clean across Windows, Ubuntu, and macOS.

---

## License
By contributing to `qsharp`, you agree that your contributions will be licensed under the MIT License.
