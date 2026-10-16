# QUALITY Execution Backlog

This file is the authoritative execution ledger for `quality`. The matching planner and executor may update it across runs. Other roles must not write it.

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
    "id": "quality-001",
    "plan": ".sys/plans/quality/quality-001-x0001-node-host-lifecycle.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Completed implementation of X-0001 Node host lifecycle fixture. Verified successfully."
  },
  {
    "id": "quality-002",
    "plan": ".sys/plans/quality/quality-002-x0002-document-workspace.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Completed implementation of X-0002 Document/workspace fixture. Verified successfully."
  },
  {
    "id": "quality-003",
    "plan": ".sys/plans/quality/quality-003-x0003-commands-settings-keys.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Completed implementation of X-0003 Commands/settings/keys fixture. Verified successfully."
  },
  {
    "id": "quality-004",
    "plan": ".sys/plans/quality/quality-004-x0004-theme-grammar-snippet.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Completed implementation of X-0004 Theme/grammar/snippet compatibility fixture. Verified package.json contribution structures and referenced JSON files syntax."
  },
  {
    "id": "quality-005",
    "plan": ".sys/plans/quality/quality-005-x0005-tree-view.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Completed implementation of X-0005 Tree view fixture. Verified successfully."
  },
  {
    "id": "quality-006",
    "plan": ".sys/plans/quality/quality-006-x0006-state-processes.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Completed implementation of X-0006 State/processes compatibility fixture. Verified successfully."
  },
  {
    "id": "quality-007",
    "plan": ".sys/plans/quality/quality-007-x0007-webview.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Completed implementation of X-0007 Webview fixture. Verified S-0053 and S-0054 behavior flows successfully."
  },
  {
    "id": "quality-008",
    "plan": ".sys/plans/quality/quality-008-x0008-language-server.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Completed implementation of X-0008 Language server fixture. Verified S-0055 and S-0056 flows successfully."
  },
  {
    "id": "quality-009",
    "plan": ".sys/plans/quality/quality-009-x0009-debug-adapter.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Completed implementation of X-0009 Debug adapter compatibility fixture. Verified S-0057 and S-0058 scenarios successfully."
  },
  {
    "id": "quality-010",
    "plan": ".sys/plans/quality/quality-010-x0010-vsix-lifecycle.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Completed implementation of X-0010 VSIX lifecycle compatibility fixture. Verified S-0059 and S-0060 behaviors successfully."
  },
  {
    "id": "quality-011",
    "plan": ".sys/plans/quality/quality-011-x0011-trust-host-fault.md",
    "state": "ready",
    "claim": "none",
    "attempt": 0,
    "result": null
  }
]
```
