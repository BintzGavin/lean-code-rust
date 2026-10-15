# desktop-004 — Editor Core Integration

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-004`

## 1. Context & Goal

- **Objective**: Integrate `editor-core` into the desktop application to load a document from command-line arguments.
- **Vision gap**: Vision requires a desktop editor to "open folders, navigate, edit/save", but the current desktop app lacks any document management or connection to the core buffer logic.
- **Evidence**: `crates/desktop/Cargo.toml` does not depend on `editor-core` and `crates/desktop/src/main.rs` contains only window and keyboard lifecycle state.
- **Impact**: Unlocks the ability to render actual document text and process input against a real buffer in future work.

## 2. File Inventory

```json
{
  "id": "desktop-004",
  "role": "desktop",
  "create": [],
  "modify": [
    "crates/desktop/Cargo.toml",
    "crates/desktop/src/main.rs"
  ],
  "read_only": [],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The desktop application parses the first command-line argument as a file path. If provided, it uses `editor-core::Document::open` to load the file and stores the resulting `Document` in the `App` struct state. If no path is provided or opening fails, it should gracefully initialize an empty document or print an error instead of crashing.
- **Compatibility constraints**: Must preserve the existing native window creation and Escape-to-close behavior.
- **Architectural constraints**: Only the `desktop` crate is modified. The `editor-core` dependency should be added as a local path dependency (`path = "../editor-core"`).
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Running `cargo run --manifest-path crates/desktop/Cargo.toml -- README.md` currently ignores the argument and doesn't load a document.
- **Verification commands**:
  - `cargo check --manifest-path crates/desktop/Cargo.toml`
  - `cargo test --manifest-path crates/desktop/Cargo.toml`
- **Success criteria**: The application compiles with the new `editor-core` dependency and successfully manages a document in its state without panicking on valid files.
- **Edge cases**: Missing command-line arguments and invalid file paths should be handled gracefully (e.g., standard error or empty document) rather than crashing the winit event loop.

## 5. Constraints

- **Non-goals**: Rendering the text to the window or processing typing input is explicitly deferred to later work orders.
- **Forbidden changes**: Do not modify `crates/editor-core/`.
- **Scope ceiling**: Minimal wiring of command-line arguments to `Document` and state storage fits easily in one run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: The `editor-core` crate interface is insufficient to manage document state.

## 7. Result
