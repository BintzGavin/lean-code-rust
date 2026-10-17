# desktop-017 — Home and End key navigation

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-017`

## 1. Context & Goal

- **Objective**: Implement Home and End key navigation to jump to the start and end of the current line.
- **Vision gap**: "A usable local desktop editor, including keyboard and accessibility paths." The editor lacks standard line boundary navigation, slowing down text editing.
- **Evidence**: `crates/desktop/src/main.rs` contains keyboard handling for arrow keys but no handlers for `NamedKey::Home` or `NamedKey::End`.
- **Impact**: Developers can efficiently jump to the beginning or end of a line, matching standard text editor behavior.

## 2. File Inventory

```json
{
  "id": "desktop-017",
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

- **Required behavior**:
  - `NamedKey::Home`: Move the cursor to the beginning of the current line (column 0). If Shift is pressed, update the selection anchor.
  - `NamedKey::End`: Move the cursor to the end of the current line (just before the newline character, or the end of the document if it's the last line). If Shift is pressed, update the selection anchor.
- **Compatibility constraints**: Integrate with the existing `selection_anchor`, `cursor_pos`, and text positional utilities in `main.rs` (e.g., `get_line_col`, `get_pos_from_line_col`).
- **Architectural constraints**: Handle events within the `winit` event loop in `main.rs`.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Pressing Home or End in the desktop application currently does not move the cursor.
- **Verification commands**:
  - `cargo check --manifest-path crates/desktop/Cargo.toml`
  - `cargo test --manifest-path crates/desktop/Cargo.toml`
  - `python3 tooling/check_scaffold.py --role desktop`
  - `bash tooling/validate_repo.sh`
- **Success criteria**: Pressing Home moves the cursor to the start of the line. Pressing End moves the cursor to the end of the line. Both work correctly with Shift for text selection.
- **Edge cases**: Pressing Home at the start of a line (should stay there). Pressing End at the end of a line (should stay there). Empty document (should stay at 0). First/last lines without trailing newlines.

## 5. Constraints

- **Non-goals**: Page Up/Page Down, Ctrl+Home/Ctrl+End (beginning/end of document) are excluded.
- **Forbidden changes**: Do not modify `lean-editor-core` or paths outside `crates/desktop/`.
- **Scope ceiling**: Standard single-line Home/End navigation.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
Implemented Home and End keys for jumping to the start and end of lines, including support for Shift-based text selection.
