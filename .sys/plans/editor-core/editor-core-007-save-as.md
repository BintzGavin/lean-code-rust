# EDITOR-CORE-007 — Implement save_as for Document

**Role**: `editor-core`
**Backlog entry**: `.sys/backlogs/editor-core.md#EDITOR-CORE-007`

## 1. Context & Goal

- **Objective**: Implement a `save_as` method to allow a document to be saved to a new path.
- **Vision gap**: The edit/save contract currently only supports saving to the originally opened path; users cannot save a document to a new location.
- **Evidence**: `crates/editor-core/src/lib.rs` lacks a `save_as` method.
- **Impact**: Unlocks the ability to duplicate or rename documents by saving them to a different path, preserving user data when a new file is desired.

## 2. File Inventory

```json
{
  "id": "EDITOR-CORE-007",
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

- **Required behavior**: `save_as(new_path)` must save the current text to `new_path`, update the document's internal path, and clear the dirty state. It should use the same crash-safe temp file and rename logic as `save()`.
- **Compatibility constraints**: Must continue to use only the Rust standard library.
- **Architectural constraints**: The new method must maintain atomic and crash-safe save guarantees.
- **Public contract changes**: Add `pub fn save_as(&mut self, new_path: impl AsRef<std::path::Path>) -> std::io::Result<()>` to `Document`.

## 4. Test Plan

- **Failing evidence**: `cargo test --manifest-path crates/editor-core/Cargo.toml` fails if tests for `save_as` are added but the method does not exist.
- **Verification commands**:
  `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml`
  `cargo fmt --manifest-path crates/editor-core/Cargo.toml --check`
- **Success criteria**: A document can be saved to a new path, its internal path is updated, and dirty state is cleared.
- **Edge cases**: Failed `save_as` due to filesystem errors must not change the document's internal path or clear its dirty state.

## 5. Constraints

- **Non-goals**: Modifying the original file or cleaning it up (that is left to the caller).
- **Forbidden changes**: Do not introduce external dependencies in `editor-core`.
- **Scope ceiling**: Fits in one execution run by extending the existing save logic.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: None

## 7. Result
