# EDITOR-CORE-005 — Document reload

**Role**: `editor-core`
**Backlog entry**: `.sys/backlogs/editor-core.md#EDITOR-CORE-005`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Implement a `reload` method on the `Document` struct to refresh the buffer from disk.
- **Vision gap**: User data and security boundaries survive errors and interruption (specifically, B-0019: Handle unsaved work and external changes visibly).
- **Evidence**: `crates/editor-core/src/lib.rs` provides `open` and `has_external_changes`, but no way to reload a document if it has been externally modified or if the user wants to revert changes.
- **Impact**: Provides the necessary capability for higher layers to revert unsaved changes or load external modifications from disk, satisfying a core vision requirement.

## 2. File Inventory

```json
{
  "id": "EDITOR-CORE-005",
  "role": "editor-core",
  "create": [],
  "modify": [
    "crates/editor-core/src/lib.rs",
    "crates/editor-core/tests/document.rs",
    "crates/editor-core/README.md"
  ],
  "read_only": [
    "VISION.md",
    "spec/catalog.json"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Add a `reload(&mut self) -> io::Result<()>` method to `Document`. This method should read the current contents of the file at `self.path` and update both `text` and `persisted_text`. It should also update the `modified_time`. If the file does not exist or is invalid UTF-8, it should return an error and leave the document unchanged.
- **Compatibility constraints**: The existing `Document` API signature should remain unchanged, except for adding the new method.
- **Architectural constraints**: Rust standard library only.
- **Public contract changes**: A new method `pub fn reload(&mut self) -> io::Result<()>` will be added to the public API of `Document`. The `README.md` must be updated to document this capability.

## 4. Test Plan

- **Failing evidence**: No failing tests exist because `reload` is currently missing.
- **Verification commands**:
  - `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml`
  - `cargo fmt --manifest-path crates/editor-core/Cargo.toml --check`
  - `CARGO_BUILD_JOBS=1 cargo run --manifest-path crates/editor-core/Cargo.toml --example edit_roundtrip`
  - `python3 tooling/check_scaffold.py --role editor-core`
- **Success criteria**: All existing tests must pass. New integration tests in `tests/document.rs` must verify that `reload` successfully updates the buffer and clears the dirty flag when the file is modified on disk, and that a failed reload (e.g. file deleted or invalid UTF-8 written) leaves the existing document state intact.
- **Edge cases**: The file might be deleted externally or replaced with invalid UTF-8, in which case `reload()` should return an error and not modify the document's `text` or `persisted_text`.

## 5. Constraints

- **Non-goals**: UI prompts or automatic reloading based on file watchers. This is strictly a manual operation on the `Document` struct.
- **Forbidden changes**: Do not add external dependencies.
- **Scope ceiling**: This change is purely bounded to the `Document` struct in `editor-core` updating its buffer from disk and exposing a single new method.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A

## 7. Result
