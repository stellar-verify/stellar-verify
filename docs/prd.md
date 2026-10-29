# Product Requirements Document (PRD): Stellar Verify

**Document status:** Draft for research and maintainer feedback  
**Product status:** Research  
**Working repository name:** `stellar-verify`  
**Product name:** Stellar Verify  
**Primary implementation language (proposed):** Rust  
**Last reviewed:** 2026-10-29  
**Ownership/affiliation:** Independent community project; not an official Stellar Development Foundation product.

> **Important:** This PRD describes a proposed product. It does not claim that a working implementation exists or that the project has been accepted into Drips Wave. The scope must be validated against existing tools before implementation begins.

## 1. Executive summary

Stellar Verify is a proposed developer toolkit for examining Stellar transaction-related evidence and reporting exactly which checks were performed, which passed, which failed, and which could not be completed. The intended initial form is a Rust library and command-line interface (CLI), if research confirms a concrete gap not already covered by existing tools.

The central product principle is **verification claims must not exceed the evidence**. Retrieving a transaction from an RPC provider, Horizon, an indexer, or an archive is not automatically equivalent to independently proving its inclusion in a ledger. Recomputing a hash commitment, checking a ledger-header hash chain, and validating consensus/finality assumptions are also distinct activities. Stellar Verify must state these distinctions plainly.

The project should begin with research and a narrowly scoped technical specification. It must not begin by promising a generic transaction-inclusion proof or by implementing cryptographic algorithms without tracing them to authoritative Stellar protocol and implementation sources.

## 2. Problem statement

Stellar developers can access transaction and ledger information through network services, history archives, XDR-encoded data, and other tooling. However, the information retrieved and the assurances derived from it can be easy to conflate.

Examples of distinct questions include:

- Was a transaction record returned by a service?
- Is the supplied XDR well-formed and decodable?
- Does a recomputed digest match the commitment defined by the relevant Stellar data structure?
- Does a supplied ledger-header chain link consistently under the implemented rules?
- Is there sufficient evidence to establish that a particular transaction is included in the relevant transaction set?
- What trust assumptions are needed to connect a ledger record to a consensus/finality claim?

These questions do not have interchangeable answers. A tool that reports simply “verified” without naming the exact check could lead users to overstate the assurance they have obtained.

Stellar documentation issue #2611 records a documentation gap around explaining transaction-inclusion verification boundaries and distinguishing full-XDR recomputation from a compact per-transaction Merkle proof. This is a useful starting point for research, not proof that a new standalone tool is necessary. See: https://github.com/stellar/stellar-docs/issues/2611

## 3. Product vision

Make Stellar transaction-related verification workflows reproducible, understandable, and honest about their trust boundaries.

## 4. Goals

1. Define one precise verification workflow supported by authoritative protocol references.
2. Reuse existing Stellar XDR and protocol libraries where appropriate.
3. Accept a clearly specified evidence format and network/protocol context.
4. Return structured, deterministic results describing each check.
5. Distinguish success, failure, incomplete evidence, and unsupported input.
6. Provide reproducible fixtures and automated tests for each supported claim.
7. Document what each result does and does not establish.
8. Make the project easy for external contributors to build, test, and review.
9. Prefer upstream contributions to a new repository when existing projects already cover the required behavior.

## 5. Non-goals

The first version will not:

- Claim to provide a compact per-transaction Merkle proof unless a protocol-supported proof structure and correct verification procedure are demonstrated.
- Claim to independently validate the full Stellar Consensus Protocol, quorum intersection, or network finality unless that functionality is explicitly designed, implemented, tested, and reviewed.
- Treat data retrieved from a service as self-authenticating proof merely because the response is well-formed.
- Replace Stellar Core, Horizon, RPC services, history archives, SDKs, or existing verification libraries.
- Reimplement Stellar XDR types if maintained libraries already provide the required types and encoding/decoding.
- Require a Soroban smart contract. The initial proposed workflow is off-chain developer tooling.
- Require a frontend for the MVP.
- Store, request, or process users' private keys or secret signing material.
- Make financial, legal, or audit-compliance guarantees.
- Describe itself as an official Stellar product or imply endorsement.
- Duplicate the existing `stellar-archivist` verifier or other existing functionality without first identifying a concrete gap.

## 6. Target users and user needs

### 6.1 Stellar application developers
Need to understand what can be verified locally when their application receives transaction or ledger evidence from an API or data provider.

### 6.2 Infrastructure and backend engineers
Need repeatable checks and machine-readable output for debugging, monitoring, or data pipeline validation.

### 6.3 Security researchers and auditors
Need explicit evidence inputs, reproducible test cases, and documented assumptions rather than an opaque boolean result.

