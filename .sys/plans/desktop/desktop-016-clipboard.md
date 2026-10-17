# [desktop-016] — Clipboard operations (Copy, Cut, Paste)

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-016`

## 1. Context & Goal

- **Objective**: Implement copy, cut, and paste functionality in the desktop editor.
- **Vision gap**: "A usable local desktop editor, including keyboard and accessibility paths." The editor currently supports typing, deleting, and selecting text, but lacks standard clipboard operations, making it difficult to rearrange or duplicate code.
- **Evidence**: `crates/desktop/src/main.rs` contains keyboard handling for typing, navigation, and undo/redo, but no handlers for `Cmd/Ctrl+C`, `Cmd/Ctrl+X`, or `Cmd/Ctrl+V`.
- **Impact**: The developer will be able to efficiently move and duplicate text, bringing the editor closer to standard usability expectations.

## 2. File Inventory

```json
{
  "id": "desktop-016",
  "role": "desktop",
  "create": [],
  "modify": ["crates/desktop/Cargo.toml", "crates/desktop/src/main.rs"],
  "read_only": [],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**:
  - `Cmd/Ctrl+C`: Copy the currently selected text to the system clipboard.
  - `Cmd/Ctrl+X`: Copy the currently selected text to the system clipboard and delete it from the document.
  - `Cmd/Ctrl+V`: Insert the contents of the system clipboard at the cursor position (replacing any active selection).
- **Compatibility constraints**: Must integrate with the existing `lean-editor-core::Document` API for edits and the system clipboard via a suitable cross-platform crate (e.g., `arboard` or `copypasta`).
- **Architectural constraints**: Keep dependencies lightweight. The clipboard access should be handled safely within the `winit` event loop in `main.rs`.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Pressing `Cmd/Ctrl+C`, `Cmd/Ctrl+X`, or `Cmd/Ctrl+V` currently does nothing.
- **Verification commands**:
  - `cargo check --manifest-path crates/desktop/Cargo.toml`
  - `cargo test --manifest-path crates/desktop/Cargo.toml`
  - `python3 tooling/check_scaffold.py --role desktop`
  - `bash tooling/validate_repo.sh`
- **Success criteria**: Selected text can be copied to the clipboard. Selected text can be cut, removing it from the document and placing it on the clipboard. Text from the clipboard can be pasted into the document, replacing any active selection.
- **Edge cases**: Copying/cutting when no text is selected (should do nothing or copy the current line, depending on convention; doing nothing is acceptable for an initial implementation). Pasting empty or non-text clipboard content (should handle gracefully without crashing).

## 5. Constraints

- **Non-goals**: Multi-cursor support, clipboard history, rich text pasting.
- **Forbidden changes**: Do not modify `lean-editor-core` or paths outside `crates/desktop/`.
- **Scope ceiling**: Standard single-selection clipboard operations using an external crate.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
