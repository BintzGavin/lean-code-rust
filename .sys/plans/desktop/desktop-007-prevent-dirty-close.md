# desktop-007 — Prevent dirty document close

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-007`

## 1. Context & Goal

- **Objective**: Prevent the application window from closing if the document has unsaved changes.
- **Vision gap**: User data and security boundaries survive errors and interruption (Vision promise 3).
- **Evidence**: `crates/desktop/src/main.rs` exits the event loop unconditionally on `WindowEvent::CloseRequested` and Escape key presses.
- **Impact**: Protects against accidental loss of user edits when closing the editor.

## 2. File Inventory

```json
{
  "id": "desktop-007",
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

- **Required behavior**: When the user requests to close the window (via `WindowEvent::CloseRequested` or `Escape` key), check if a document is loaded. If it is, call `Document::request_close()`. If the outcome is `lean_editor_core::CloseOutcome::Cancelled` (meaning it's dirty), do not exit the event loop. Otherwise, or if no document is loaded, exit the event loop.
- **Compatibility constraints**: None.
- **Architectural constraints**: Handle events within the Winit `ApplicationHandler`'s `window_event` method.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Launch the app with a file, modify the document text (using a test rig if needed or testing dirty logic manually), press Escape or close the window, and observe that it exits unconditionally.
- **Verification commands**: `cargo check --manifest-path crates/desktop/Cargo.toml` and `cargo test --manifest-path crates/desktop/Cargo.toml`
- **Success criteria**: The application stays open when `Escape` is pressed or the window is closed if the document is dirty.
- **Edge cases**: If no document is open, the app should close normally. If the document is clean, the app should close normally.

## 5. Constraints

- **Non-goals**: Adding a "Save changes?" dialog UI. This is purely to prevent the data loss; UI for handling the dirty state can be a separate task.
- **Forbidden changes**: Do not modify `crates/editor-core/src/lib.rs`.
- **Scope ceiling**: Fits in a single execution run as it only modifies existing winit event match branches.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
