# desktop-006 — Undo/Redo Shortcuts

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-006`

## 1. Context & Goal

- **Objective**: Wire up keyboard shortcuts for Undo (Cmd/Ctrl+Z) and Redo (Cmd/Ctrl+Shift+Z or Cmd/Ctrl+Y) to the editor core's undo/redo APIs.
- **Vision gap**: A usable local desktop editor, including keyboard paths.
- **Evidence**: `crates/desktop/src/main.rs` does not handle Undo or Redo shortcuts, though `crates/editor-core/src/lib.rs` provides `undo()` and `redo()`.
- **Impact**: Enables basic document history traversal via standard keyboard shortcuts.

## 2. File Inventory

```json
{
  "id": "desktop-006",
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

- **Required behavior**: When the user presses `Ctrl+Z` (or `Cmd+Z` on Mac), call `Document::undo()`. When the user presses `Ctrl+Shift+Z` or `Cmd+Shift+Z` (or `Ctrl+Y` / `Cmd+Y`), call `Document::redo()`.
- **Compatibility constraints**: Standard keybindings expected for a native application.
- **Architectural constraints**: Handle events within the Winit `ApplicationHandler`'s `window_event` method.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Run `cargo run --manifest-path crates/desktop/Cargo.toml` and observe that Ctrl+Z does not mutate document state or trigger any undo logic.
- **Verification commands**: `cargo check --manifest-path crates/desktop/Cargo.toml` and `cargo test --manifest-path crates/desktop/Cargo.toml`
- **Success criteria**: The application routes `Ctrl+Z` to `undo()` and `Ctrl+Shift+Z`/`Ctrl+Y` to `redo()`.
- **Edge cases**: Undo or redo when the stack is empty should be a safe no-op.

## 5. Constraints

- **Non-goals**: Modifying document text or rendering. Only wiring keyboard shortcuts to existing document APIs.
- **Forbidden changes**: Do not modify `crates/editor-core/src/lib.rs`.
- **Scope ceiling**: Fits in a single execution run as it only adds match branches to the existing winit event loop.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
