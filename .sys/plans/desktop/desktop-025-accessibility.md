# desktop-025 — Accessibility basics

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-025`

## 1. Context & Goal

- **Objective**: Implement basic accessibility by exposing text and cursor position to platform accessibility APIs using AccessKit.
- **Vision gap**: The `VISION.md` states "Keep accessibility basics". Currently, the custom `softbuffer`-based renderer is entirely opaque to screen readers on macOS.
- **Evidence**: `crates/desktop/src/main.rs` directly renders text via a pixel buffer, lacking any accessibility API integration.
- **Impact**: Enables screen readers to read the document content and track the cursor position, meeting the core accessibility requirement.

## 2. File Inventory

```json
{
  "id": "desktop-025",
  "role": "desktop",
  "create": [],
  "modify": [
    "crates/desktop/Cargo.toml",
    "crates/desktop/src/main.rs"
  ],
  "read_only": [
    "VISION.md",
    "crates/editor-core/src/lib.rs"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The desktop window should expose its text content and cursor position using AccessKit (winit integration). When the document changes, the accessibility tree should be updated.
- **Compatibility constraints**: The existing rendering (softbuffer) and input handling (winit events) must remain exactly the same. AccessKit runs parallel to the visual rendering.
- **Architectural constraints**: Keep dependencies minimal. Use `accesskit_winit` to provide the node tree.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Run the application and use a screen reader (e.g., VoiceOver on macOS). It currently sees an empty window.
- **Verification commands**:
  - `cargo check --manifest-path crates/desktop/Cargo.toml`
  - `cargo test --manifest-path crates/desktop/Cargo.toml`
  - `python3 tooling/check_scaffold.py --role desktop`
- **Success criteria**: The application compiles and tests pass.
- **Edge cases**: Empty document, very large document (tree update performance).

## 5. Constraints

- **Non-goals**: Full text editing semantics (like precise selection reporting or inline text edits) if they are too complex for a basic integration. The goal is basic exposure.
- **Forbidden changes**: Do not change `editor-core`.
- **Scope ceiling**: Basic document text and cursor position exposure.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A

## 7. Result
