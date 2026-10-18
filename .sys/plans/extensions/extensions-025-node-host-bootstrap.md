# [extensions-025] — Implement Node Host Bootstrap Shim

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-025`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Implement a Node.js bootstrap shim (`bootstrap.js`) to allow the `ExtensionHost` to programmatically load an extension script and invoke its `activate` function.
- **Vision gap**: X-0001 Node host lifecycle specifies "activation, context subscriptions, disposal" and "activation exception". While `extensions-024` added basic IPC to the `ExtensionHost`, there is no mechanism to dynamically load an extension's main script and call its `activate` function via IPC.
- **Evidence**: `crates/extensions/src/host.rs` spawns a given Node script, but no standard host bootstrap script exists to coordinate loading extensions based on IPC commands.
- **Impact**: Enables the actual activation and lifecycle management of VS Code extensions within the isolated Node host.

## 2. File Inventory

```json
{
  "id": "extensions-025",
  "role": "extensions",
  "create": [
    "crates/extensions/src/bootstrap.js"
  ],
  "modify": [
    "crates/extensions/src/host.rs",
    "crates/extensions/Cargo.toml"
  ],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Create a `bootstrap.js` script that uses `readline` to listen to standard input for IPC messages. When it receives a specific command (e.g., `{"type":"activate", "path":"..."}` or similar), it should `require()` the specified path and call its `activate` function. Support sending a response back via `stdout`. Update `ExtensionHost::spawn` to spawn this `bootstrap.js` script, or provide a way to load it.
- **Compatibility constraints**: The bootstrap script must run in a standard Node.js environment.
- **Architectural constraints**: Maintain strict IPC isolation using `stdin`/`stdout`.
- **Public contract changes**: Modify `ExtensionHost` or its initialization to utilize the `bootstrap.js` script.

## 4. Test Plan

- **Failing evidence**: Currently, there is no way to tell the `ExtensionHost` to dynamically load and activate a secondary script.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: A new integration test in `host.rs` successfully spawns the `ExtensionHost` using the bootstrap script, sends an IPC message to load a synthetic extension script, and receives a successful activation response.
- **Edge cases**: The extension script does not exist; the extension script throws an error during `activate`.

## 5. Constraints

- **Non-goals**: Implementing the full VS Code extension API context (e.g., `vscode` module mock). Only basic loading and `activate` invocation.
- **Forbidden changes**: Do not modify existing manifest parsing or VSIX lifecycle logic.
- **Scope ceiling**: Strictly bounded to creating the Node bootstrap shim and updating `ExtensionHost` to use it for basic activation.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
