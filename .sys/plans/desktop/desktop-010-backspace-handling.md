# desktop-010 — Backspace deletion handling

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-010`

## 1. Context & Goal

- **Objective**: Handle the Backspace key to remove the last character from the document.
- **Vision gap**: A usable local desktop editor requires editing capabilities, which includes removing text via Backspace (Vision promise 1).
- **Evidence**: `crates/desktop/src/main.rs` processes text input via `WindowEvent::KeyboardInput` and shortcut keys like Escape, but lacks any logic for `Key::Named(NamedKey::Backspace)` to delete characters. Line 150 shows `doc.replace_text(format!("{}{}", current_text, text_str));` is used to append text.
- **Impact**: Enables users to correct mistakes by deleting the most recently typed character.

## 2. File Inventory

```json
{
  "id": "desktop-010",
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

- **Required behavior**: In the `window_event` method's `KeyboardInput` match arm, add a new case to handle the Backspace key (`logical_key: Key::Named(NamedKey::Backspace)`). When pressed, if the document has text, remove the last character and update the document using `doc.replace_text()`. We use `replace_text()` because `main.rs` already uses it for appending text (line 150). We pop a `char` rather than a byte to correctly handle UTF-8.
- **Compatibility constraints**: Must not crash if the document is empty. Ensure multi-byte UTF-8 characters are correctly removed by popping a `char`.
- **Architectural constraints**: Keep logic within `main.rs`'s event loop.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Pressing Backspace currently appends nothing or a control character, and does not remove the last character.
- **Verification commands**: `cargo check --manifest-path crates/desktop/Cargo.toml` and `cargo test --manifest-path crates/desktop/Cargo.toml`
- **Success criteria**: Backspace correctly removes the last character from the document buffer.
- **Edge cases**: Empty document should remain empty on Backspace. Multibyte characters should be removed entirely, not corrupting the string.

## 5. Constraints

- **Non-goals**: Delete handling (forward delete), cursor position handling (deletion is always at the end), or text selection deletion.
- **Forbidden changes**: Do not modify `crates/editor-core/src/lib.rs` or structural components of winit.
- **Scope ceiling**: Can be implemented as a single new match arm for keyboard events.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
