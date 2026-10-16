# desktop-011 — Enter key handling

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-011`

## 1. Context & Goal

- **Objective**: Handle the Enter key to insert a newline into the document.
- **Vision gap**: A usable local desktop editor requires editing capabilities, which includes inserting newlines via Enter/Return (Vision promise 1).
- **Evidence**: `crates/desktop/src/main.rs` processes text input via `WindowEvent::KeyboardInput` and Backspace, but lacks any logic for `Key::Named(NamedKey::Enter)` to insert newlines. The `WindowEvent::KeyboardInput` match arm for `text: Some(t)` usually doesn't emit a newline on Return in winit, or if it does, handling `NamedKey::Enter` explicitly is safer and guarantees the correct behavior for standard keyboard editing.
- **Impact**: Enables users to write multi-line documents by pressing Enter.

## 2. File Inventory

```json
{
  "id": "desktop-011",
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

- **Required behavior**: In the `window_event` method's `KeyboardInput` match arm, add a new case to handle the Enter key (`logical_key: Key::Named(NamedKey::Enter)`). When pressed, append a newline character `\n` to the document using `doc.replace_text()`. We use `replace_text()` because `main.rs` already uses it for appending text.
- **Compatibility constraints**: Must correctly append a newline character without crashing.
- **Architectural constraints**: Keep logic within `main.rs`'s event loop.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Pressing Enter currently appends nothing or a control character, and does not insert a new line visually or in the saved buffer correctly.
- **Verification commands**: `cargo check --manifest-path crates/desktop/Cargo.toml` and `cargo test --manifest-path crates/desktop/Cargo.toml`
- **Success criteria**: Enter correctly appends a `\n` to the document buffer.
- **Edge cases**: Empty document should insert a newline correctly.

## 5. Constraints

- **Non-goals**: Multi-line cursor support or complex multi-line text rendering. This strictly focuses on the internal buffer representation.
- **Forbidden changes**: Do not modify `crates/editor-core/src/lib.rs` or structural components of winit.
- **Scope ceiling**: Can be implemented as a single new match arm for keyboard events.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
