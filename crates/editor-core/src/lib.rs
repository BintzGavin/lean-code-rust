//! A small UTF-8 document buffer with explicit save and close decisions.
//!
//! This crate uses a temporary file and atomic rename for saves.
//! See the crate README for the contract and its limits.

use std::fs::{self, File};
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
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    /// Replace only the buffer. No filesystem operations occur here.
    pub fn replace_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
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
    /// remain unchanged, and the temporary file is deleted. The data is
    /// synchronized to stable storage before rename to ensure crash durability.
    /// Permission and metadata preservation are not promised.
    pub fn save(&mut self) -> io::Result<()> {
        static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);
        let parent = self.path.parent().unwrap_or_else(|| Path::new(""));
        let file_name = self.path.file_name().unwrap_or_default();

        let temp_name = format!(
            ".{}.{}-{}.tmp",
            file_name.to_string_lossy(),
            std::process::id(),
            NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed)
        );
        let temp_path = parent.join(temp_name);

        let mut file = match File::create(&temp_path) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

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

        if let Err(e) = fs::rename(&temp_path, &self.path) {
            let _ = fs::remove_file(&temp_path);
            return Err(e);
        }

        self.persisted_text.clone_from(&self.text);
        self.modified_time = fs::metadata(&self.path).and_then(|m| m.modified()).ok();
        Ok(())
    }

    /// Cancel a dirty close, or allow a clean close, without changing any state.
    ///
    /// This is a decision API, not a UI or lifetime manager. The caller retains
    /// the document on `Cancelled` and may drop it on `Closed`. To close edited
    /// content, save successfully and request close again.

    /// Check if the file on disk has been modified since it was last opened or saved.
    pub fn has_external_changes(&self) -> io::Result<bool> {
        let current_modified = fs::metadata(&self.path)?.modified()?;
        Ok(self
            .modified_time
            .map_or(false, |time| current_modified != time))
    }

    /// Refresh the document buffer from the file on disk.
    ///
    /// If the file does not exist or contains invalid UTF-8, it returns an error
    /// and leaves the current document buffer and saved snapshot unchanged.
    pub fn reload(&mut self) -> io::Result<()> {
        let text = fs::read_to_string(&self.path)?;
        let modified_time = fs::metadata(&self.path)?.modified().ok();

        self.text = text.clone();
        self.persisted_text = text;
        self.modified_time = modified_time;

        Ok(())
    }

    pub fn request_close(&self) -> CloseOutcome {
        if self.is_dirty() {
            CloseOutcome::Cancelled
        } else {
            CloseOutcome::Closed
        }
    }
}
