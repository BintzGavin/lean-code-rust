# desktop-021 — Viewport scrolling to follow cursor

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-021`

## 1. Context & Goal

- **Objective**: Implement viewport scrolling in the renderer to ensure the text cursor is always visible.
- **Vision gap**: A usable local desktop editor requires developers to see the text they are editing, but currently the renderer does not scroll, hiding text that overflows the window.
- **Evidence**: `crates/desktop/src/render.rs` hardcodes the starting Y coordinate (`let mut y = 20.0 + v_metrics.ascent;`) and does not track a scroll offset, causing the cursor to move off-screen when navigating long documents.
- **Impact**: Unlocks editing and navigating large files by keeping the active cursor area visible within the native window bounds.

## 2. File Inventory

```json
{
  "id": "desktop-021",
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

- **Required behavior**: The renderer must maintain a vertical scroll offset. When drawing, text rendering should be offset by this amount. The application must update this scroll offset when the cursor moves (e.g. typing, arrow keys) to ensure the cursor's Y position remains within the visible window height.
- **Compatibility constraints**: Existing font metrics and scaling from `rusttype` must continue to govern line height calculations.
- **Architectural constraints**: Keep the offset logic inside the `Renderer` or `App` state. The native window remains the source of truth for window dimensions.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Opening a large document or pressing Enter repeatedly causes the cursor to disappear below the window bottom.
- **Verification commands**: `cargo test --manifest-path crates/desktop/Cargo.toml`
- **Success criteria**: The cursor remains visibly drawn on screen, and text scrolls up when the cursor moves past the bottom of the window, or scrolls down when moving past the top.
- **Edge cases**: Resizing the window should not leave the cursor off-screen if it was previously visible.

## 5. Constraints

- **Non-goals**: Horizontal scrolling is not required for this iteration. Smooth scrolling animation is excluded.
- **Forbidden changes**: Do not introduce UI scrollbars or external rendering dependencies in this work order.
- **Scope ceiling**: Updating the Y coordinate offset based on the cursor position and window height fits cleanly within one execution run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
