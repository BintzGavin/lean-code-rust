# Independent document core

A standalone Rust standard-library-only UTF-8 document buffer. This is the
scoped EDITOR-CORE-001 contract, implemented from this repository's work order
and read-only specification without existing editor source.

`Document::open(path)` retains the supplied path and exact UTF-8 text.
`replace_text` updates only the buffer. `edit_range` replaces a specific character range. Both update the buffer and the undo history. `is_dirty` compares it with the text
from the last successful open/save. `save` writes the current bytes and updates
that snapshot only after success. `save_as` writes to a new path, updating the
document's path and snapshot on success. `reload` updates the buffer from disk,
replacing the saved snapshot and clearing dirty state. Restoring the saved text
clears dirty state. Invalid UTF-8 and filesystem failures return `std::io::Error`.

`request_close` returns `Cancelled` for dirty content and `Closed` for clean
content without modifying the document or disk. This is a decision API: callers
retain cancelled documents and may drop clean ones. There is no UI, discard
option, forced invalidation or automatic save.

Run from the repository root:

```sh
CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml --offline --locked
cargo fmt --manifest-path crates/editor-core/Cargo.toml --check
CARGO_BUILD_JOBS=1 cargo run --manifest-path crates/editor-core/Cargo.toml --example edit_roundtrip --offline --locked
```

The eight integration tests cover initial clean state, edit/cancel without disk
changes, save/reopen bytes, restoring the last saved text, Unicode and mixed
newlines, empty content, missing/invalid UTF-8 files, and deterministic failed
save plus retry. The failure fixture replaces its synthetic file with a
directory, so it does not depend on permissions or the executing user.

The example creates a new synthetic directory under the system temp
directory, exercises the edit/cancel/save/reopen sequence, and removes only its
own file and directory. Existing name collisions are skipped. Tests also use
new synthetic directories; no personal input files or network are needed.

## Manual recovery file safety

`write_backup(path)` creates a **new, caller-owned file** containing the current
UTF-8 buffer, calls `sync_data`, and closes it. It never truncates an existing
entry: regular files, symlinks (including dangling ones), hardlinks, and earlier
backups are rejected. Choose a fresh path for each snapshot. The current document
destination is rejected even when missing, comparing canonical parent directories
and the final filename to recognize lexical and directory aliases. If the document
entry is missing, **all backup paths in its canonical directory are rejected**:
portable filename comparison cannot rule out case or Unicode normalization aliases.
Use a separate existing directory for recovery in that case. A dangling symlink
at the document path causes backup writes to fail conservatively.

`restore_backup(path)` reads UTF-8 into the buffer and clears undo/redo history
only after a successful read. It leaves the persisted snapshot unchanged, so
restoring identical text remains clean. Reading a file does not transfer ownership:
**save, restore, reload, and drop never automatically delete recovery files**.
This intentionally replaces the unsafe earlier behavior that deleted any supplied
restore path after save, and that overwrote existing backup paths.

A caller must decide when and how to dispose of retained recovery copies. An
explicit `save_as(backup_path)` still means replace that destination with the
current buffer, using the ordinary save contract; the saved destination remains
present. Merely restoring a file does not authorize its deletion or overwrite.
The 20 tests in `tests/recovery_safety.rs` cover collisions, borrowed sources,
missing document destinations, aliases, replaced entries, and save-as-to-backup.

This is manual snapshot storage, not a recovery registry or automatic journal.
A failed backup write/sync may leave a partial newly created file; it is retained
for caller inspection instead of removing a possibly replaced pathname. There is
no guarantee against concurrent parent-directory renames or symlink retargeting
between path comparison and creation, and no directory-sync or crash-recovery
claim is added. Relative paths still require an unchanged working directory.

## Limits

- Saves use a temporary file and atomic rename. Failure preserves the in-memory
  buffer and saved snapshot, and leaves the original file unchanged. Saves are
  written with `sync_data` and the temporary file is closed before rename.
  Full crash durability and cross-platform file-lock behavior are not established;
  there is no directory synchronization or executed crash test.
  Permission and metadata preservation are not promised.
- Only valid UTF-8 is supported. Bytes within valid UTF-8, including newlines
  and a UTF-8 BOM if present, are not normalized. There is no encoding detection.
- There is no locking, conflict handling or symlink
  isolation. Saves will fail if concurrent external changes are detected, returning an `AlreadyExists` io error. Relative paths require
  the process working directory to stay unchanged.
- The whole text and saved snapshot are stored in memory; replacement and dirty
  comparison are whole-document operations. There is basic linear undo/redo,
  but no incremental edit, autosave, or large-file optimization. It provides manual methods for a recovery journal (`write_backup`, `restore_backup`).
- Interrupted processes can leave synthetic temp files; normal example cleanup
  is checked, and error cleanup is best effort.
- Passing these synthetic tests establishes neither VS Code parity nor a
  desktop launch, extension compatibility, security isolation, performance
  baseline or measured network behavior.
