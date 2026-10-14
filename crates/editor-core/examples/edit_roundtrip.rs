use lean_editor_core::{CloseOutcome, Document};
use std::fs;
use std::io;
use std::path::PathBuf;

struct SyntheticFile {
    directory: PathBuf,
    file: PathBuf,
}

impl SyntheticFile {
    fn create() -> io::Result<Self> {
        for suffix in 0_u64.. {
            let directory = std::env::temp_dir().join(format!(
                "lean-editor-core-example-{}-{suffix}",
                std::process::id()
            ));
            match fs::create_dir(&directory) {
                Ok(()) => {
                    let fixture = Self {
                        file: directory.join("document.txt"),
                        directory,
                    };
                    fs::write(&fixture.file, "cedar\nline two\n")?;
                    return Ok(fixture);
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error),
            }
        }
        unreachable!("temporary directory counter exhausted")
    }

    fn cleanup(&self) -> io::Result<()> {
        fs::remove_file(&self.file)?;
        fs::remove_dir(&self.directory)
    }
}

impl Drop for SyntheticFile {
    fn drop(&mut self) {
        // Best-effort cleanup on errors; only remove our exact file/directory.
        let _ = fs::remove_file(&self.file);
        let _ = fs::remove_dir(&self.directory);
    }
}

fn main() -> io::Result<()> {
    let fixture = SyntheticFile::create()?;
    let mut document = Document::open(&fixture.file)?;
    assert!(!document.is_dirty());
    document.replace_text("maple\nline two\n");
    assert!(document.is_dirty());
    assert_eq!(fs::read(&fixture.file)?, b"cedar\nline two\n");
    assert_eq!(document.request_close(), CloseOutcome::Cancelled);
    assert_eq!(document.text(), "maple\nline two\n");
    assert_eq!(fs::read(&fixture.file)?, b"cedar\nline two\n");
    document.save()?;
    assert!(!document.is_dirty());
    assert_eq!(document.request_close(), CloseOutcome::Closed);
    drop(document);
    let reopened = Document::open(&fixture.file)?;
    assert_eq!(reopened.text(), "maple\nline two\n");
    assert_eq!(fs::read(&fixture.file)?, b"maple\nline two\n");
    fixture.cleanup()?;
    println!("PASS: open → edit → cancel → save → clean close → reopen; synthetic file removed");
    Ok(())
}
