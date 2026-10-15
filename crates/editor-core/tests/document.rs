use lean_editor_core::{CloseOutcome, Document};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct Fixture {
    directory: PathBuf,
    file: PathBuf,
}

impl Fixture {
    fn new(bytes: &[u8]) -> Self {
        let directory = loop {
            let candidate = std::env::temp_dir().join(format!(
                "lean-editor-core-test-{}-{}",
                std::process::id(),
                NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&candidate) {
                Ok(()) => break candidate,
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("create synthetic fixture: {error}"),
            }
        };
        let fixture = Self {
            file: directory.join("document.txt"),
            directory,
        };
        fs::write(&fixture.file, bytes).unwrap();
        fixture
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        // Only these two owned paths are removed; never recursively delete a tree.
        let _ = fs::remove_file(&self.file);
        let _ = fs::remove_dir(&self.file);
        let _ = fs::remove_dir(&self.directory);
    }
}

#[test]
fn open_preserves_path_and_text_and_starts_clean() {
    let fixture = Fixture::new(b"cedar\nline two\n");
    let document = Document::open(&fixture.file).unwrap();
    assert_eq!(document.path(), fixture.file);
    assert_eq!(document.text(), "cedar\nline two\n");
    assert!(!document.is_dirty());
    assert_eq!(document.request_close(), CloseOutcome::Closed);
}

#[test]
fn unsaved_edit_changes_only_the_buffer_and_cancel_preserves_it() {
    let fixture = Fixture::new(b"cedar\nline two\n");
    let mut document = Document::open(&fixture.file).unwrap();
    document.replace_text("maple\nline two\n");
    assert!(document.is_dirty());
    assert_eq!(fs::read(&fixture.file).unwrap(), b"cedar\nline two\n");
    assert_eq!(document.request_close(), CloseOutcome::Cancelled);
    assert_eq!(document.text(), "maple\nline two\n");
    assert!(document.is_dirty());
    assert_eq!(fs::read(&fixture.file).unwrap(), b"cedar\nline two\n");
}

#[test]
fn save_clears_dirty_and_reopen_observes_exact_saved_bytes() {
    let fixture = Fixture::new(b"cedar\nline two\n");
    let mut document = Document::open(&fixture.file).unwrap();
    document.replace_text("maple\nline two\n");
    document.save().unwrap();
    assert!(!document.is_dirty());
    assert_eq!(fs::read(&fixture.file).unwrap(), b"maple\nline two\n");
    assert_eq!(document.request_close(), CloseOutcome::Closed);
    drop(document);
    let reopened = Document::open(&fixture.file).unwrap();
    assert_eq!(reopened.text(), "maple\nline two\n");
    assert!(!reopened.is_dirty());
}

#[test]
fn restoring_the_last_saved_text_clears_dirty_without_writing() {
    let fixture = Fixture::new(b"original");
    let mut document = Document::open(&fixture.file).unwrap();
    document.replace_text("edit");
    document.replace_text("original");
    assert!(!document.is_dirty());
    document.replace_text("saved");
    document.save().unwrap();
    document.replace_text("original");
    assert!(document.is_dirty());
    document.replace_text("saved");
    assert!(!document.is_dirty());
    assert_eq!(fs::read(&fixture.file).unwrap(), b"saved");
}

#[test]
fn unicode_and_mixed_newlines_roundtrip_without_normalization() {
    let original = "café\r\n東京\n🦀\r\n";
    let edited = "naïve\r\n🌲\n終わり";
    let fixture = Fixture::new(original.as_bytes());
    let mut document = Document::open(&fixture.file).unwrap();
    assert_eq!(document.text().as_bytes(), original.as_bytes());
    document.replace_text(edited);
    document.save().unwrap();
    assert_eq!(fs::read(&fixture.file).unwrap(), edited.as_bytes());
    assert_eq!(Document::open(&fixture.file).unwrap().text(), edited);
}

#[test]
fn empty_content_can_be_saved_and_reopened() {
    let fixture = Fixture::new(b"content");
    let mut document = Document::open(&fixture.file).unwrap();
    document.replace_text("");
    assert!(document.is_dirty());
    document.save().unwrap();
    assert!(!document.is_dirty());
    assert!(fs::read(&fixture.file).unwrap().is_empty());
    assert_eq!(Document::open(&fixture.file).unwrap().text(), "");
}

#[test]
fn missing_file_and_invalid_utf8_return_errors() {
    let fixture = Fixture::new(&[0xff, 0xfe, 0x80]);
    let invalid = Document::open(&fixture.file).unwrap_err();
    assert_eq!(invalid.kind(), io::ErrorKind::InvalidData);
    assert_eq!(fs::read(&fixture.file).unwrap(), [0xff, 0xfe, 0x80]);
    let missing = Document::open(fixture.directory.join("missing.txt")).unwrap_err();
    assert_eq!(missing.kind(), io::ErrorKind::NotFound);
}

#[test]
fn failed_save_retains_buffer_dirty_state_and_last_saved_baseline() {
    let fixture = Fixture::new(b"original");
    let mut document = Document::open(&fixture.file).unwrap();
    document.replace_text("unsaved");
    // A directory cannot be written as a file, regardless of permissions/root.
    fs::remove_file(&fixture.file).unwrap();
    fs::create_dir(&fixture.file).unwrap();
    assert!(document.save().is_err());
    assert_eq!(document.text(), "unsaved");
    assert_eq!(document.path(), fixture.file);
    assert!(document.is_dirty());
    assert_eq!(document.request_close(), CloseOutcome::Cancelled);
    document.replace_text("original");
    assert!(!document.is_dirty());
    document.replace_text("retry");
    fs::remove_dir(&fixture.file).unwrap();
    document.save().unwrap();
    assert!(!document.is_dirty());
    assert_eq!(fs::read(&fixture.file).unwrap(), b"retry");
}

#[test]
fn atomic_save_failure_cleans_up_temp_file_and_leaves_target_intact() {
    let fixture = Fixture::new(b"original");
    let mut document = Document::open(&fixture.file).unwrap();
    document.replace_text("unsaved");

    // Simulate rename failure by replacing target with a directory
    fs::remove_file(&fixture.file).unwrap();
    fs::create_dir(&fixture.file).unwrap();

    let result = document.save();
    assert!(result.is_err());

    // The directory we put there should still be intact
    assert!(fixture.file.is_dir());

    // There should be no temporary files left in the directory
    let entries = fs::read_dir(&fixture.directory).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        assert_eq!(
            entry.path(),
            fixture.file,
            "Expected only the target file/dir to exist, found {:?}",
            entry.path()
        );
    }
}

