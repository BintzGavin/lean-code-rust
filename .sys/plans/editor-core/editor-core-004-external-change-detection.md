# EDITOR-CORE-004 — External change detection

**Role**: `editor-core`
**Backlog entry**: `.sys/backlogs/editor-core.md#EDITOR-CORE-004`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Implement external file change detection by recording file modified time when opening or saving, and checking it on demand.
- **Vision gap**: User data and security boundaries survive errors and interruption (specifically, B-0019: Handle unsaved work and external changes visibly).
- **Evidence**: `crates/editor-core/src/lib.rs` currently only compares in-memory buffer with the text loaded at `open()`. `crates/editor-core/README.md` clearly states: "There is no external-change detection, locking, conflict handling or symlink isolation. Saves can overwrite concurrent changes."
- **Impact**: Provides the necessary capability for higher layers to detect external modifications and prevent unintended overwriting of data changed by external tools, satisfying a core vision requirement.

## 2. File Inventory

```json
{
  "id": "EDITOR-CORE-004",
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

- **Required behavior**: The `Document` struct must store the file modification time (`SystemTime`) when it is successfully opened or saved. Expose a method `has_external_changes(&self) -> io::Result<bool>` that queries the filesystem's current modification time for the file and returns `true` if it differs from the last recorded time.
- **Compatibility constraints**: The existing `Document` API signature should remain unchanged, except for adding the new method. The current behavior of `save()` and `open()` must persist their respective signatures.
- **Architectural constraints**: Rust standard library only.
- **Public contract changes**: A new method `pub fn has_external_changes(&self) -> io::Result<bool>` will be added to the public API of `Document`. The `README.md` must be updated to remove the statement about lacking external change detection.

## 4. Test Plan

- **Failing evidence**: No failing tests exist because external change detection is explicitly not supported.
- **Verification commands**:
  - `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml`
  - `cargo fmt --manifest-path crates/editor-core/Cargo.toml --check`
  - `CARGO_BUILD_JOBS=1 cargo run --manifest-path crates/editor-core/Cargo.toml --example edit_roundtrip`
  - `python3 tooling/check_scaffold.py --role editor-core`
- **Success criteria**: All existing tests must pass. A new integration test in `tests/document.rs` must verify that `has_external_changes` accurately reports `true` when the file is modified on disk by an external process, and `false` after a successful `save()` or immediately after `open()`.
- **Edge cases**: The file might be deleted externally, in which case `has_external_changes()` should return an error.

## 5. Constraints

- **Non-goals**: File watchers, autosave, prompt interactions, or UI changes. This is strictly querying metadata on demand.
- **Forbidden changes**: Do not add dependencies or spawn long-running threads/processes for watching.
- **Scope ceiling**: This change is purely bounded to the `Document` struct in `editor-core` tracking metadata and exposing a single new query method.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A

## 7. Result

Added modified_time to Document and implemented has_external_changes method. Added test and updated README. All checks passed.
