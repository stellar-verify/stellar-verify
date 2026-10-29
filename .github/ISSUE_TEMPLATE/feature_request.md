---
name: Feature Request
about: Suggest an idea for this project
title: "[Scope] [Feature] "
labels: enhancement
assignees: ''
---
📁 Implementation Directory
`src/`

🎯 Goal
[State the core objective]

📝 Description
[Detailed explanation of the feature]

📋 Requirements
- [Requirement 1]
- [Requirement 2]

🔧 Technical Scope
- Rust
- Core Logic / CLI

🌿 Branch
`git checkout -b feat/your-feature-name`

✅ Acceptance Criteria
- Strict Modular Architecture (Non-Negotiable): Implement using the established domain separation.
- Data Source: Must rely strictly on deterministic XDR fixtures and Ledger hashes. No inline mock objects.
- Production Ready: Code must look production-ready (robust error handling via `thiserror`, strong typings).
- Proof of Work: Upload a screenshot of work done (i.e., CLI execution success, or all `cargo test` passing).
- PR Content: Must include Closes #[issue_id] with a work done summary.

📌 PR Requirements
- ⚠️ Assignment required before starting — comment on this issue to request assignment.
- PR must comply fully with the CONTRIBUTING.md.
