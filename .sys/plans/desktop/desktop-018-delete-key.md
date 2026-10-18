# desktop-018 — Forward delete key handling

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-018`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Implement forward delete handling in the desktop application using the `Delete` key.
- **Vision gap**: A usable local desktop editor requires standard keyboard input paths. While `Backspace` handles backward deletion, forward deletion is currently missing.
- **Evidence**: `crates/desktop/src/main.rs` matches `Key::Named(NamedKey::Backspace)` but lacks a match for `Key::Named(NamedKey::Delete)`.
- **Impact**: Provides standard forward deletion capabilities for a familiar text editing experience.

## 2. File Inventory

```json
{
  "id": "desktop-018",
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

- **Required behavior**: When the `Delete` key is pressed: if text is selected, delete the selection. If no text is selected and the cursor is not at the end of the document, delete the character immediately to the right of the cursor without moving the cursor's logical position.
- **Compatibility constraints**: Must integrate with `ApplicationHandler::window_event` like other keyboard shortcuts.
- **Architectural constraints**: Maintain separation of UI and document logic.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Check if pressing the `Delete` key deletes the character forward.
- **Verification commands**: `cargo test --manifest-path crates/desktop/Cargo.toml`, `python3 tooling/check_scaffold.py --role desktop`.
- **Success criteria**: The application handles forward deletion correctly, properly advancing logical state and triggering a redraw.
- **Edge cases**: End of document deletion, text selection deletion, multi-byte character deletion.

## 5. Constraints

- **Non-goals**: Modifying other cursor behaviors.
- **Forbidden changes**: Modifying the core lean-editor-core API.
- **Scope ceiling**: Forward delete is a single self-contained text mutation path.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
