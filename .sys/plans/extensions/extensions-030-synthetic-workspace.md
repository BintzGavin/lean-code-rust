# [extensions-030] — Implement core vscode.workspace types and basic activeTextEditor

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-030`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Implement missing `workspace` and `window` capabilities in the synthetic `vscode` module to provide a minimally viable workspace context for extensions.
- **Vision gap**: X-0002 Document/workspace requires basic file and event interfaces. Many extensions fail to activate or operate if they cannot query basic open document state.
- **Evidence**: `crates/extensions/src/bootstrap.js` defines an empty `workspace` object (`workspace: {}`) and an empty `window` object (`window: {}`). No document models exist.
- **Impact**: Unblocks extensions that unconditionally read workspace folders or the active text editor upon activation.

## 2. File Inventory

```json
{
  "id": "extensions-030",
  "role": "extensions",
  "create": [],
  "modify": [
    "crates/extensions/src/bootstrap.js",
    "crates/extensions/src/host.rs"
  ],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Implement missing `workspace` and `window` capabilities in the Node host's synthetic `vscode` module. They can be empty/undefined by default but their shape should allow extensions to gracefully handle an empty workspace without failing to activate.
- **Compatibility constraints**: The APIs must mimic the behavior of the real VS Code API for these types.
- **Architectural constraints**: Keep logic within `bootstrap.js`. We are mocking these types, not delegating them to Rust for now.
- **Public contract changes**: None at the Rust level.

## 4. Test Plan

- **Failing evidence**: Currently, a test loading an extension that uses `vscode.workspace` properties will fail if not correctly structured.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: Add a test in `crates/extensions/src/host.rs` where a temporary extension activates and successfully accesses these basic properties.
- **Edge cases**: None.

## 5. Constraints

- **Non-goals**: Implementing full workspace logic or syncing from Rust right now.
- **Forbidden changes**: Do not introduce external npm dependencies in `bootstrap.js`.
- **Scope ceiling**: Bounded to implementing mock workspace properties.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
