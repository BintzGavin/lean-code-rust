# desktop-020 — Handle Tab key to insert 4 spaces

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-020`

## 1. Context & Goal

- **Objective**: Handle the `Tab` key to insert 4 spaces in the document.
- **Vision gap**: A usable local desktop editor, including keyboard and accessibility paths. Currently the `Tab` key is ignored because it does not match normal text input logic.
- **Evidence**: `crates/desktop/src/main.rs` does not have a `WindowEvent::KeyboardInput` arm for `NamedKey::Tab`. Typing `Tab` triggers the `text` arm, but since `'\t'.is_control()` is `true`, it is filtered out and nothing is inserted.
- **Impact**: Allows users to indent text and code properly.

## 2. File Inventory

```json
{
  "id": "desktop-020",
  "role": "desktop",
  "create": [],
  "modify": ["crates/desktop/src/main.rs"],
  "read_only": ["crates/desktop/Cargo.toml", "crates/desktop/src/render.rs"],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: When `WindowEvent::KeyboardInput` for `Key::Named(NamedKey::Tab)` is received and pressed, 4 spaces (`"    "`) should be inserted at the cursor, replacing any selected text.
- **Compatibility constraints**: Must correctly advance the cursor and clear the selection anchor.
- **Architectural constraints**: Same logic as other input keys (like `Enter`, `Backspace`, or regular text input). Use `doc.edit_range` and update `self.cursor_pos`. Must trigger a redraw and update title.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Open application, press `Tab`, no spaces are inserted.
- **Verification commands**: `cargo test --manifest-path crates/desktop/Cargo.toml`, `cargo check --manifest-path crates/desktop/Cargo.toml`.
- **Success criteria**: Pressing `Tab` inserts 4 spaces and updates the cursor correctly.
- **Edge cases**: Pressing `Tab` with an active selection should replace the selection with 4 spaces.

## 5. Constraints

- **Non-goals**: No configurable tab sizes for this specific work order, 4 spaces is sufficient. No shift-tab (unindent) required.
- **Forbidden changes**: Do not modify rendering or core editor structures.
- **Scope ceiling**: It is a simple addition of one match arm in the event loop.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result

Completed successfully. Tab key now inserts 4 spaces.
