# Contributing to stellar-verify

First off, thank you for considering contributing to `stellar-verify`! 

## 🛠 Local Development Setup
1. Fork and clone the repository.
2. Ensure you have the [Rust toolchain](https://rustup.rs/) installed (1.75.0+).
3. Run `cargo build` to compile the project.

## 📌 PR Expectations and Review Cadence
- **Review Cadence:** PRs are reviewed within 48 hours of submission.
- **CI Requirements:** GitHub Actions CI checks (`fmt`, `clippy`, `test`) **must pass** before a maintainer will begin a code review.
- **Commit Standards:** We use Conventional Commits (e.g., `feat:`, `fix:`, `docs:`).

## ✅ Pre-Pull Request Checklist
Before opening a PR, you **must** run the following locally:
\`\`\`bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test
\`\`\`
If your PR introduces new logic, you must include deterministic XDR test fixtures demonstrating the success and failure paths.
