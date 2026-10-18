# [desktop-019] — Implement Select All keyboard shortcut

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-019`

## 1. Context & Goal

- **Objective**: Implement Select All functionality via `Cmd/Ctrl+A` in the desktop editor.
- **Vision gap**: The desktop app supports text selection via shift+arrows, but it lacks the standard keyboard shortcut `Cmd/Ctrl+A` to select the entire document, which causes friction in fluent keyboard use.
- **Evidence**: `crates/desktop/src/main.rs` contains keyboard handling for copy/paste and cursor movement but no handler for `Key::Character("a")` with modifiers.
- **Impact**: Accelerates full-document edits and replacement.

## 2. File Inventory

```json
{
  "id": "desktop-019",
  "role": "desktop",
  "create": [],
  "modify": [
    "crates/desktop/src/main.rs"
  ],
  "read_only": [],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: When `Cmd/Ctrl+A` is pressed, the entire text of the document is selected. The `selection_anchor` is set to the start of the document (`0`) and the `cursor_pos` is set to the end of the document (`self.doc_char_count()`).
- **Compatibility constraints**: Standard keybindings for other platforms (Windows/Linux using Ctrl, macOS using Cmd).
- **Architectural constraints**: Update `selection_anchor` and `cursor_pos` in `App` state and request redraw.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Run the app and press `Cmd/Ctrl+A`; nothing happens.
- **Verification commands**:
  - `cargo check --manifest-path crates/desktop/Cargo.toml`
  - `cargo test --manifest-path crates/desktop/Cargo.toml`
  - `python3 tooling/check_scaffold.py --role desktop`
  - `bash tooling/validate_repo.sh`
- **Success criteria**: Tests pass, repo is valid, and the visual selection covers the entire document text.
- **Edge cases**: Empty document.

## 5. Constraints

- **Non-goals**: Modifying clipboard or other shortcuts.
- **Forbidden changes**: Modifying `editor-core` or changing the core text representation.
- **Scope ceiling**: Fits in one run, adding one `WindowEvent::KeyboardInput` arm.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A

## 7. Result
