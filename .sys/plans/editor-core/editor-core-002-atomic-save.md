# EDITOR-CORE-002 — Atomic save implementation

**Role**: `editor-core`
**Backlog entry**: `.sys/backlogs/editor-core.md#EDITOR-CORE-002`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Implement atomic saves to prevent file truncation or partial writes on failure.
- **Vision gap**: User data and security boundaries survive errors and interruption.
- **Evidence**: `crates/editor-core/src/lib.rs` uses `fs::write` directly. `crates/editor-core/README.md` explicitly calls out: "Saves use `std::fs::write`: they are not atomic, crash-safe or synchronized to stable storage. Failure preserves the in-memory buffer and saved snapshot, but may leave a truncated or partially written file."
- **Impact**: Ensures that when saving fails, the file on disk remains completely untouched and valid, preventing user data loss.

## 2. File Inventory

```json
{
  "id": "EDITOR-CORE-002",
  "role": "editor-core",
  "create": [],
  "modify": [
    "crates/editor-core/src/lib.rs",
    "crates/editor-core/tests/document.rs",
    "crates/editor-core/README.md"
  ],
  "read_only": [
    "VISION.md",
    "spec/requirements.md",
    "spec/interaction-cases.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The `Document::save` method must write to a temporary file in the same directory and then atomically rename it to the target path. On any failure before or during the rename, the original target file must remain completely unmodified.
- **Compatibility constraints**: The existing `Document` API signature should remain unchanged, returning `io::Result<()>`. The behavior of keeping the buffer and dirty state intact on error must be preserved.
- **Architectural constraints**: Rust standard library only. Do not introduce third-party crate dependencies for this. Ensure the temporary file is created in the same directory as the target path to allow atomic rename.
- **Public contract changes**: The documentation in `src/lib.rs` and `README.md` should be updated to state that saves are atomic.

## 4. Test Plan

- **Failing evidence**: `crates/editor-core/tests/document.rs` tests failed saves, but currently does not (and cannot) assert that the disk file was not truncated mid-write in a failure scenario, because `fs::write` replaces it.
- **Verification commands**:
  - `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml --offline --locked`
  - `cargo fmt --manifest-path crates/editor-core/Cargo.toml --check`
  - `CARGO_BUILD_JOBS=1 cargo run --manifest-path crates/editor-core/Cargo.toml --example edit_roundtrip --offline --locked`
  - `python3 tooling/check_scaffold.py --role editor-core`
- **Success criteria**: All existing tests pass. A new test case verifying that an interrupted save (if simulatable, e.g. via permission denial on the temp file or directory after the target exists) leaves the original target file completely intact and unmodified on disk.
- **Edge cases**: Existing file is a symlink (if standard atomic rename overwrites the symlink rather than the target, document this limitation or resolve it within std bounds, but atomic replace is priority). The file is in a directory with restricted permissions where creating temp files might fail.

## 5. Constraints

- **Non-goals**: Crash durability (e.g. `fsync` or `O_DSYNC` to physical media) is not strictly required in this slice, only atomic rename to prevent partial writes. File metadata (permissions, ownership) preservation is a non-goal for this slice unless std library handles it trivially. Cross-device renames are a non-goal; temp file must be in same dir.
- **Forbidden changes**: Do not add dependencies to `Cargo.toml`. Do not change the `Document` struct's public interface footprint (types).
- **Scope ceiling**: This requires only changing the implementation of `save` to use standard `fs::File`, `write_all`, and `fs::rename`, plus adding a test and updating documentation.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: Target file directory cannot be written to to create temp files (expected to fail the save, but original file must still be intact).

## 7. Result
Implemented atomic save using a temporary file in the same directory and `std::fs::rename`. Added a test to verify that a failed rename leaves the original file completely intact and deletes the temp file. All tests and verification commands passed. Updated `README.md` and `lib.rs` docs to reflect the new behavior.
