# [extensions-028] — Implement vscode.commands.registerCommand in Node host

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-028`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Implement `vscode.commands.registerCommand` and `vscode.commands.executeCommand` in the synthetic `vscode` module, along with Rust-to-Node IPC for command invocation.
- **Vision gap**: X-0003 Commands/settings/keys requires extensions to register commands and the host to execute them. Currently, `vscode.commands` is an empty object, causing extensions that register commands to fail, and there is no IPC mechanism to trigger an extension command from the Rust host.
- **Evidence**: `crates/extensions/src/bootstrap.js` defines `commands: {}`. `crates/extensions/src/host.rs` only handles `activate` and `deactivate` messages.
- **Impact**: Enables extensions to register commands and the editor core/desktop to eventually invoke those commands, moving closer to full extension activation and interaction.

## 2. File Inventory

```json
{
  "id": "extensions-028",
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
  - In `bootstrap.js`, populate `syntheticVscode.commands` with `registerCommand(command, callback, thisArg)` and `executeCommand(command, ...rest)`.
  - Maintain a registry of commands in the Node host. `registerCommand` should add to this registry and return a `Disposable` that removes it.
  - In `bootstrap.js` IPC handler, add support for a new message `type: 'invoke_command'` with `command` and optional `args`. Find the command in the registry, invoke it (handling Promises), and return `type: 'command_success'` (with optional `result`) or `type: 'command_error'`.
  - In `host.rs`, add a method to `ExtensionHost` to send an `invoke_command` IPC message and wait for the response, similar to activation/deactivation.
- **Compatibility constraints**: `registerCommand` callbacks can be sync or async. The IPC response must wait for async callbacks to resolve. `registerCommand` returns a `Disposable`.
- **Architectural constraints**: Keep logic within `bootstrap.js` and `host.rs`.
- **Public contract changes**: None at the Rust level, beyond adding a method to `ExtensionHost`.

## 4. Test Plan

- **Failing evidence**: Currently, a test trying to load an extension that calls `vscode.commands.registerCommand` throws "registerCommand is not a function".
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: Add a test in `host.rs` where a temporary extension registers a command returning a specific value. The test activates the extension, uses `ExtensionHost` to send an `invoke_command` message, and asserts that the `command_success` message contains the correct returned value.
- **Edge cases**: Command throws an error (should return `command_error`); command returns a Promise; command is not found.

## 5. Constraints

- **Non-goals**: Do not implement complex command argument validation or `executeCommand` across multiple extension hosts (we only have one currently).
- **Forbidden changes**: Do not introduce external dependencies.
- **Scope ceiling**: Bounded to successfully registering and invoking a command within the existing Node host.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
