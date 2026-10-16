# desktop-008 — Dirty title indicator

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-008`

## 1. Context & Goal

- **Objective**: Update the window title to indicate whether the document has unsaved changes.
- **Vision gap**: A usable local desktop editor requires visible indication that a document has unsaved changes (Vision promise 1).
- **Evidence**: `crates/desktop/src/main.rs` statically sets the window title to "Lean Editor" and never updates it based on document state.
- **Impact**: Provides visual feedback to the user about the document's unsaved status.

## 2. File Inventory

```json
{
  "id": "desktop-008",
  "role": "desktop",
  "create": [],
  "modify": [
    "crates/desktop/src/main.rs"
  ],
  "read_only": [
    "crates/editor-core/src/lib.rs"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: After processing any `WindowEvent` (or specifically when the document's dirty state might change, like on keystrokes, save, undo, redo, or document load), update the window's title. If a document is loaded and is dirty, the title should be "Lean Editor *" (or include the file name and an asterisk). If clean or no document is loaded, it should be "Lean Editor".
- **Compatibility constraints**: None.
- **Architectural constraints**: Handle this in the `ApplicationHandler` implementation, likely via a helper method called at the end of `window_event` or when relevant state changes. Use `window.set_title()`.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Launch the app, modify the document via keyboard shortcuts, and observe that the window title does not change.
- **Verification commands**: `cargo check --manifest-path crates/desktop/Cargo.toml` and `cargo test --manifest-path crates/desktop/Cargo.toml`
- **Success criteria**: The window title dynamically adds an asterisk when the document is dirty and removes it when saved, undone to a clean state, or reloaded.
- **Edge cases**: Document loaded from arguments vs no document.

## 5. Constraints

- **Non-goals**: Rendering the document text in the window client area.
- **Forbidden changes**: Do not modify `crates/editor-core/src/lib.rs`.
- **Scope ceiling**: Fits in a single execution run as it only modifies existing event match branches or adds a post-event check.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result

Added dynamic window title indicating dirty document state.