### 6.4 Documentation and developer-relations contributors
Need accurate examples explaining transaction retrieval, data integrity, ledger commitments, inclusion, and consensus/finality without conflating them.

### 6.5 Open-source contributors
Need small issues, reliable local setup, clear acceptance criteria, and tests that define the expected behavior.

## 7. Jobs to be done

- When I receive Stellar transaction-related evidence, I want to know which properties I can check independently, so that I do not mistake a service response for stronger proof.
- When a check fails, I want a specific diagnostic, so that I can identify whether the cause is malformed input, mismatched evidence, missing data, or unsupported input.
- When I integrate verification into a workflow, I want deterministic JSON output and a stable exit-code contract.
- When I review a result, I want to know the exact evidence and assumptions behind it.
- When a protocol version or input format is unsupported, I want an explicit unsupported result rather than a silent fallback.

## 8. Proposed MVP scope

The precise first operation is **not selected yet**. It must be selected only after the existing-tool audit.

The MVP should support exactly one narrowly described verification workflow. Candidate workflow categories for research—not commitments—include:

- Validating the internal consistency of a supplied ledger-close evidence package.
- Recomputing a documented commitment from a complete, supported XDR structure.
- Checking a supported sequence of ledger headers against documented hash-linking rules.
- Producing a structured explanation of which evidence is present and which claims remain unverified.

The chosen workflow must have an authoritative specification, reproducible fixtures, and a clear distinction between cryptographic checks and external trust assumptions.

### MVP must include
- A written verification model.
- A documented evidence/input format.
- A Rust library API, if implementation is justified.
- A CLI wrapper, if it improves practical usability.
- Text and JSON output, if both are included in the selected scope.
- Deterministic tests for valid, invalid, malformed, incomplete, and unsupported cases.
- A documented error taxonomy and exit-code contract.
- A threat model and limitations section.
- CI for formatting, linting, and tests.

### MVP explicitly excludes
- Generic “verify any Stellar transaction” claims.
- Unproven compact inclusion-proof support.
- Full consensus verification.
- A browser frontend.
- A Soroban contract.
- Hosted verification services or accounts.
- Automatic trust in a remote endpoint as an independent authority.

## 9. Functional requirements

### FR-1: Evidence input
The tool shall accept only documented input formats. It shall validate required fields and reject malformed data with a useful diagnostic.

### FR-2: Network and protocol context
Where relevant, the tool shall require or derive a clearly defined network/protocol context. It shall not silently mix evidence from different networks or incompatible protocol versions.

### FR-3: Explicit verification steps
Every implemented check shall be named and documented. The tool shall not collapse materially different checks into an ambiguous “verified” result.

### FR-4: Result states
The result model shall distinguish at least:
- `passed`: the specified check completed and its stated condition held;
- `failed`: the specified check completed and its condition did not hold;
- `incomplete`: required evidence was missing or insufficient;
- `unsupported`: the input or operation is outside the implemented support boundary;
- `error`: an unexpected operational or parsing error prevented a valid result.

Exact names may change during implementation, but semantic distinctions must remain.

### FR-5: Human-readable output
CLI output shall summarize the operation, result, evidence checked, and key limitations without implying stronger guarantees than were tested.

### FR-6: Machine-readable output
If JSON output is part of the MVP, it shall be deterministic and documented with a versioned schema or compatibility policy.

### FR-7: Exit codes
The CLI shall define stable exit codes for successful checks, failed checks, incomplete/unsupported evidence, and operational errors. The mapping must be documented and tested.

### FR-8: Local operation
The core verification operation should work offline on supplied evidence unless a specific workflow requires external data. Any network access must be explicit and documented.

### FR-9: Resource limits
The tool shall set reasonable limits for input sizes and collection lengths, where relevant, to avoid unbounded memory or CPU consumption from hostile input.

### FR-10: No secret collection
The tool shall not request private keys, seeds, passwords, or secret signing material for the proposed verification workflow.

### FR-11: Version awareness
The tool shall detect or require the relevant version information and fail explicitly when the evidence cannot be interpreted safely.

### FR-12: Reproducibility
The same supported input and tool version shall produce the same logical result and stable machine-readable fields.

## 10. Non-functional requirements

