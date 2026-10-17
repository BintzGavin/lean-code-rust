# desktop-014 — Replacing text selection on input or deletion

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-014`

## 1. Context & Goal

- **Objective**: Ensure text input, `Backspace`, and `Enter` operations replace or delete the currently selected text, instead of just acting on a single cursor position.
- **Vision gap**: A usable local desktop editor requires standard text editing operations where selections are replaced by typed input or deleted entirely upon Backspace.
- **Evidence**: `crates/desktop/src/main.rs` contains `selection_anchor` in `App` state (added in `desktop-013`), but text input, `Backspace`, and `Enter` event handlers only look at `self.cursor_pos` and perform single-character/newline edits without checking `self.selection_anchor`.
- **Impact**: Enables functional text replacement and bulk deletion, essential for basic editing efficiency.

## 2. File Inventory

```json
{
  "id": "desktop-014",
  "role": "desktop",
  "create": [],
  "modify": ["crates/desktop/src/main.rs"],
  "read_only": [],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**:
  - If `self.selection_anchor` is `Some(anchor)`, determine the `start` and `end` of the selection (`min(cursor_pos, anchor)` to `max(cursor_pos, anchor)`).
  - For standard text input, `Backspace`, and `Enter`, invoke `doc.edit_range(start, end, text_to_insert)` instead of acting solely on `cursor_pos`.
  - After the edit, clear the selection (`self.selection_anchor = None`) and update `self.cursor_pos` to immediately follow the inserted text (or the start of the deletion).
- **Compatibility constraints**: Must continue to work correctly when there is no active selection (anchor is None).
- **Architectural constraints**: Maintain current `WindowEvent::KeyboardInput` structure and rely on `lean_editor_core::Document::edit_range`.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Currently, typing or pressing backspace while text is selected (Shift+Arrow) ignores the selection and acts at the active cursor position only.
- **Verification commands**: `cargo check --manifest-path crates/desktop/Cargo.toml`, `cargo test --manifest-path crates/desktop/Cargo.toml`
- **Success criteria**: Code compiles, tests pass. Manual testing confirms that when text is selected, pressing a character key replaces it, pressing Enter replaces it with a newline, and pressing Backspace deletes the entire selection.
- **Edge cases**: Selecting text backwards (anchor > cursor), deleting a selection that spans the whole document.

## 5. Constraints

- **Non-goals**: Rendering the selection visually, clipboard integration.
- **Forbidden changes**: Do not modify `lean_editor_core`.
- **Scope ceiling**: Updating input handlers to respect an already-existing state value.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
