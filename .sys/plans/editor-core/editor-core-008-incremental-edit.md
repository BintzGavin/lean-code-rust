# EDITOR-CORE-008 — Incremental edit API

**Role**: `editor-core`
**Backlog entry**: `.sys/backlogs/editor-core.md#EDITOR-CORE-008`

## 1. Context & Goal

- **Objective**: Add an incremental editing API to `Document` to allow replacing specific character ranges without replacing the whole buffer.
- **Vision gap**: Editing, dirty state, save and cancellation preserve user data. Current whole-document replacement via `replace_text` limits editor integration efficiency.
- **Evidence**: `crates/editor-core/src/lib.rs` currently only has `pub fn replace_text(&mut self, text: impl Into<String>)`.
- **Impact**: Unlocks incremental updates from the frontend, making edits faster and more memory efficient, paving the way for LSP integration.

## 2. File Inventory

```json
{
  "id": "EDITOR-CORE-008",
  "role": "editor-core",
  "create": [],
  "modify": [
    "crates/editor-core/src/lib.rs",
    "crates/editor-core/tests/document.rs",
    "crates/editor-core/README.md"
  ],
  "read_only": [],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Add a method `edit_range(&mut self, start_char: usize, end_char: usize, new_text: &str)` to `Document`. It replaces the text between `start_char` and `end_char` (0-indexed character offsets) with `new_text`. It should panic if indices are out of bounds. It must update the undo/redo stack properly.
- **Compatibility constraints**: Existing `replace_text` must continue to work. Undo/redo must correctly revert the incremental change.
- **Architectural constraints**: Maintain the standard-library-only requirement for `editor-core` product code.
- **Public contract changes**: Add `edit_range` to `Document`. Update `README.md` to mention it alongside `replace_text`.

## 4. Test Plan

- **Failing evidence**: No existing test uses `edit_range`. Attempting to call it would fail compilation.
- **Verification commands**:
  - `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml`
  - `cargo fmt --manifest-path crates/editor-core/Cargo.toml --check`
  - `CARGO_BUILD_JOBS=1 cargo run --manifest-path crates/editor-core/Cargo.toml --example edit_roundtrip`
- **Success criteria**: Tests demonstrate `edit_range` works correctly, handles bounds, and integrates with undo/redo. All core tests and checks pass.
- **Edge cases**: `start_char` == `end_char` (insertion), `start_char` > `end_char` (panic), out-of-bounds indices (panic), empty `new_text` (deletion), UTF-8 character boundaries (must work with chars, not bytes, or document expectations on char/byte indices).

## 5. Constraints

- **Non-goals**: No multi-cursor edits or concurrent edit synchronization in this work order.
- **Forbidden changes**: Do not add external dependencies to `crates/editor-core/Cargo.toml`.
- **Scope ceiling**: A single new method and tests easily fit in one execution.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