- **Correctness:** Each check is grounded in an authoritative protocol definition or implementation.
- **Transparency:** Verification claims and trust assumptions are documented.
- **Maintainability:** Small modules, explicit types, narrow public API, and documented dependencies.
- **Portability:** Support common developer environments where Rust is supported.
- **Performance:** Avoid unnecessary parsing or copying; benchmark only once representative fixtures exist.
- **Security:** Reject malformed input safely and avoid panics on user-controlled data.
- **Accessibility:** CLI messages are plain text and usable in automated environments.
- **Compatibility:** Breaking changes to CLI output or library API are documented.
- **Privacy:** No telemetry or upload of supplied evidence by default.
- **Supply-chain hygiene:** Pin and review dependencies, use lockfiles for applications, and run automated dependency/security checks where practical.

## 11. Technical architecture (proposed)

### 11.1 Rust library
Owns the supported evidence types, validation steps, typed result model, and error taxonomy. It should not contain CLI presentation concerns.

### 11.2 Rust CLI
Parses command-line arguments, reads files/stdin, calls the library, formats results, and returns documented exit codes.

### 11.3 Stellar protocol libraries
Investigate `stellar/rs-stellar-xdr` and related maintained libraries before implementing data types or encoders. Reference: https://github.com/stellar/rs-stellar-xdr

### 11.4 Test fixtures
Keep small, documented fixtures for each supported case. Where a fixture is derived from public network data, record its source, ledger/protocol context, and any transformation. Do not include secrets or unlicensed material.

### 11.5 Optional future TypeScript interface
Only consider after the core workflow is useful and stable. A frontend must call or consume the same verification implementation or a well-defined interface; it must not silently duplicate security-sensitive logic.

### 11.6 Smart contracts
No Soroban contract is proposed for the MVP. On-chain execution is not required merely to examine evidence locally. A contract would be considered only if a separately demonstrated requirement needs on-chain verification or enforcement.

## 12. Data model and output contract

The implementation should eventually define a versioned result object containing fields equivalent to:
- tool/schema version;
- operation name;
- overall status;
- checks performed;
- evidence identifiers or digests that are safe to display;
- network/protocol context where relevant;
- diagnostics;
- limitations and assumptions;
- optional timing/metadata that does not affect verification semantics.

Do not finalize field names until the chosen workflow is selected. Avoid printing full raw evidence by default if it can be large or sensitive.

## 13. Security and threat model

### Assets
- Integrity of verification results.
- Correct interpretation of protocol evidence.
- User-provided evidence files.
- Availability of local tooling.

### Threats
- Malformed or adversarially large inputs.
- Evidence mixed across networks or protocol versions.
- A digest recomputed over the wrong serialization or wrong object.
- Confusion between transaction data and transaction-result data.
- A service response being mistaken for independently verified evidence.
- A hash-chain check being mistaken for independent consensus/finality verification.
- Dependency compromise or unreviewed cryptographic code.
- Output ambiguity that causes downstream systems to treat incomplete evidence as success.

### Mitigations
- Use maintained Stellar protocol libraries.
- Ground calculations in authoritative sources and cross-check against known-good implementations/fixtures.
- Add fuzzing/property tests where useful and feasible.
- Set input limits and handle errors without panics.
- Separate parsing, verification, and presentation layers.
- Use typed statuses and documented exit codes.
- Include explicit limitations in docs and output.
- Review dependencies and keep CI visible.
- Require peer review for changes affecting verification semantics.

### Out of scope
The tool cannot compensate for every compromised endpoint, malicious operator, or incorrect external trust anchor. The threat model must be refined after the exact workflow is selected.

## 14. Verification model and terminology

The project documentation must distinguish at least these concepts:

- **Retrieval:** a service returned a record or file.
- **Parsing:** input conforms to the expected encoding/schema sufficiently to decode.
- **Integrity check:** a specified recomputation or consistency rule matches.
- **Commitment verification:** a supplied value matches a documented cryptographic commitment to the exact data and serialization used.
- **Inclusion evidence:** evidence establishes that an item belongs to a committed collection under the relevant data structure and verification rules.
- **Ledger-chain consistency:** a sequence of ledger headers links according to the implemented rules.
- **Consensus/finality assurance:** a separate question involving consensus rules and trust assumptions; do not claim it merely because a hash matches.

A result must identify which level(s) were actually checked. Do not use “proof” as a generic synonym for retrieved data or successful parsing.

## 15. Test strategy

### Unit tests
- Input parsing and validation.
- Each individual verification rule.
- Status and diagnostic mapping.
- Output serialization and exit-code mapping.

### Integration tests
- End-to-end CLI on documented fixtures.
- File input and stdin behavior if both are supported.
- JSON output schema and deterministic behavior.

### Negative tests
- Truncated input.
- Invalid base encoding where applicable.
- Wrong object type.
- Mismatched digest/commitment.
- Missing required evidence.
- Unsupported protocol version.
- Oversized collections or input.
- Cross-network or context mismatch where relevant.

