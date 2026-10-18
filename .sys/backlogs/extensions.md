# EXTENSIONS Execution Backlog

This file is the authoritative execution ledger for `extensions`. The matching planner and executor may update it across runs. Other roles must not write it.

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
    "id": "extensions-001",
    "plan": ".sys/plans/extensions/extensions-001-bootstrap-crate.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Successfully bootstrapped the extensions crate."
  },
  {
    "id": "extensions-002",
    "plan": ".sys/plans/extensions/extensions-002-parse-extension-manifest.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Parsed VS Code extension manifest package.json."
  },
  {
    "id": "extensions-003",
    "plan": ".sys/plans/extensions/extensions-003-parse-extension-contributions.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Extended ExtensionManifest to parse activationEvents and basic contributes.commands."
  },
  {
    "id": "extensions-004",
    "plan": ".sys/plans/extensions/extensions-004-parse-extension-dependencies.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Extended ExtensionManifest to parse extensionDependencies and extensionPack."
  },
  {
    "id": "extensions-005",
    "plan": ".sys/plans/extensions/extensions-005-parse-contributes-configuration-keybindings.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Extended ExtensionContributes to parse configuration and keybindings."
  },
  {
    "id": "extensions-006",
    "plan": ".sys/plans/extensions/extensions-006-parse-contributes-menus.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Added parsing for contributes.menus"
  },
  {
    "id": "extensions-007",
    "plan": ".sys/plans/extensions/extensions-007-parse-contributes-theme-grammar-snippets.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Parsed themes, languages, grammars, and snippets from VS Code extension manifest package.json."
  },
  {
    "id": "extensions-008",
    "plan": ".sys/plans/extensions/extensions-008-parse-views-and-views-containers.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Parsed contributes.views and contributes.viewsContainers"
  },
  {
    "id": "extensions-009",
    "plan": ".sys/plans/extensions/extensions-009-parse-contributes-debuggers.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Parsed contributes.debuggers and contributes.breakpoints from package.json."
  },
  {
    "id": "extensions-010",
    "plan": ".sys/plans/extensions/extensions-010-parse-contributes-task-definitions.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Parsed contributes.taskDefinitions from package.json."
  },
  {
    "id": "extensions-011",
    "plan": ".sys/plans/extensions/extensions-011-parse-workspace-trust-capabilities.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Parsed workspace trust capabilities and extension kind."
  },
  {
    "id": "extensions-012",
    "plan": ".sys/plans/extensions/extensions-012-parse-api-proposals.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Parsed enabledApiProposals from package.json manifest."
  },
  {
    "id": "extensions-013",
    "plan": ".sys/plans/extensions/extensions-013-parse-contributes-problem-matchers.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Parsed contributes.problemMatchers and contributes.problemPatterns."
  },
  {
    "id": "extensions-014",
    "plan": ".sys/plans/extensions/extensions-014-parse-metadata-and-browser.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Parsed metadata and browser entry point from package.json manifest."
  },
  {
    "id": "extensions-015",
    "plan": ".sys/plans/extensions/extensions-015-parse-contributes-colors-and-defaults.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Parsed contributes.colors and contributes.configurationDefaults from package.json."
  },
  {
    "id": "extensions-016",
    "plan": ".sys/plans/extensions/extensions-016-parse-contributes-submenus.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Parsed contributes.submenus from package.json manifest."
  },
  {
    "id": "extensions-017",
    "plan": ".sys/plans/extensions/extensions-017-parse-missing-manifest-fields.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Successfully parsed the missing manifest metadata fields into ExtensionManifest."
  },
  {
    "id": "extensions-018",
    "plan": ".sys/plans/extensions/extensions-018-parse-vsix-archive.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Extracted and parsed extension/package.json from a VS Code .vsix archive."
  },
  {
    "id": "extensions-019",
    "plan": ".sys/plans/extensions/extensions-019-extract-vsix-archive.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Successfully implemented extract_vsix"
  },
  {
    "id": "extensions-020",
    "plan": ".sys/plans/extensions/extensions-020-install-vsix.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Implemented install_vsix to extract extensions into standard canonical paths."
  },
  {
    "id": "extensions-021",
    "plan": ".sys/plans/extensions/extensions-021-uninstall-vsix.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Implemented uninstall_vsix to remove canonical extension directories."
  },
  {
    "id": "extensions-022",
    "plan": ".sys/plans/extensions/extensions-022-list-installed-extensions.md",
    "state": "completed",
    "claim": "none",
    "attempt": 2,
    "result": "Implemented list_installed_extensions to discover extensions from disk."
  },
  {
    "id": "extensions-023",
    "plan": ".sys/plans/extensions/extensions-023-extension-host-process.md",
    "state": "completed",
    "claim": "none",
    "attempt": 1,
    "result": "Successfully implemented the ExtensionHost process manager for isolated Node hosts."
  },
  {
    "id": "extensions-024",
    "plan": ".sys/plans/extensions/extensions-024-node-host-ipc.md",
    "state": "ready",
    "claim": "none",
    "attempt": 0,
    "result": ""
  }
]
```
