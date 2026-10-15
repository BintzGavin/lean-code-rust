# desktop-002 — Initialize native window and event loop

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-002`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Initialize a native application window and event loop in the `desktop` crate.
- **Vision gap**: The Vision requires a usable local desktop editor, but the current `desktop` crate only contains an empty CLI scaffolding without a graphical window or event loop.
- **Evidence**: `crates/desktop/src/main.rs` only prints a placeholder message and exits immediately.
- **Impact**: Unlocks the ability to render UI and receive user input, forming the foundation for the graphical editor.

## 2. File Inventory

```json
{
  "id": "desktop-002",
  "role": "desktop",
  "create": [],
  "modify": [
    "crates/desktop/Cargo.toml",
    "crates/desktop/src/main.rs"
  ],
  "read_only": [
    "VISION.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The application must open a native window with a title, run an event loop that prevents immediate exit, and close cleanly when the user requests to close the window.
- **Compatibility constraints**: Must support macOS as per the Vision requirements.
- **Architectural constraints**: No webviews or heavy UI frameworks for this basic foundation; stick to a minimal windowing library (e.g., winit) or platform-native APIs.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Running `cargo run --manifest-path crates/desktop/Cargo.toml` prints to stdout and exits instantly, demonstrating the lack of a window or event loop.
- **Verification commands**:
  ```sh
  cargo check --manifest-path crates/desktop/Cargo.toml
  cargo test --manifest-path crates/desktop/Cargo.toml
  ```
- **Success criteria**: The crate compiles with the new windowing dependency, and the application opens a stable window when executed.
- **Edge cases**: The application should gracefully terminate the process when the window close event is triggered.

## 5. Constraints

- **Non-goals**: Rendering text buffers, integrating with `editor-core`, or handling complex keyboard inputs beyond closing the window.
- **Forbidden changes**: Do not modify files outside `crates/desktop/`.
- **Scope ceiling**: Setting up a basic window and event loop is a well-bounded initial step for the graphical client.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: A suitable native windowing library cannot be resolved or compiled on the target platform.

## 7. Result
Completed successfully. Added winit dependency and initialized native window and event loop in main.rs.
