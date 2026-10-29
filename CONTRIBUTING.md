# Contributing to Stellar Verify

Thank you for your interest in contributing. This project prioritizes correct cryptographic boundaries, deterministic results, and explicit trust assumptions. 

## Local Setup

1. Ensure you have a stable Rust toolchain installed.
2. Clone the repository:
   ```bash
   git clone [https://github.com/stellar-verify/stellar-verify.git](https://github.com/stellar-verify/stellar-verify.git)
   cd stellar-verify

3. Build the project:
    Bash

    cargo build

Development Workflow

Before opening a pull request, ensure your changes pass the project's strict quality gates:

    Format: cargo fmt --all

    Lint: cargo clippy --workspace --all-targets -- -D warnings

    Test: cargo test --workspace

Pull Request Guidelines

    Discuss first: If you are adding a new feature or verification workflow, open an issue to discuss the protocol-level evidence requirements before writing code.

    Scope: Keep pull requests narrowly scoped to a single issue.

    Testing: Any new verification check must include test fixtures representing valid, invalid, malformed, and incomplete evidence.

    No panic: The core library must not panic on user-controlled input. Map all errors to the typed VerifyError enum.