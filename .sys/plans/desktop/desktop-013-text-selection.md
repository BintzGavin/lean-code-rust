# desktop-013 — Text selection with Shift+Arrow keys

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-013`

## 1. Context & Goal

- **Objective**: Add basic text selection capabilities using Shift + Arrow keys.
- **Vision gap**: A usable local desktop editor requires selection paths for editing efficiency and standard keyboard operations.
- **Evidence**: `crates/desktop/src/main.rs` only handles basic cursor position (`cursor_pos`) without an anchor or selection boundaries. Arrow keys currently update `cursor_pos` regardless of modifiers.
- **Impact**: Enables future editing operations (copy, paste, delete selection) and fulfills basic text editing expectations.

## 2. File Inventory

```json
{
  "id": "desktop-013",
  "role": "desktop",
  "create": [],
  "modify": ["crates/desktop/src/main.rs"],
  "read_only": [],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Holding Shift while pressing Arrow keys expands or shrinks a text selection. `App` state needs a `selection_anchor` or similar `Option<usize>` to track the starting point of the selection.
- **Compatibility constraints**: Standard cursor movement without Shift must clear the selection.
- **Architectural constraints**: Maintain current `WindowEvent::KeyboardInput` structure and `App` state management.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Shift+Arrow currently moves the cursor identically to bare Arrow keys, without persisting a selection state.
- **Verification commands**: `cargo check --manifest-path crates/desktop/Cargo.toml`, `cargo test --manifest-path crates/desktop/Cargo.toml`
- **Success criteria**: Code compiles, tests pass, and manual inspection confirms Shift+Arrow logic is present in the codebase and tracks selection bounds.
- **Edge cases**: Selecting text at the beginning or end of the document boundaries.

## 5. Constraints

- **Non-goals**: Rendering the text selection visually, clipboard operations (copy/paste), replacing selected text on input, and mouse drag selection.
- **Forbidden changes**: Do not modify `lean_editor_core`.
- **Scope ceiling**: Updating the state model and event routing for selection is a single cohesive change.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
Added selection_anchor to App state and updated arrow key handlers to track text selection bounds when Shift is pressed.
