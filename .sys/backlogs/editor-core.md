# EDITOR-CORE Execution Backlog

This file is the authoritative execution ledger for `editor-core`. The matching planner and executor may update it across runs. Other roles must not write it.

## Lifecycle contract

- The planner creates one work order and appends its backlog entry in the same repository change. A `ready` work order is immutable; a `needs_input` draft may be revised only to apply its recorded resolution before becoming `ready`.
- The executor claims work by changing `ready` → `in_progress`, incrementing the attempt, and replacing `none` with a stable claim identifier.
- The executor must persist the claim before modifying product files. If the claim cannot be established from current durable repository state, stop without implementation.
- A successful executor changes `in_progress` → `completed` and resets Claim to `none` only after verification passes.
- A missing dependency changes `in_progress` → `blocked`, resets the Claim to `none`, and records evidence without closing the work.
- A permission-sensitive question changes `in_progress` → `needs_input`, resets the Claim to `none`, preserves the exact question, and pauses for manual intervention.
- `blocked` and `needs_input` are recoverable. After durable resolution evidence is recorded, an authorized role-local update may return the entry to `ready` for another claimed attempt.
- `completed` and `cancelled` are terminal. Never return a terminal entry to `ready` under the same work-order ID.
- If a resolution requires different scope or ownership, explicitly cancel the old entry only after enqueueing a replacement work order that references it.
- Preserve terminal entries as history.
- This backlog owns lifecycle state. Work orders own scope and acceptance criteria; status files summarize evidence.

## Recovery contract

- Ask first when a human decision could authorize safe continuation.
- If interactive input is available, ask the exact question and wait.
- If interactive input is unavailable, persist the exact question in the entry's Result, set the state to `needs_input`, reset the Claim to `none`, and pause.
- Work in `needs_input` must not be marked `completed` or `cancelled` merely because no answer is immediately available.
- Work in `blocked` must name the missing dependency and the evidence required to return it to `ready`.
- A planner may perform a recovery review when new durable resolution evidence exists. It must resolve the existing entry instead of stacking unrelated work.
- Never infer that an unresolved claim is abandoned from elapsed time alone. Recovery of in_progress requires durable platform/session evidence that the exact invocation stopped and inspection of its partial changes. The planner records this evidence in Result, preserves all partial work and returns unchanged scope to ready with Claim none. If scope must change, atomically enqueue one replacement and explicitly cancel the old entry; only the planner may do this. If termination is uncertain, leave the claim intact and request human input.

## Entries

The following JSON block is the sole entry data; do not duplicate lifecycle state in prose. Each entry has id, plan, state, claim, attempt and result. An empty array is valid and requires no changes. Preserve entries as history.

```json
[
  {
    "id": "EDITOR-CORE-001",
    "plan": ".sys/plans/editor-core/editor-core-001-document-roundtrip.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Eight integration tests passed after recorded E0432 red check; synthetic roundtrip example, fmt, scaffold, 21 tooling tests and spec validation passed. See work-order Result."
  },
  {
    "id": "EDITOR-CORE-002",
    "plan": ".sys/plans/editor-core/editor-core-002-atomic-save.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Implemented atomic save via temp file and atomic rename. Verified target is preserved on failure. All tests and checks passed."
  },
  {
    "id": "EDITOR-CORE-003",
    "plan": ".sys/plans/editor-core/editor-core-003-crash-safe-save.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Implemented crash-safe save via `File::create`, `write_all`, `sync_data`, and explicit drop before rename. Updated README docs. All tests and checks passed."
  },
  {
    "id": "EDITOR-CORE-004",
    "plan": ".sys/plans/editor-core/editor-core-004-external-change-detection.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Added modified_time to Document and implemented has_external_changes method. Added test and updated README. All checks passed."
  },
  {
    "id": "EDITOR-CORE-005",
    "plan": ".sys/plans/editor-core/editor-core-005-document-reload.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Added reload method to Document, updated tests and README. All checks passed."
  },
  {
    "id": "EDITOR-CORE-006",
    "plan": ".sys/plans/editor-core/editor-core-006-undo-redo.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Implemented basic linear undo/redo history in Document. replace_text pushes to undo stack, undo() and redo() navigate history. All tests passed."
  },
  {
    "id": "EDITOR-CORE-007",
    "plan": ".sys/plans/editor-core/editor-core-007-save-as.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Implemented save_as method on Document to save to a new path and update internal state. Modified save method to delegate to save_as. Added success and failure tests. Updated README. All tests and checks passed."
  },
  {
    "id": "EDITOR-CORE-008",
    "plan": ".sys/plans/editor-core/editor-core-008-incremental-edit.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Implemented edit_range method to Document. Verified substring replacement, out of bounds panics, and integration with undo/redo stack. Updated README. All tests passed."
  },
  {
    "id": "EDITOR-CORE-009",
    "plan": ".sys/plans/editor-core/editor-core-009-prevent-external-overwrite.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Modified `save_as` to check for external changes and return `io::ErrorKind::AlreadyExists` if concurrent changes are detected. Added `force_save_for_testing` to allow internal save tests to bypass external check. Updated README and tests."
  }
]
```
