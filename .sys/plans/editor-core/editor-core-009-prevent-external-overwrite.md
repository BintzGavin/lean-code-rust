# EDITOR-CORE-009 — Prevent external overwrite

**Role**: `editor-core`
**Backlog entry**: `.sys/backlogs/editor-core.md#EDITOR-CORE-009`

## 1. Context & Goal

- **Objective**: Prevent saving a document if it has been modified externally since it was opened or last saved.
- **Vision gap**: Editing, dirty state, save and cancellation preserve user data. Overwriting external changes without explicit user consent risks data loss.
- **Evidence**: `has_external_changes` exists but is not checked in `save()` or `save_as()`. Tests explicitly document that concurrent changes can be overwritten.
- **Impact**: Ensures data safety by preventing accidental overwrites of concurrent modifications.

## 2. File Inventory

```json
{
  "id": "EDITOR-CORE-009",
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

- **Required behavior**: Modify `save()` and `save_as()` to return an `io::ErrorKind::AlreadyExists` (or similar appropriate error) if `has_external_changes()` returns true.
- **Compatibility constraints**: Must not interfere with normal saves where no external changes have occurred.
- **Architectural constraints**: Maintain the standard-library-only requirement for `editor-core` product code.
- **Public contract changes**: `save()` and `save_as()` will return an error when external changes are detected. Update `README.md` to reflect this new behavior.

## 4. Test Plan

- **Failing evidence**: No existing test asserts that `save()` fails when `has_external_changes()` is true.
- **Verification commands**:
  - `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml`
  - `cargo fmt --manifest-path crates/editor-core/Cargo.toml --check`
  - `CARGO_BUILD_JOBS=1 cargo run --manifest-path crates/editor-core/Cargo.toml --example edit_roundtrip`
- **Success criteria**: Tests demonstrate `save()` fails when external changes exist. All core tests and checks pass.
- **Edge cases**: The file is deleted externally (should likely fail).

## 5. Constraints

- **Non-goals**: Resolving conflicts or prompting the user. The scope is strictly limited to failing the save operation.
- **Forbidden changes**: Do not add external dependencies to `crates/editor-core/Cargo.toml`.
- **Scope ceiling**: A simple check and error return, plus tests, fits in one execution.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
