## Description
<!-- Describe your changes in detail. Link to the specific issue this resolves. -->
Fixes #

## Verification Boundaries
<!-- Did you change how evidence is parsed, or how cryptographic commitments are verified? -->
- [ ] This PR explicitly distinguishes between data parsing and cryptographic proof.

## Checklist
- [ ] I have read the [CONTRIBUTING.md](../CONTRIBUTING.md) document.
- [ ] `cargo fmt --all` passes.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes without warnings.
- [ ] `cargo test --workspace` passes, including new tests for this change.
- [ ] My code safely handles malformed input without panicking.
