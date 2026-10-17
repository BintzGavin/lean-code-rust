# [desktop-015] — Render text and cursor in the window

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-015`

## 1. Context & Goal

- **Objective**: Render the document's text and the cursor position in the application window.
- **Vision gap**: The `desktop` crate currently manages document state, text input, and cursor navigation, but the `RedrawRequested` event is missing, meaning the native window is completely blank and useless. "The developer should reach useful, visible file content and inspect changes with as little waiting as possible; opening a window alone is not success."
- **Evidence**: `crates/desktop/src/main.rs` contains no handling for `WindowEvent::RedrawRequested`. Running the app produces a blank screen.
- **Impact**: The developer will be able to see the text they are editing and the cursor's location, establishing a minimal usable local code editor view.

## 2. File Inventory

```json
{
  "id": "desktop-015",
  "role": "desktop",
  "create": [],
  "modify": ["crates/desktop/Cargo.toml", "crates/desktop/src/main.rs"],
  "read_only": [],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: When the `winit` event loop requests a redraw (`WindowEvent::RedrawRequested`), the application must draw the document text and a visual indicator of the cursor position to the window.
- **Compatibility constraints**: Must remain within the `desktop` domain (Rust, native OS rendering). Ensure `window.request_redraw()` is called on state changes (e.g. key presses, input) so the window stays up-to-date.
- **Architectural constraints**: Keep dependencies lightweight and focused on performance. Do not introduce an entire web rendering engine or heavy UI framework.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `cargo run --manifest-path crates/desktop/Cargo.toml` opens a window that remains entirely blank.
- **Verification commands**:
  - `cargo check --manifest-path crates/desktop/Cargo.toml`
  - `cargo test --manifest-path crates/desktop/Cargo.toml`
  - `python3 tooling/check_scaffold.py --role desktop`
  - `bash tooling/validate_repo.sh`
- **Success criteria**: The window displays the text of the currently loaded or edited document. A cursor is visible at the correct `cursor_pos`. Typing, moving the cursor, and saving update the visual state.
- **Edge cases**: Empty document (only cursor visible), text longer than window bounds (basic clipping or scrolling if trivial).

## 5. Constraints

- **Non-goals**: Advanced text layout (ligatures, bidi), syntax highlighting, minimap, scrollbars.
- **Forbidden changes**: Do not modify `lean-editor-core` or any paths outside `crates/desktop/`.
- **Scope ceiling**: Basic 2D rendering or text drawing to unblock visual feedback, achievable in one execution run using a suitable crate like `wgpu`, `softbuffer`, or `tiny-skia` + `rusttype`/`font-kit`.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
