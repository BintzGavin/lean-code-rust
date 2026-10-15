# desktop-003 — Keyboard input handling

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-003`

## 1. Context & Goal

- **Objective**: Implement basic keyboard input handling in the native window.
- **Vision gap**: The application requires keyboard navigation paths for a usable local desktop editor.
- **Evidence**: `crates/desktop/src/main.rs` currently only handles `WindowEvent::CloseRequested` and ignores all other input.
- **Impact**: Enables future keyboard shortcuts, text input, and command palette invocation.

## 2. File Inventory

```json
{
  "id": "desktop-003",
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

- **Required behavior**: The window must close when the Escape key is pressed.
- **Compatibility constraints**: Must use `winit`'s stable event loop API.
- **Architectural constraints**: Keep logic within `ApplicationHandler`.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Run the application and press Escape; the window does not close.
- **Verification commands**: `cargo test --manifest-path crates/desktop/Cargo.toml`, `cargo check --manifest-path crates/desktop/Cargo.toml`
- **Success criteria**: The code compiles and keyboard events are properly routed.
- **Edge cases**: None.

## 5. Constraints

- **Non-goals**: Text rendering, complex keybindings.
- **Forbidden changes**: Do not add external dependencies.
- **Scope ceiling**: Fits in one run because it only matches `WindowEvent::KeyboardInput`.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: None

## 7. Result
