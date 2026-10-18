# [extensions-027] — Inject synthetic vscode module into Node host

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-027`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Inject a synthetic `vscode` module into the Node environment so extensions can `require('vscode')` without errors.
- **Vision gap**: X-0001 Node host lifecycle and other extension compatibility fixtures require extensions to interact with the `vscode` API. Currently, any extension calling `require('vscode')` will crash because the module does not exist in the isolated Node host.
- **Evidence**: `crates/extensions/src/bootstrap.js` loads extensions via `require(msg.path)`, but does not mock or supply the `vscode` module. Running a script with `require('vscode')` results in a "Cannot find module" error.
- **Impact**: Enables loading and activating real VS Code extensions that declare `vscode` as a dependency, serving as the foundation for incrementally building out the API surface.

## 2. File Inventory

```json
{
  "id": "extensions-027",
  "role": "extensions",
  "create": [
    ".sys/plans/extensions/extensions-027-inject-vscode-module.md"
  ],
  "modify": [
    "crates/extensions/src/bootstrap.js",
    "crates/extensions/src/host.rs",
    ".sys/backlogs/extensions.md"
  ],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**:
  - In `bootstrap.js`, intercept or provide the `vscode` module so that `require('vscode')` returns a synthetic object rather than throwing an error.
  - A standard approach in Node is to modify `require.cache` or use a module interceptor (e.g., `module.constructor._load` wrapper) so that when `vscode` is requested, it returns an empty but extensible object representing the API namespace.
  - The synthetic module should initially provide empty namespaces (e.g., `window`, `workspace`, `commands`) to prevent immediate undefined property access errors during simple activations.
- **Compatibility constraints**: Must allow extensions to resolve `require('vscode')` synchronously during load and activation.
- **Architectural constraints**: Keep logic within `bootstrap.js`. Do not create a separate file on disk for the module unless necessary.
- **Public contract changes**: None at the Rust level.

## 4. Test Plan

- **Failing evidence**: Currently, a test trying to load an extension that `require('vscode')`s fails.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: Add a test in `host.rs` where the temporary extension script contains `const vscode = require('vscode'); vscode.window;`. The test should successfully activate the extension without throwing a module not found error.
- **Edge cases**: Extensions might check for the existence of specific `vscode` properties immediately upon requiring it.

## 5. Constraints

- **Non-goals**: Do not implement full functionality for `vscode` properties (like `vscode.window.showInformationMessage`). Just ensure the module loads and the basic namespace structure exists.
- **Forbidden changes**: Do not introduce heavy npm dependencies to mock `require`.
- **Scope ceiling**: Bounded to successfully resolving `require('vscode')` in the Node host.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