### Property/fuzz tests
Consider fuzzing parsers and invariants after the core implementation stabilizes. Fuzzing is supplementary and does not replace protocol-level review.

### Test provenance
Every non-synthetic fixture should document where it came from and what it is intended to demonstrate. Avoid treating a fixture from one implementation as the sole oracle for that same implementation.

## 16. Acceptance criteria

The MVP is ready for an initial release candidate only when:
1. The existing-tool audit is complete and the chosen gap is documented.
2. The supported operation is defined precisely and reviewed by a knowledgeable reviewer.
3. The evidence format and required context are documented.
4. Valid, invalid, incomplete, malformed, and unsupported cases are tested.
5. The output does not overstate what has been verified.
6. CI passes formatting, linting, and tests.
7. Installation and usage instructions have been tested from a clean checkout.
8. Security boundaries and limitations are published.
9. Dependencies and licensing are reviewed.
10. The README accurately reflects the actual implementation status.

## 17. Metrics for success

Early-stage measures:
- One precise workflow documented and reproducible.
- No known duplication of existing maintained functionality.
- A small set of authoritative references linked from the verification model.
- All defined test cases passing in CI.
- External reviewer feedback incorporated.

After release, possible measures:
- Successful clean installations.
- Issues that reveal reproducible bugs or documentation gaps.
- Contributions from people other than the initial author.
- Downstream integrations that use the output contract correctly.
- Time to diagnose a failed verification fixture.

Do not use stars or download counts as the primary measure of technical correctness.

## 18. Roadmap

### Phase 0 — Research (current)
- Read Stellar Docs issue #2611 and its discussion.
- Audit `stellar-archivist`, Stellar Core, XDR tooling, SDKs, and relevant docs.
- Build a capability matrix.
- Select a single supported workflow or recommend an upstream contribution instead.
- Ask for maintainer feedback before committing to a standalone tool.

### Phase 1 — Specification
- Publish verification model and threat model.
- Define evidence format, statuses, diagnostics, and exit codes.
- Create fixture plan and test vectors.
- Obtain technical review.

### Phase 2 — Minimal implementation
- Implement library logic for the agreed workflow.
- Add CLI wrapper only as needed.
- Add positive and negative fixtures.
- Wire CI.

### Phase 3 — Contributor readiness
- Finalize CONTRIBUTING, security reporting, issue templates, and roadmap.
- Create small, independently actionable issues.
- Document release/versioning policy.

### Phase 4 — Optional expansion
- Add another verification operation only when separately specified and tested.
- Consider TypeScript UI only if it materially improves a validated workflow.
- Consider online data acquisition only with explicit trust labeling.
- Consider on-chain functionality only with a demonstrated requirement.

## 19. Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Existing tools already solve the proposed task | Duplicated maintenance | Audit first; contribute upstream if appropriate |
| Scope implies stronger proof than implemented | Users misinterpret results | Explicit statuses, terminology, review |
| Protocol details are misunderstood | Incorrect results | Authoritative references and independent review |
| Protocol changes break compatibility | Stale/unsafe behavior | Version-aware support policy and tests |
| Project begins with too many features | Slow delivery | One workflow, one MVP |
| No maintainer feedback | Wrong contribution target | Open a scoped research issue and ask before major work |
| Documentation drifts from implementation | Misleading claims | CI/docs review and release checklist |

## 20. Open questions

1. Which exact verification operation is not already implemented by existing tools?
2. Which Stellar protocol versions and input formats must be supported?
3. What evidence source is authoritative for the selected check?
4. Is a new standalone repository justified, or would an upstream contribution be better?
5. What output fields are useful to real downstream callers?
6. What external reviewer can validate the protocol reasoning?

## 21. References to investigate

- Stellar Docs issue #2611: https://github.com/stellar/stellar-docs/issues/2611
- Stellar Rust XDR library: https://github.com/stellar/rs-stellar-xdr
- Stellar XDR definitions: https://github.com/stellar/stellar-xdr
- Stellar documentation repository: https://github.com/stellar/stellar-docs
- `stellar-archivist` verification module: https://docs.rs/stellar-archivist/latest/stellar_archivist/xdr_verify/index.html

These links are research starting points, not endorsements or confirmation that a new tool is required.

## 22. Decision gate

After Phase 0, choose one:
- **Proceed:** a concrete, independently useful gap is confirmed.
- **Contribute upstream:** existing software is the right home for the missing behavior.
- **Documentation-first:** the primary gap is explanation rather than software.
- **Stop:** no sufficiently distinct value is established.

A well-evidenced decision not to create redundant software is a successful research outcome.