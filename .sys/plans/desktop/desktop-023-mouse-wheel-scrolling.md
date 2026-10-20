# desktop-023 — Mouse wheel scrolling

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-023`

## 1. Context & Goal

- **Objective**: Implement vertical scrolling using the mouse wheel or trackpad.
- **Vision gap**: A usable local desktop editor must allow fluent navigation of large documents, but currently, users cannot use their mouse wheel or trackpad to scroll the viewport.
- **Evidence**: `WindowEvent::MouseWheel` is not handled in `crates/desktop/src/main.rs`, and `Renderer` does not expose a way to adjust `scroll_y` directly.
- **Impact**: Enables smooth and intuitive vertical scrolling of documents.

## 2. File Inventory

```json
{
  "id": "desktop-023",
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

- **Required behavior**: The renderer must expose a way to directly scroll the viewport vertically by a given delta. `main.rs` must handle `WindowEvent::MouseWheel` and invoke this new renderer function to adjust the scroll offset accordingly.
- **Compatibility constraints**: Scrolling must respect the document bounds and not scroll past the top (0.0). Scrolling behavior should be relatively smooth or at least responsive to delta values provided by winit.
- **Architectural constraints**: Offset logic remains inside `Renderer`.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Moving the scroll wheel currently does nothing.
- **Verification commands**: `cargo check --manifest-path crates/desktop/Cargo.toml` and `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/desktop/Cargo.toml`
- **Success criteria**: Scrolling the mouse wheel or trackpad up and down vertically scrolls the text in the window.
- **Edge cases**: Scrolling past the top of the file should clamp `scroll_y` to 0.0. Scrolling past the bottom can optionally clamp, or at least be handled reasonably.

## 5. Constraints

- **Non-goals**: Horizontal scrolling is not required. Kinetic/momentum scrolling is not required if not provided natively by the OS event.
- **Forbidden changes**: Do not introduce UI scrollbars.
- **Scope ceiling**: Handling one new event type and exposing one method on `Renderer` fits in a single run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
