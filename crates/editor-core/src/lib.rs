//! A small UTF-8 document buffer with explicit save and close decisions.
//!
//! This crate uses a temporary file and atomic rename for saves.
//! See the crate README for the contract and its limits.

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

/// The caller may release a clean document, or retain a cancelled dirty one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseOutcome {
    Closed,
    Cancelled,
}

/// An in-memory UTF-8 document and its last successful persisted text.
#[derive(Debug)]
pub struct Document {
    path: PathBuf,
    text: String,
    persisted_text: String,
    modified_time: Option<SystemTime>,
    undo_stack: Vec<String>,
    redo_stack: Vec<String>,
}

impl Document {
    /// Read UTF-8 content without changing the file or normalizing newlines.
    ///
    /// The supplied path is retained as-is. A caller using a relative path must
    /// keep the working directory unchanged while using the document.
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let text = fs::read_to_string(&path)?;
        let modified_time = fs::metadata(&path)?.modified().ok();
        Ok(Self {
            path,
            persisted_text: text.clone(),
            text,
            modified_time,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        })
    }

    /// Write the current document state to a new, caller-owned backup file.
    ///
    /// Existing entries (including symlinks and earlier backups) are rejected.
    /// The current document destination is also rejected when missing, using
    /// canonical parent directories to recognize lexical and directory aliases.
    /// If the document is missing, all backups in its directory are rejected:
    /// portable filename comparison cannot rule out case/Unicode aliases.
    /// No backup is automatically removed by save, restore, reload, or drop.
    /// Failed writes may leave a partial new backup for the caller to inspect.
    pub fn write_backup(&mut self, backup_path: impl AsRef<Path>) -> io::Result<()> {
        let backup_path = backup_path.as_ref();
        let backup_location = destination_location(backup_path)?;
        let document_location = destination_location(&self.path)?;
        // An existing final symlink is rejected by create_new. Resolve the
        // document's final symlink too: a dangling document link fails closed.
        let document_target = match fs::symlink_metadata(&self.path) {
            Ok(metadata) if metadata.file_type().is_symlink() => fs::canonicalize(&self.path)?,
            Ok(_) => document_location.clone(),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                // Without an existing entry, canonicalize cannot establish
                // case/Unicode filename equivalence on this filesystem. Keep
                // recovery possible in a different existing directory only.
                if backup_location.parent() == document_location.parent() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "A missing document requires a backup in another directory",
                    ));
                }
                document_location.clone()
            }
            Err(error) => return Err(error),
        };
        if backup_location == document_location || backup_location == document_target {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "A backup must use a separate document destination",
            ));
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(backup_path)?;
        file.write_all(self.text.as_bytes())?;
        file.sync_data()?;
        drop(file);
        Ok(())
    }

    /// Read a UTF-8 recovery source into the buffer, without taking ownership.
    ///
    /// The source is never removed automatically, even if this document wrote
    /// it earlier. Dirty state still compares against the last open/save text.
    pub fn restore_backup(&mut self, backup_path: impl AsRef<Path>) -> io::Result<()> {
        let backup_path_ref = backup_path.as_ref();
        let text = fs::read_to_string(backup_path_ref)?;
        self.text = text;
        // Keep the persisted snapshot; equal restored text remains clean.
        self.undo_stack.clear();
        self.redo_stack.clear();
        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    /// Replace the text between `start_char` and `end_char` (0-indexed character offsets) with `new_text`.
    /// Panics if the indices are out of bounds or if `start_char > end_char`.
    pub fn edit_range(&mut self, start_char: usize, end_char: usize, new_text: &str) {
        assert!(start_char <= end_char, "start_char must be <= end_char");

        let char_count = self.text.chars().count();
        assert!(end_char <= char_count, "end_char out of bounds");

        let byte_start = self
            .text
            .char_indices()
            .nth(start_char)
            .map(|(i, _)| i)
            .unwrap_or(self.text.len());
        let byte_end = self
            .text
            .char_indices()
            .nth(end_char)
            .map(|(i, _)| i)
            .unwrap_or(self.text.len());

        let mut next_text = String::with_capacity(self.text.len() + new_text.len());
        next_text.push_str(&self.text[..byte_start]);
        next_text.push_str(new_text);
        next_text.push_str(&self.text[byte_end..]);

        if self.text != next_text {
            self.undo_stack.push(self.text.clone());
            self.redo_stack.clear();
            self.text = next_text;
        }
    }

    /// Replace only the buffer. No filesystem operations occur here.
    pub fn replace_text(&mut self, text: impl Into<String>) {
        let new_text = text.into();
        if self.text != new_text {
            self.undo_stack.push(self.text.clone());
            self.redo_stack.clear();
            self.text = new_text;
        }
    }

    /// Compare the buffer with the last successful open/save snapshot.
    /// External file changes are not observed by this comparison.
    pub fn is_dirty(&self) -> bool {
        self.text != self.persisted_text
    }

    /// Write current UTF-8 bytes and update the saved snapshot only on success.
    ///
    /// Saves are performed by writing to a temporary file in the same directory
    /// and then atomically renaming it to the target path. On failure before or
    /// during the rename, the original target file and the document buffer
    /// remain unchanged, and any temporary file created by this save is deleted.
    /// An existing temporary path causes an error without changing that entry.
    /// The temporary file receives `sync_data` and is closed before rename.
    /// Directory synchronization and full crash durability are not established.
    /// Permission and metadata preservation are not promised.
    pub fn save(&mut self) -> io::Result<()> {
        let path = self.path.clone();
        self.save_as(&path)
    }

    /// Write current UTF-8 bytes to a new path and update the document's internal path
    /// on success. The dirty state is cleared, and the internal saved snapshot is updated.
    ///
    /// Saves are performed by writing to a temporary file in the target directory
    /// and then atomically renaming it to the target path. On failure before or
    /// during the rename, the original target file and the document buffer
    /// remain unchanged, and any temporary file created by this save is deleted.
    /// An existing temporary path causes an error without changing that entry.
    /// The temporary file receives `sync_data` and is closed before rename.
    /// Directory synchronization and full crash durability are not established.
    #[doc(hidden)]
    pub fn force_save_for_testing(&mut self) -> io::Result<()> {
        let path = self.path.clone();
        self.force_save_as_for_testing(&path)
    }

    #[doc(hidden)]
    pub fn force_save_as_for_testing(&mut self, new_path: impl AsRef<Path>) -> io::Result<()> {
        self.save_as_internal(new_path, false)
    }

    pub fn save_as(&mut self, new_path: impl AsRef<Path>) -> io::Result<()> {
        self.save_as_internal(new_path, true)
    }

    fn save_as_internal(
        &mut self,
        new_path: impl AsRef<Path>,
        check_external: bool,
    ) -> io::Result<()> {
        let new_path_ref = new_path.as_ref();
        let is_same_path = self.path == new_path_ref;
        if check_external && is_same_path && self.has_external_changes().unwrap_or(true) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "File has been modified externally since it was opened",
            ));
        }

        let new_path = new_path.as_ref().to_path_buf();
        static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);
        let parent = new_path.parent().unwrap_or_else(|| Path::new(""));
        let file_name = new_path.file_name().unwrap_or_default();

        let temp_name = format!(
            ".{}.{}-{}.tmp",
            file_name.to_string_lossy(),
            std::process::id(),
            NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed)
        );
        let temp_path = parent.join(temp_name);

        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)?;

        if let Err(e) = file.write_all(self.text.as_bytes()) {
            drop(file);
            let _ = fs::remove_file(&temp_path);
            return Err(e);
        }

        if let Err(e) = file.sync_data() {
            drop(file);
            let _ = fs::remove_file(&temp_path);
            return Err(e);
        }

        drop(file);

        if let Err(e) = fs::rename(&temp_path, &new_path) {
            let _ = fs::remove_file(&temp_path);
            return Err(e);
        }

        self.path = new_path.clone();
        self.persisted_text.clone_from(&self.text);
        self.modified_time = fs::metadata(&self.path).and_then(|m| m.modified()).ok();

        Ok(())
    }

    /// Cancel a dirty close, or allow a clean close, without changing any state.
    ///
    /// This is a decision API, not a UI or lifetime manager. The caller retains
    /// the document on `Cancelled` and may drop it on `Closed`. To close edited
    /// content, save successfully and request close again.
    ///
    /// Check if the file on disk has been modified since it was last opened or saved.
    pub fn has_external_changes(&self) -> io::Result<bool> {
        let current_modified = fs::metadata(&self.path)?.modified()?;
        Ok(self
            .modified_time
            .is_some_and(|time| current_modified != time))
    }

    /// Reload the document from disk, updating the text, persisted text, and modified time.
    /// Returns an error if the file does not exist or contains invalid UTF-8, leaving the
    /// document unchanged.
    pub fn reload(&mut self) -> io::Result<()> {
        let text = fs::read_to_string(&self.path)?;
        let modified_time = fs::metadata(&self.path)?.modified().ok();
        self.text.clone_from(&text);
        self.persisted_text = text;
        self.modified_time = modified_time;
        self.undo_stack.clear();
        self.redo_stack.clear();
        Ok(())
    }

    pub fn undo(&mut self) -> bool {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.text.clone());
            self.text = prev;
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.text.clone());
            self.text = next;
            true
        } else {
            false
        }
    }

    pub fn request_close(&self) -> CloseOutcome {
        if self.is_dirty() {
            CloseOutcome::Cancelled
        } else {
            CloseOutcome::Closed
        }
    }
}

// Resolve the directory independently so a missing final entry can still be
// compared with the document destination. This does not lock directories
// against concurrent renames or symlink retargeting.
fn destination_location(path: &Path) -> io::Result<PathBuf> {
    let name = path.file_name().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "Expected a file destination")
    })?;
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    Ok(fs::canonicalize(parent)?.join(name))
}
