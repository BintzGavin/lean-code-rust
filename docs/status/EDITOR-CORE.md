# EDITOR-CORE Status

**Responsibility**: Document buffers, local edits and safe persistence.
**Owned paths**:
- `crates/editor-core/`
- `.sys/plans/editor-core/`
- `.sys/backlogs/editor-core.md`
- `.sys/memory/editor-core.md`
- `docs/status/EDITOR-CORE.md`

## Current state

EDITOR-CORE-001 completed: independent std-only UTF-8 document core and synthetic roundtrip example. The broader desktop and extension requirements remain unimplemented.

## Evidence entry format

```markdown
### [WORK-ORDER-ID] — [completed | blocked | needs_input | cancelled]
- **Result**: [Concise outcome]
- **Changed files**: [Exact paths]
- **Checks**: [Commands and results]
- **Dependency**: [Durable blocked condition, or “None”]
- **Question**: [Exact human question for `needs_input`, or “None”]
- **Resolution evidence**: [Recorded answer or dependency evidence, or “None”]
```

### EDITOR-CORE-001 — completed
- **Result**: Eight integration tests and the executable synthetic edit/cancel/save/close/reopen sequence passed. Dirty state and failed-save buffer retention are covered. Claim commit `202857d` preceded product edits; Claim is now `none`.
- **Changed files**: `crates/editor-core/Cargo.toml`, `crates/editor-core/Cargo.lock`, `crates/editor-core/src/lib.rs`, `crates/editor-core/tests/document.rs`, `crates/editor-core/examples/edit_roundtrip.rs`, `crates/editor-core/README.md`, `.sys/backlogs/editor-core.md`, `.sys/plans/editor-core/editor-core-001-document-roundtrip.md`, `docs/status/EDITOR-CORE.md`.
- **Checks**: Locked/offline single-job Cargo tests first exited 101 (E0432, library absent), then passed 8 integration tests. Cargo fmt check and single-job locked/offline example passed. Role/global scaffold checks, 21 tooling unittests and spec package validation passed. Spec product scenarios executed remain 0. Exact commands and outputs are summarized in the work-order Result.
- **Dependency**: None for this slice. Desktop launch, ordinary extension compatibility and measured product targets remain separate work; no parity claim.
- **Question**: None.
- **Resolution evidence**: Parent setup commit `cda6144` allowed the tiny std-only check under the heavy-build 40% threshold. Latest prebuild free memory was 33%; sysctl pressure level 2 (warning, not critical). Cargo used one job without downloads; no resource errors. Non-fatal xcrun/cc temp-directory warnings fell back to `/tmp`.
- **Limits**: UTF-8 and whole-document edits only; close returns a decision; no atomic save, crash durability, conflict detection or metadata guarantee. Failed save retains memory, but disk may be partially written. No original editor source was inspected.
