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

### 2026-10-16 — explicit human file-safety correction

Claim: `recovery-safety-2026-10-16`, direct human-authorized correction of
PR1699927158 behavior at baseline `cf8ba7fcda073780f4eed0abed6faa95ccac293b`.
This is not recurring backlog execution: completed entries remain terminal,
no new work order is seeded, and no backlog/status lifecycle is changed.
Scope: recovery writes/restores and their interaction with save cleanup;
synthetic regression tests first, no new dependencies or publication.
The safety correction deliberately supersedes this plan's automatic cleanup
contract: backup paths are caller-owned and retained; existing entries must
not be truncated or adopted for deletion. Verification pending.

Correction outcome (2026-10-16): implemented for independent review, not committed
or published. `write_backup` uses exclusive `create_new`, rejects the document's
canonical-parent destination, and rejects every same-canonical-directory backup
when the document entry is missing (case/Unicode filename equivalence cannot be
proved portably). A separate existing directory remains usable in that case;
a dangling document symlink fails closed. `restore_backup` borrows the source;
all automatic backup deletion was removed, including after save. Caller-owned
copies remain until explicit caller disposal. Explicit `save_as(backup_path)`
still intentionally replaces that destination and leaves the saved file present.
Existing exclusive save-temp creation, `sync_data`, explicit drop, and rename
sequence are unchanged. No desktop recovery call sites were found by
`rg -n 'write_backup|restore_backup' crates/desktop`.

Changed correction files: `crates/editor-core/src/lib.rs`,
`crates/editor-core/tests/recovery_safety.rs`,
`crates/editor-core/tests/document.rs` (replace unsafe cleanup expectation),
`crates/editor-core/README.md`, and this appended Result only.

Red/green evidence from synthetic temp files on this Mac:

- Baseline production library compiled directly with `rustc`; initial
  `recovery_safety` suite: 14 failed / 2 passed before the fix. Failures:
  `backup_rejects_existing_unrelated_file_without_changing_state`,
  `backup_rejects_same_lexical_and_canonical_document_paths`,
  `backup_rejects_missing_document_destination_and_lexical_alias`,
  `repeated_backup_write_does_not_replace_previous_recovery_copy`,
  `restore_current_document_then_save_keeps_document_even_without_edits`,
  `restored_unrelated_source_is_retained_after_save`,
  `written_backup_is_retained_after_save`,
  `replaced_backup_entry_is_not_deleted_after_save`,
  `explicit_save_as_to_written_or_restored_backup_keeps_saved_destination`,
  and Unix tests `backup_rejects_symlink_and_hardlink_entries`,
  `backup_rejects_dangling_symlink_without_creating_target`,
  `backup_rejects_missing_document_through_symlinked_parent`,
  `restore_document_alias_then_save_preserves_alias_entry`,
  `backup_path_retargeted_to_document_is_not_deleted_after_save`.
- Initial fix: 16/16 passed. Review then identified missing-filename equivalence
  on this filesystem; extended suite before second correction: 17 passed / 3
  failed. Exact failures: `missing_document_rejects_same_directory_backup_when_names_may_alias`,
  `missing_document_rejects_case_alias_on_case_insensitive_filesystems`, and
  `missing_document_rejects_unicode_alias_on_normalizing_filesystems`.
  Both capability-probed alias tests executed their assertions on this host.
- Final direct-rustc recovery suite: **20/20 passed**. The two existing
  `save_file_collision` and `save_symlink_collision` binaries: **1/1 each passed**.
  Direct-rustc `edit_roundtrip` example: passed and removed its synthetic files.
  The two collision binaries and example ran after the initial safety fix; the
  subsequent change only tightened missing-document backup path validation.
- `cargo fmt --manifest-path crates/editor-core/Cargo.toml --check`,
  `git diff --check`, role and repository scaffold checks passed;
  `python3 -m unittest discover -s tooling -p 'test_*.py'`: **23 passed**;
  `python3 spec/checks/check_package.py`: passed, still 64 blocked/unknown
  scenarios and zero executed product scenarios.

Direct std-only test invocation (one compiler/test process at a time):

```sh
rustc --edition=2021 --crate-type=lib --crate-name lean_editor_core crates/editor-core/src/lib.rs --out-dir /tmp/lean-recovery-validation
rustc --edition=2021 --test crates/editor-core/tests/recovery_safety.rs --extern lean_editor_core=/tmp/lean-recovery-validation/liblean_editor_core.rlib -o /tmp/lean-recovery-validation/recovery_safety
/tmp/lean-recovery-validation/recovery_safety
```

Validation blockers, not passing claims: requested
`CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml --offline --locked`
failed before compilation because workspace dependency `winit` is uncached.
Compiling the unchanged-dependency `tests/document.rs` binary directly failed
with E0433 for existing `filetime` usage at lines 198 and 431. The default Cargo
registry directory is absent. The nested core `Cargo.lock` lacks filetime;
the root workspace lock contains it. No lockfiles/dependencies were changed,
no downloads attempted, and the full 26-test document binary was not executed.
No GUI/heavy build was run (38% free memory at preflight).

Conservative limits: backups accumulate until caller disposal; repeated writes
require a fresh path; failed write/sync can leave a partial newly created backup;
missing-source backups need a different existing directory. No recovery registry,
directory synchronization/crash guarantee, or concurrent directory-retargeting
protection is claimed. These do not broaden the ordinary save contract.

Documentation accuracy follow-up: replaced pre-existing full crash-safety and
cross-platform lock claims in the touched README/lib API comments with the
observed `sync_data` → close → rename sequence and its unverified limits. No save
implementation or durability scope was changed.
