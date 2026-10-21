# [desktop-024] — PageUp and PageDown keyboard navigation

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-024`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Implement PageUp and PageDown keyboard handling to allow fast vertical navigation through documents.
- **Vision gap**: The Vision requires fluent keyboard use, accessibility and discoverable actions. Currently, the desktop lacks PageUp and PageDown keys for fast vertical scrolling and cursor movement.
- **Evidence**: `crates/desktop/src/main.rs` contains no handlers for `NamedKey::PageUp` or `NamedKey::PageDown` inside the `WindowEvent::KeyboardInput` match arms.
- **Impact**: Unlocks fast vertical scrolling and cursor placement across large files, bringing navigation closer to expected desktop editor behavior.

## 2. File Inventory

```json
{
  "id": "desktop-024",
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

- **Required behavior**: Pressing PageUp moves the cursor up by a page-sized amount of lines and adjusts the viewport. Pressing PageDown moves the cursor down similarly. It must integrate with text selection if Shift is held, identical to the arrow keys behavior.
- **Compatibility constraints**: None.
- **Architectural constraints**: Maintain the current event loop structure in `App::window_event` and `Renderer` state logic.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Run the application and press PageUp or PageDown to observe no cursor movement or scrolling.
- **Verification commands**: `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/desktop/Cargo.toml` and `python3 tooling/check_scaffold.py --role desktop`.
- **Success criteria**: Pressing PageUp or PageDown moves the cursor in the document vertically and automatically scrolls the text view if the new cursor position goes out of the current viewport.
- **Edge cases**: PageUp at the top of the document correctly stops at the first line. PageDown at the bottom correctly stops at the last line.

## 5. Constraints

- **Non-goals**: Smooth animated scrolling or horizontal page navigation.
- **Forbidden changes**: Do not alter how existing navigation keys (Arrow Up/Down, Home, End) function.
- **Scope ceiling**: Can be completed within one execution run by calculating a page step size and reusing existing position translation functions.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: Winit does not deliver PageUp/PageDown events on the target platform.

## 7. Result

Completed successfully. Implemented PageUp and PageDown keyboard navigation.
