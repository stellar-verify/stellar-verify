# Capability Matrix: Stellar Verification Tools

This audit compares existing Stellar ecosystem tools against the proposed capabilities of `stellar-verify` to ensure no duplication of effort, fulfilling Issue #1.

| Capability | `rs-stellar-xdr` | `stellar-archivist` | `stellar-verify` (Proposed MVP) |
| :--- | :--- | :--- | :--- |
| **XDR Decoding/Encoding** | ✅ Full support | ❌ Relies on `rs-stellar-xdr` | ❌ Relies on `rs-stellar-xdr` |
| **History Archive Validation** | ❌ Out of scope | ✅ Full support (checkpoint chains) | ❌ Out of scope |
| **Transaction Hash Recomputation** | ❌ Out of scope | ✅ Internal to archive verification | ✅ Explicit offline CLI/library check |
| **Offline TX Inclusion Proofs** | ❌ Out of scope | ❌ Not exposed as single-tx CLI | ✅ Core MVP workflow |
| **Deterministic JSON Status Output** | ❌ Out of scope | ❌ Log-based output | ✅ Enforced by `VerificationStatus` |
| **Independent Network Finality** | ❌ Out of scope | ❌ Relies on history archive trust | ❌ Out of scope (Explicitly denied) |

**Conclusion:** 
`rs-stellar-xdr` handles serialization, and `stellar-archivist` is designed for bulk history archive verification. There is a concrete gap for an offline, granular CLI tool that verifies a single transaction's inclusion against a user-supplied trusted root (e.g., a ledger header hash) and outputs deterministic, machine-readable results (`Passed`, `Failed`, `Incomplete`). 

**Recommendation:** Proceed with `stellar-verify` MVP focusing strictly on the offline transaction inclusion workflow.