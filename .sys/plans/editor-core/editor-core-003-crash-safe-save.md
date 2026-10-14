# EDITOR-CORE-003 — Crash-safe document persistence

**Role**: `editor-core`
**Backlog entry**: `.sys/backlogs/editor-core.md#EDITOR-CORE-003`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Implement crash-safe saves by synchronizing data to stable storage and avoiding cross-platform locking errors.
- **Vision gap**: User data and security boundaries survive errors and interruption.
- **Evidence**: `crates/editor-core/src/lib.rs` uses `fs::write` directly for the temporary file, and does not synchronize the file to disk before renaming. The current implementation in `EDITOR-CORE-002` added atomic rename, but left the data vulnerable to system crashes before the OS flushes the temporary file to disk. Furthermore, explicitly dropping the file handle before `fs::rename` is missing, which can cause cross-platform file locking errors (e.g., on Windows). `crates/editor-core/README.md` notes that saves are "not crash-safe or synchronized to stable storage".
- **Impact**: Ensures that when a save operation completes successfully, the data is guaranteed to be durably written to stable storage, preventing data loss even if the system crashes immediately after. It also prevents file lock issues during rename on certain platforms.

## 2. File Inventory

```json
{
  "id": "EDITOR-CORE-003",
  "role": "editor-core",
  "create": [],
  "modify": [
    "crates/editor-core/src/lib.rs",
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

- **Required behavior**: The `Document::save` method must create a `std::fs::File`, write the bytes to it, and call `sync_data()` on the file handle to ensure the data is flushed to physical media before the atomic rename. The file handle must be explicitly dropped (e.g., `drop(file)`) before invoking `fs::rename` to prevent cross-platform file locking errors.
- **Compatibility constraints**: The existing `Document` API signature should remain unchanged, returning `io::Result<()>`.
- **Architectural constraints**: Rust standard library only.
- **Public contract changes**: The documentation in `src/lib.rs` and `README.md` should be updated to state that saves are now synchronized to stable storage (crash-safe) and handle cross-platform file locks appropriately.

## 4. Test Plan

- **Failing evidence**: No failing tests exist because crash safety and Windows file locking are not currently tested, but the implementation relies on OS buffers.
- **Verification commands**:
  - `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml --offline --locked`
  - `cargo fmt --manifest-path crates/editor-core/Cargo.toml --check`
  - `CARGO_BUILD_JOBS=1 cargo run --manifest-path crates/editor-core/Cargo.toml --example edit_roundtrip --offline --locked`
  - `python3 tooling/check_scaffold.py --role editor-core`
- **Success criteria**: All existing tests must pass, proving the new synchronization and dropping behavior doesn't break atomic rename or successful saves.
- **Edge cases**: `sync_data()` could fail (e.g., device full during flush, or device removed). This error must be propagated and the temporary file cleaned up, without touching the target file.

## 5. Constraints

- **Non-goals**: Modifying the test suite to physically simulate crashes or cross-platform behavior is out of scope.
- **Forbidden changes**: Do not add third-party crate dependencies to `Cargo.toml`.
- **Scope ceiling**: This requires only changing the implementation of `save` to use standard `fs::File`, `write_all`, `sync_data`, and `drop`, plus updating documentation.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A

## 7. Result

Implemented crash-safe save via `File::create`, `write_all`, `sync_data`, and explicit drop before rename. Updated README docs. All tests and checks passed.