#[test]
fn external_change_detection() {
    let fixture = Fixture::new(b"original");
    let mut document = Document::open(&fixture.file).unwrap();

    // Initially no changes
    assert!(!document.has_external_changes().unwrap());

    // Modify the file externally
    // we need to set the modified time manually sometimes because tests are fast
    // and mtime resolution is low on some OSes
    let time = std::time::SystemTime::now() + std::time::Duration::from_secs(2);
    filetime::set_file_mtime(&fixture.file, filetime::FileTime::from_system_time(time)).unwrap();

    // Now it should detect changes
    assert!(document.has_external_changes().unwrap());

    // Save document
    document.replace_text("saved");
    document.save().unwrap();

    // Should no longer have external changes
    assert!(!document.has_external_changes().unwrap());

    // External delete should result in an error
    fs::remove_file(&fixture.file).unwrap();
    assert!(document.has_external_changes().is_err());
}

#[test]
fn reload_updates_buffer_and_clears_dirty_from_disk() {
    let fixture = Fixture::new(b"original");
    let mut document = Document::open(&fixture.file).unwrap();
    document.replace_text("unsaved");
    assert!(document.is_dirty());

    // Modify file externally
    fs::write(&fixture.file, b"external change").unwrap();

    // Reload
    document.reload().unwrap();
    assert!(!document.is_dirty());
    assert_eq!(document.text(), "external change");

    // A second reload should also succeed and stay clean
    document.reload().unwrap();
    assert!(!document.is_dirty());
    assert_eq!(document.text(), "external change");
}

#[test]
fn failed_reload_leaves_document_intact() {
    let fixture = Fixture::new(b"original");
    let mut document = Document::open(&fixture.file).unwrap();
    document.replace_text("unsaved");

    // Attempt to reload missing file
    fs::remove_file(&fixture.file).unwrap();
    let err = document.reload().unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::NotFound);

    // Document should be intact
    assert!(document.is_dirty());
    assert_eq!(document.text(), "unsaved");

    // Attempt to reload invalid UTF-8
    fs::write(&fixture.file, &[0xff, 0xfe, 0x80]).unwrap();
    let err = document.reload().unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::InvalidData);

    // Document should be intact
    assert!(document.is_dirty());
    assert_eq!(document.text(), "unsaved");
}

