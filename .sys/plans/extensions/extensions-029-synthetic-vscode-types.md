# [extensions-029] — Implement core vscode types (Disposable, EventEmitter, Uri)

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-029`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Implement basic `vscode.Disposable`, `vscode.EventEmitter` (and `Event`), and `vscode.Uri` types in the Node host's synthetic `vscode` module.
- **Vision gap**: X-0002 Document/workspace requires basic interfaces like `Uri` and `event` for document interactions. These foundational types are currently missing, causing extensions that rely on them to fail during activation or operation.
- **Evidence**: `crates/extensions/src/bootstrap.js` defines an empty `workspace` and lacks class definitions for `Disposable`, `EventEmitter`, and `Uri`.
- **Impact**: Provides foundational API types required by almost all VS Code extensions to manage resources, emit events, and reference files, unblocking further integration.

## 2. File Inventory

```json
{
  "id": "extensions-029",
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
  - In `crates/extensions/src/bootstrap.js`, define `class Disposable` that takes a `callOnDispose` function and calls it exactly once when `dispose()` is called. Also implement `Disposable.from(...disposables)`.
  - Define `class EventEmitter` with a `fire(data)` method and an `event` property. The `event` property should be a function `(listener, thisArgs, disposables)` that adds the listener and returns a `Disposable`.
  - Define `class Uri` with basic properties (`scheme`, `authority`, `path`, `query`, `fragment`, `fsPath`) and static methods like `Uri.file(path)` and `Uri.parse(value)`.
  - Add these classes to the `syntheticVscode` object so they are available to extensions as `vscode.Disposable`, `vscode.EventEmitter`, and `vscode.Uri`.
- **Compatibility constraints**: The APIs must mimic the behavior of the real VS Code API for these types. `EventEmitter` must support multiple listeners and unregistration via the returned `Disposable`. `Uri.file` must handle basic local path conversion to `fsPath`.
- **Architectural constraints**: Keep logic within `bootstrap.js`. We are mocking these types, not delegating them to Rust for now, as they are mostly local state/logic in the extension host.
- **Public contract changes**: None at the Rust level.

## 4. Test Plan

- **Failing evidence**: Currently, a test loading an extension that uses `new vscode.EventEmitter()` will fail with "vscode.EventEmitter is not a constructor".
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: Add a test in `crates/extensions/src/host.rs` where a temporary extension activates and successfully instantiates a `Uri`, creates an `EventEmitter`, registers a listener, fires an event, and then disposes it. The extension should return a success message via IPC or `invoke_command` confirming the types behave correctly.
- **Edge cases**: Calling `dispose` multiple times on a `Disposable` (should only invoke callback once); event listeners modifying the listener list during a `fire` call.

## 5. Constraints

- **Non-goals**: Do not implement full RFC 3986 URI parsing with all edge cases. Focus on `Uri.file` and basic `Uri.parse` used in typical extensions.
- **Forbidden changes**: Do not introduce external npm dependencies in `bootstrap.js`.
- **Scope ceiling**: Bounded to implementing three standalone classes purely in JavaScript.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
