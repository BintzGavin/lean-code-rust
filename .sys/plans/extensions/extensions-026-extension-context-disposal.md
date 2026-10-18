# [extensions-026] — Implement ExtensionContext subscriptions and disposal

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-026`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Pass an `ExtensionContext` with a `subscriptions` array to extension `activate` functions and implement a `deactivate` IPC command to trigger disposal.
- **Vision gap**: X-0001 Node host lifecycle specifies "context subscriptions, disposal". Currently, the Node host activates extensions without providing a context and has no mechanism to deactivate them or clean up their subscriptions.
- **Evidence**: `crates/extensions/src/bootstrap.js` calls `Promise.resolve(ext.activate())` with no arguments. `crates/extensions/src/host.rs` has no `deactivate` IPC message.
- **Impact**: Enables extensions to register disposables during activation and ensures they are properly cleaned up upon deactivation, fulfilling the VS Code lifecycle contract.

## 2. File Inventory

```json
{
  "id": "extensions-026",
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

- **Required behavior**:
  - In `bootstrap.js`, maintain a registry of activated extensions.
  - When handling the `activate` command, create a `context` object containing a `subscriptions` array. Pass this `context` to `ext.activate(context)`. Store the context and the extension module.
  - Implement a `deactivate` IPC command in `bootstrap.js` that takes an extension `path`.
  - On `deactivate`, if the extension exports a `deactivate` function, call it. Then iterate through its `context.subscriptions` and call `dispose()` on any object that has a `dispose` function.
  - Send a `deactivate_success` IPC response upon completion (or `deactivate_error` on failure).
  - Update `ExtensionHost` in `crates/extensions/src/host.rs` to expose a method to send the `deactivate` command.
- **Compatibility constraints**: Must match the VS Code behavior where `context.subscriptions` items are disposed when the extension is deactivated.
- **Architectural constraints**: Keep logic within the existing IPC and `bootstrap.js` structure.
- **Public contract changes**: Modify `ExtensionHost` to support sending `deactivate` messages.

## 4. Test Plan

- **Failing evidence**: Currently, there is no way to send a `deactivate` command, and extensions do not receive a context during `activate`.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: A new test in `host.rs` spawns an extension that pushes a disposable object into `context.subscriptions` during `activate`. The host sends a `deactivate` command, and the test verifies that the disposable's `dispose` function was called (e.g., by logging to stdout) and that `deactivate_success` is received.
- **Edge cases**: The extension does not export a `deactivate` function; a disposable in `subscriptions` throws an error during `dispose()`.

## 5. Constraints

- **Non-goals**: Mocking the entire `vscode` module or implementing specific API disposables (like commands). Only the generic `ExtensionContext` and `dispose()` lifecycle.
- **Forbidden changes**: Do not modify how the Node process itself is spawned or killed.
- **Scope ceiling**: Bounded to the `ExtensionContext` lifecycle injected during `activate` and cleaned up during `deactivate`.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
