//! A small UTF-8 document buffer with explicit save and close decisions.
//!
//! This crate uses atomic filesystem writes (write to a temporary file, then rename).
//! See the crate README for the contract and its limits.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static SAVE_COUNTER: AtomicUsize = AtomicUsize::new(0);

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
}

impl Document {
    /// Read UTF-8 content without changing the file or normalizing newlines.
    ///
    /// The supplied path is retained as-is. A caller using a relative path must
    /// keep the working directory unchanged while using the document.
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let text = fs::read_to_string(&path)?;
        Ok(Self {
            path,
            persisted_text: text.clone(),
            text,
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

    /// Write current UTF-8 bytes to a temporary file and atomically rename it.
    ///
    /// On failure the buffer and saved snapshot remain unchanged, and the original file
    /// remains unmodified. This provides atomic saves, though it does not guarantee
    /// crash durability (e.g. via fsync) or preserve metadata.
    pub fn save(&mut self) -> io::Result<()> {
        let parent = self.path.parent().unwrap_or_else(|| Path::new(""));
        let mut temp_name = self.path.file_name().unwrap_or_default().to_os_string();
        let counter = SAVE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros();
        temp_name.push(format!(".~tmp.{}.{}.{}", std::process::id(), counter, time));
        let temp_path = parent.join(temp_name);

        use std::io::Write;

        let mut file = match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)
        {
            Ok(f) => f,
            Err(_) => {
                // Fallback to in-place write if we can't create the temp file (e.g., read-only dir)
                fs::write(&self.path, self.text.as_bytes())?;
                self.persisted_text.clone_from(&self.text);
                return Ok(());
            }
        };

        if let Err(e) = file.write_all(self.text.as_bytes()) {
            drop(file);
            let _ = fs::remove_file(&temp_path);
            return Err(e);
        }

        // Ensure file is closed before renaming (important on Windows)
        drop(file);

        if let Err(e) = fs::rename(&temp_path, &self.path) {
            let _ = fs::remove_file(&temp_path);
            return Err(e);
        }

        self.persisted_text.clone_from(&self.text);
        Ok(())
    }

    /// Cancel a dirty close, or allow a clean close, without changing any state.
    ///
    /// This is a decision API, not a UI or lifetime manager. The caller retains
    /// the document on `Cancelled` and may drop it on `Closed`. To close edited
    /// content, save successfully and request close again.
    pub fn request_close(&self) -> CloseOutcome {
        if self.is_dirty() {
            CloseOutcome::Cancelled
        } else {
            CloseOutcome::Closed
        }
    }
}