#[test]
fn undo_and_redo_restore_text() {
    let fixture = Fixture::new(b"original");
    let mut document = Document::open(&fixture.file).unwrap();

    document.replace_text("edit 1");
    document.replace_text("edit 2");
    assert_eq!(document.text(), "edit 2");
    assert!(document.is_dirty());

    assert!(document.undo());
    assert_eq!(document.text(), "edit 1");
    assert!(document.is_dirty());

    assert!(document.undo());
    assert_eq!(document.text(), "original");
    assert!(!document.is_dirty());

    assert!(!document.undo()); // empty history
    assert_eq!(document.text(), "original");

    assert!(document.redo());
    assert_eq!(document.text(), "edit 1");
    assert!(document.is_dirty());

    assert!(document.redo());
    assert_eq!(document.text(), "edit 2");

    assert!(!document.redo()); // empty redo history
}

#[test]
fn new_edit_clears_redo_history() {
    let fixture = Fixture::new(b"original");
    let mut document = Document::open(&fixture.file).unwrap();

    document.replace_text("edit 1");
    document.replace_text("edit 2");
    document.undo();
    assert_eq!(document.text(), "edit 1");

    document.replace_text("edit 3");
    assert!(!document.redo()); // redo is cleared
    assert_eq!(document.text(), "edit 3");

    document.undo();
    assert_eq!(document.text(), "edit 1");
}

#[test]
fn reload_clears_undo_and_redo_history() {
    let fixture = Fixture::new(b"original");
    let mut document = Document::open(&fixture.file).unwrap();

    document.replace_text("edit 1");
    document.undo();
    document.redo();

    document.reload().unwrap();
    assert!(!document.undo());
    assert!(!document.redo());
}

#[test]
fn save_as_updates_path_and_clears_dirty_state() {
    let fixture = Fixture::new(b"original");
    let mut document = Document::open(&fixture.file).unwrap();
    document.replace_text("unsaved");
    assert!(document.is_dirty());

    let new_path = fixture.directory.join("new_file.txt");
    document.save_as(&new_path).unwrap();

    assert!(!document.is_dirty());
    assert_eq!(document.path(), new_path);
    assert_eq!(fs::read(&new_path).unwrap(), b"unsaved");
    // Original file remains unchanged
    assert_eq!(fs::read(&fixture.file).unwrap(), b"original");
}

#[test]
fn save_as_failure_retains_original_path_and_dirty_state() {
    let fixture = Fixture::new(b"original");
    let mut document = Document::open(&fixture.file).unwrap();
    document.replace_text("unsaved");

    let new_path = fixture.directory.join("new_dir");
    fs::create_dir(&new_path).unwrap(); // Target is a directory, rename will fail

    assert!(document.save_as(&new_path).is_err());

    assert!(document.is_dirty());
    assert_eq!(document.path(), fixture.file);
    assert_eq!(document.text(), "unsaved");
    // Original file remains unchanged
    assert_eq!(fs::read(&fixture.file).unwrap(), b"original");
}

#[test]
fn edit_range_replaces_substring() {
    let fixture = Fixture::new(b"0123456789");
    let mut document = Document::open(&fixture.file).unwrap();
    document.edit_range(2, 5, "abc");
    assert_eq!(document.text(), "01abc56789");
    assert!(document.is_dirty());
}

#[test]
fn edit_range_inserts_text() {
    let fixture = Fixture::new(b"0123456789");
    let mut document = Document::open(&fixture.file).unwrap();
    document.edit_range(5, 5, "abc");
    assert_eq!(document.text(), "01234abc56789");
    assert!(document.is_dirty());
}

#[test]
fn edit_range_deletes_text() {
    let fixture = Fixture::new(b"0123456789");
    let mut document = Document::open(&fixture.file).unwrap();
    document.edit_range(2, 5, "");
    assert_eq!(document.text(), "0156789");
    assert!(document.is_dirty());
}

#[test]
#[should_panic(expected = "start_char must be <= end_char")]
fn edit_range_panics_if_start_greater_than_end() {
    let fixture = Fixture::new(b"0123456789");
    let mut document = Document::open(&fixture.file).unwrap();
    document.edit_range(5, 2, "abc");
}

#[test]
#[should_panic(expected = "end_char out of bounds")]
fn edit_range_panics_if_out_of_bounds() {
    let fixture = Fixture::new(b"0123456789");
    let mut document = Document::open(&fixture.file).unwrap();
    document.edit_range(0, 11, "abc");
}

#[test]
fn edit_range_integrates_with_undo_redo() {
    let fixture = Fixture::new(b"0123456789");
    let mut document = Document::open(&fixture.file).unwrap();

    document.edit_range(2, 5, "abc");
    assert_eq!(document.text(), "01abc56789");

    assert!(document.undo());
    assert_eq!(document.text(), "0123456789");

    assert!(document.redo());
    assert_eq!(document.text(), "01abc56789");
}
