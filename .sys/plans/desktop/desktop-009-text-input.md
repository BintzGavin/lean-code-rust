# desktop-009 — Text input handling

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-009`

## 1. Context & Goal

- **Objective**: Handle standard text typing via winit's `text` field in `KeyEvent` to insert characters into the document.
- **Vision gap**: A usable local desktop editor requires keyboard input paths, specifically typing text into the document (Vision promise 1).
- **Evidence**: `crates/desktop/src/main.rs` only handles shortcut keys like Cmd+S and Escape, but ignores ordinary text characters. The winit `KeyEvent` contains a `text` field with typed characters.
- **Impact**: Enables basic text entry for the user.

## 2. File Inventory

```json
{
  "id": "desktop-009",
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

- **Required behavior**: In the `window_event` method's `KeyboardInput` match arm, when `state` is `ElementState::Pressed`, and the event is not matched by existing shortcuts (or shortcuts take precedence), extract the `text` field. If `text` is `Some(smol_str)`, and no modifier keys like Control or Super are active (Shift is fine), append the text to the document or insert it. Since `Document` only has `replace_text` and `edit_range`, we will append it to the end of the text for now by reading the current text, appending, and calling `replace_text`.
- **Compatibility constraints**: Need to check modifiers to ensure we don't accidentally insert text when pressing `Cmd+S` etc.
- **Architectural constraints**: Text handling should occur in `main.rs`. We can implement a fallback branch for `KeyEvent` to catch ordinary text.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Run the app, type keys, and see no text added to the document.
- **Verification commands**: `cargo check --manifest-path crates/desktop/Cargo.toml` and `cargo test --manifest-path crates/desktop/Cargo.toml`
- **Success criteria**: Pressing standard text keys (e.g. 'a', 'b', 'c', '1', '2', ' ') will modify the document buffer.
- **Edge cases**: Avoid inserting text when `Cmd` or `Ctrl` is pressed.

## 5. Constraints

- **Non-goals**: Implementing full cursor management and text insertion at arbitrary locations. Appending to the end of the document is sufficient for this scope.
- **Forbidden changes**: Do not modify `crates/editor-core/src/lib.rs`.
- **Scope ceiling**: Fits in a single execution run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
Successfully added text input handling to append typed characters to the document.
