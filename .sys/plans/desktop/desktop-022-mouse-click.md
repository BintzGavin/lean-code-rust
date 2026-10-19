# desktop-022 — Mouse click to place cursor

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-022`

## 1. Context & Goal

- **Objective**: Implement mouse click interaction to place the text cursor at the clicked location.
- **Vision gap**: A usable local desktop editor requires developers to interact with the text using a pointer, but currently the user cannot click to place the text cursor, relying exclusively on keyboard navigation.
- **Evidence**: `crates/desktop/src/main.rs` processes keyboard events but drops mouse input. The `Renderer` (in `crates/desktop/src/render.rs`) determines character layout but does not expose a way to map window coordinates back to a document character index.
- **Impact**: Unlocks basic mouse interaction, allowing developers to quickly jump to specific code sections without repeatedly pressing arrow keys.

## 2. File Inventory

```json
{
  "id": "desktop-022",
  "role": "desktop",
  "create": [],
  "modify": [
    "crates/desktop/src/main.rs",
    "crates/desktop/src/render.rs"
  ],
  "read_only": [],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**:
  - Track the current mouse cursor position in `App` via `WindowEvent::CursorMoved`.
  - On `WindowEvent::MouseInput` with left click pressed, map the `(x, y)` coordinate to a character index in the document.
  - Update `self.cursor_pos` and clear the selection anchor if Shift is not held (optionally support Shift+Click for selection).
  - The `Renderer` must expose a new `hit_test(&self, text: &str, x: f32, y: f32) -> usize` method that faithfully mimics the text layout loop in `draw` to identify the character under the cursor.
- **Compatibility constraints**: The coordinate mapping in `hit_test` must incorporate the current `self.scroll_y` and static margins (20.0 padding) exactly as `draw` does.
- **Architectural constraints**: Keep layout logic encapsulated in `Renderer`. The `App` should not compute font metrics directly.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Clicking on text in the window has no effect on the cursor position.
- **Verification commands**:
  - `cargo check --manifest-path crates/desktop/Cargo.toml`
  - `cargo test --manifest-path crates/desktop/Cargo.toml`
  - `python3 tooling/check_scaffold.py --role desktop`
  - `bash tooling/validate_repo.sh`
- **Success criteria**: The code compiles and tests pass. Running the app allows the user to left-click within the window and the cursor instantly moves to the clicked character. Clicking past the end of a line places the cursor at the end of that line.
- **Edge cases**: Clicking below the last line places the cursor at the end of the document. Scrolling does not break coordinate mapping.

## 5. Constraints

- **Non-goals**: Mouse dragging to select text is excluded from this work order. Mouse scroll wheel support is excluded.
- **Forbidden changes**: Do not modify the existing core keyboard navigation logic.
- **Scope ceiling**: Tracking `CursorMoved` and `MouseInput` in `main.rs` and duplicating layout math into `hit_test` in `render.rs` fits comfortably within one execution run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
Successfully implemented mouse click to place cursor.
