# EDITOR-CORE-010 — Recovery journal for unsaved edits

**Role**: `editor-core`
**Backlog entry**: `.sys/backlogs/editor-core.md#EDITOR-CORE-010`

## 1. Context & Goal

- **Objective**: Provide a mechanism to persist and restore unsaved edits (a recovery journal or backup) to survive unexpected interruption or crashes.
- **Vision gap**: User data and security boundaries survive errors and interruption. Currently, unsaved edits exist only in memory and are lost if the process crashes.
- **Evidence**: `Document` stores edits in memory. The README states "no autosave, recovery journal".
- **Impact**: Ensures unsaved changes can be recovered after a crash or unexpected termination.

## 2. File Inventory

```json
{
  "id": "EDITOR-CORE-010",
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

- **Required behavior**: Add capabilities to write the current unsaved buffer state to a separate backup file/journal, and a mechanism to detect and restore from it upon opening or reloading.
- **Compatibility constraints**: Standard saves should still behave normally and clear the backup. Normal `open` should remain unaffected unless explicitly instructed to recover.
- **Architectural constraints**: Maintain standard-library-only requirement. Do not introduce background threads or async tasks for autosave; expose synchronous methods for the caller to invoke backup writes.
- **Public contract changes**: Add backup/recovery related methods to `Document` (e.g., `write_backup`, `restore_backup`). Update `README.md` to document the recovery journal capabilities.

## 4. Test Plan

- **Failing evidence**: No existing tests cover recovering unsaved edits after dropping a `Document` without saving.
- **Verification commands**:
  - `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml`
  - `cargo fmt --manifest-path crates/editor-core/Cargo.toml --check`
  - `CARGO_BUILD_JOBS=1 cargo run --manifest-path crates/editor-core/Cargo.toml --example edit_roundtrip`
- **Success criteria**: Tests demonstrate that an unsaved buffer can be written to a backup, and subsequently restored in a new `Document` instance, and that standard saves clear the backup.
- **Edge cases**: Existing backup from another session, corrupted backup file, saving clearing the backup file, and restoring when no backup exists.

## 5. Constraints

- **Non-goals**: Implementing an automatic background autosave loop (this is the caller's responsibility).
- **Forbidden changes**: Do not add external dependencies to `crates/editor-core/Cargo.toml`.
- **Scope ceiling**: Adding a few synchronous methods for writing and reading a backup file fits in one execution.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
