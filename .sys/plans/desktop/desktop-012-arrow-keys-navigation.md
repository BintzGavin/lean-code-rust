# DESKTOP-012 — Implement Arrow Keys Navigation

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-012`

## 1. Context & Goal

- **Objective**: Implement basic cursor navigation using arrow keys (Left, Right, Up, Down).
- **Vision gap**: A usable local desktop editor requires fluent keyboard use, including navigation.
- **Evidence**: Currently, there is no way to move a cursor through the document in `crates/desktop/src/main.rs` and `lean_editor_core::Document` doesn't expose cursor state.
- **Impact**: Unlocks basic text editing capabilities where the user can move the cursor and edit text at different positions.

## 2. File Inventory

```json
{
  "id": "desktop-012",
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

- **Required behavior**: The application should track a cursor position. Pressing Left/Right arrow keys should move the cursor backward/forward through the text (bounded by document start/end). Pressing Up/Down should move the cursor up/down lines. Text insertion and deletion should occur at the cursor position.
- **Compatibility constraints**: Keyboard event handling must integrate with the existing `winit` implementation.
- **Architectural constraints**: Since `lean_editor_core::Document` does not currently maintain cursor state, the desktop app must track the cursor position. (Or, if the planner sees fit, they can note this constraint).
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Launching the app and pressing arrow keys does nothing. Text is always appended to the end of the document.
- **Verification commands**: `cargo build --manifest-path crates/desktop/Cargo.toml`, `cargo test --manifest-path crates/desktop/Cargo.toml`, manual verification by running the app.
- **Success criteria**: Arrow keys correctly move the logical cursor position.
- **Edge cases**: Moving left at the beginning of the document, moving right at the end, moving up on the first line, moving down on the last line.

## 5. Constraints

- **Non-goals**: Rendering the cursor visually (this is a separate task). Selection and copy/paste.
- **Forbidden changes**: Modifying `lean_editor_core` (this role only owns `desktop`).
- **Scope ceiling**: Basic logical cursor tracking and moving the insertion point is sufficient for one execution run.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: N/A

## 7. Result

Implemented basic cursor navigation using arrow keys (Left, Right, Up, Down) and integrated insertion/deletion at cursor.
