use lean_editor_core::Document;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        loop {
            let root = std::env::temp_dir().join(format!(
                "lean-editor-recovery-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&root) {
                Ok(()) => {
                    fs::write(root.join("document"), b"original").unwrap();
                    fs::create_dir(root.join("subdir")).unwrap();
                    return Self(root);
                }
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("create synthetic fixture: {e}"),
            }
        }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }

    fn document(&self) -> Document {
        Document::open(self.path("document")).unwrap()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        for name in [
            "document",
            "backup",
            "other",
            "alias",
            "parent_link",
            "subdir/backup",
            "café",
            "cafe\u{301}",
            "DOCUMENT",
        ] {
            let _ = fs::remove_file(self.path(name));
        }
        let _ = fs::remove_dir(self.path("subdir"));
        let _ = fs::remove_dir(&self.0);
    }
}

fn assert_write_rejected(document: &mut Document, path: &Path) {
    let original_path = document.path().to_owned();
    document.replace_text("first edit");
    document.replace_text("second edit");
    assert!(document.undo());
    assert!(document.write_backup(path).is_err());
    assert_eq!(document.path(), original_path);
    assert_eq!(document.text(), "first edit");
    assert!(document.is_dirty());
    assert!(document.redo());
    assert_eq!(document.text(), "second edit");
    assert!(document.undo());
    assert!(document.undo());
    assert_eq!(document.text(), "original");
    assert!(!document.is_dirty());
}

#[test]
fn backup_rejects_existing_unrelated_file_without_changing_state() {
    let f = Fixture::new();
    fs::write(f.path("other"), b"unrelated bytes").unwrap();
    assert_write_rejected(&mut f.document(), &f.path("other"));
    assert_eq!(fs::read(f.path("other")).unwrap(), b"unrelated bytes");
    assert_eq!(fs::read(f.path("document")).unwrap(), b"original");
}

#[test]
fn backup_rejects_same_lexical_and_canonical_document_paths() {
    let f = Fixture::new();
    for path in [
        f.path("document"),
        f.path("subdir/../document"),
        fs::canonicalize(f.path("document")).unwrap(),
    ] {
        assert_write_rejected(&mut f.document(), &path);
        assert_eq!(fs::read(f.path("document")).unwrap(), b"original");
    }
}

#[test]
fn backup_rejects_missing_document_destination_and_lexical_alias() {
    for lexical in [false, true] {
        let f = Fixture::new();
        let mut doc = f.document();
        fs::remove_file(f.path("document")).unwrap();
        let path = f.path(if lexical {
            "subdir/../document"
        } else {
            "document"
        });
        assert_write_rejected(&mut doc, &path);
        assert!(!f.path("document").exists());
    }
}

#[test]
fn backup_can_preserve_edits_in_another_directory_when_document_was_removed() {
    let f = Fixture::new();
    let mut doc = f.document();
    fs::remove_file(f.path("document")).unwrap();
    doc.replace_text("recover me");
    doc.write_backup(f.path("subdir/backup")).unwrap();
    assert_eq!(fs::read(f.path("subdir/backup")).unwrap(), b"recover me");
    assert!(!f.path("document").exists());
}

#[test]
fn repeated_backup_write_does_not_replace_previous_recovery_copy() {
    let f = Fixture::new();
    let mut doc = f.document();
    doc.replace_text("first recovery copy");
    doc.write_backup(f.path("backup")).unwrap();
    doc.replace_text("later edits");
    assert_eq!(
        doc.write_backup(f.path("backup")).unwrap_err().kind(),
        io::ErrorKind::AlreadyExists
    );
    assert_eq!(fs::read(f.path("backup")).unwrap(), b"first recovery copy");
    assert_eq!(doc.text(), "later edits");
    assert!(doc.is_dirty());
}

#[test]
fn restore_current_document_then_save_keeps_document_even_without_edits() {
    let f = Fixture::new();
    for path in [
        f.path("document"),
        f.path("subdir/../document"),
        fs::canonicalize(f.path("document")).unwrap(),
    ] {
        let mut doc = f.document();
        doc.restore_backup(path).unwrap();
        assert!(!doc.is_dirty());
        doc.save().unwrap();
        assert_eq!(fs::read(f.path("document")).unwrap(), b"original");
    }
}

#[test]
fn restored_unrelated_source_is_retained_after_save() {
    let f = Fixture::new();
    fs::write(f.path("other"), b"restored content").unwrap();
    let mut doc = f.document();
    doc.restore_backup(f.path("other")).unwrap();
    assert!(doc.is_dirty());
    doc.save().unwrap();
    assert!(!doc.is_dirty());
    assert_eq!(fs::read(f.path("document")).unwrap(), b"restored content");
    assert_eq!(fs::read(f.path("other")).unwrap(), b"restored content");
}

#[test]
fn written_backup_is_retained_after_save() {
    let f = Fixture::new();
    let mut doc = f.document();
    doc.replace_text("recovery copy");
    doc.write_backup(f.path("backup")).unwrap();
    doc.replace_text("saved later");
    doc.save().unwrap();
    assert_eq!(fs::read(f.path("backup")).unwrap(), b"recovery copy");
    assert_eq!(fs::read(f.path("document")).unwrap(), b"saved later");
}

#[test]
fn replaced_backup_entry_is_not_deleted_after_save() {
    let f = Fixture::new();
    let mut doc = f.document();
    doc.write_backup(f.path("backup")).unwrap();
    fs::remove_file(f.path("backup")).unwrap();
    fs::write(f.path("backup"), b"new unrelated file").unwrap();
    doc.save().unwrap();
    assert_eq!(fs::read(f.path("backup")).unwrap(), b"new unrelated file");
}

#[test]
fn explicit_save_as_to_written_or_restored_backup_keeps_saved_destination() {
    for restore in [false, true] {
        let f = Fixture::new();
        let mut doc = f.document();
        doc.write_backup(f.path("backup")).unwrap();
        if restore {
            doc.restore_backup(f.path("backup")).unwrap();
        }
        doc.replace_text("explicitly saved bytes");
        doc.save_as(f.path("backup")).unwrap();
        assert_eq!(doc.path(), f.path("backup"));
        assert!(!doc.is_dirty());
        assert_eq!(
            fs::read(f.path("backup")).unwrap(),
            b"explicitly saved bytes"
        );
        assert_eq!(fs::read(f.path("document")).unwrap(), b"original");
    }
}

#[test]
fn failed_restore_preserves_buffer_dirty_state_and_history() {
    let f = Fixture::new();
    fs::write(f.path("other"), [0xff]).unwrap();
    let mut doc = f.document();
    doc.replace_text("edits");
    assert!(doc.restore_backup(f.path("other")).is_err());
    assert!(doc.restore_backup(f.path("backup")).is_err());
    assert_eq!(doc.text(), "edits");
    assert!(doc.is_dirty());
    assert!(doc.undo());
    assert_eq!(doc.text(), "original");
    assert_eq!(fs::read(f.path("other")).unwrap(), [0xff]);
}

#[cfg(unix)]
mod unix {
    use super::*;
    use std::os::unix::fs::symlink;

    #[test]
    fn backup_rejects_symlink_and_hardlink_entries() {
        for hard in [false, true] {
            for target in ["document", "other"] {
                let f = Fixture::new();
                fs::write(f.path("other"), b"unrelated bytes").unwrap();
                if hard {
                    fs::hard_link(f.path(target), f.path("alias")).unwrap();
                } else {
                    symlink(f.path(target), f.path("alias")).unwrap();
                }
                assert_write_rejected(&mut f.document(), &f.path("alias"));
                assert_eq!(fs::read(f.path("document")).unwrap(), b"original");
                assert_eq!(fs::read(f.path("other")).unwrap(), b"unrelated bytes");
                if !hard {
                    assert_eq!(fs::read_link(f.path("alias")).unwrap(), f.path(target));
                }
            }
        }
    }

    #[test]
    fn backup_rejects_dangling_symlink_without_creating_target() {
        let f = Fixture::new();
        symlink(f.path("other"), f.path("alias")).unwrap();
        assert_write_rejected(&mut f.document(), &f.path("alias"));
        assert!(!f.path("other").exists());
        assert_eq!(fs::read_link(f.path("alias")).unwrap(), f.path("other"));
    }

    #[test]
    fn backup_rejects_missing_document_through_symlinked_parent() {
        let f = Fixture::new();
        symlink(&f.0, f.path("parent_link")).unwrap();
        let mut doc = f.document();
        fs::remove_file(f.path("document")).unwrap();
        assert_write_rejected(&mut doc, &f.path("parent_link/document"));
        assert!(!f.path("document").exists());
    }

    #[test]
    fn restore_document_alias_then_save_preserves_alias_entry() {
        for hard in [false, true] {
            let f = Fixture::new();
            if hard {
                fs::hard_link(f.path("document"), f.path("alias")).unwrap();
            } else {
                symlink(f.path("document"), f.path("alias")).unwrap();
            }
            let mut doc = f.document();
            doc.restore_backup(f.path("alias")).unwrap();
            doc.save().unwrap();
            assert_eq!(fs::read(f.path("document")).unwrap(), b"original");
            assert_eq!(fs::read(f.path("alias")).unwrap(), b"original");
            if !hard {
                assert_eq!(fs::read_link(f.path("alias")).unwrap(), f.path("document"));
            }
        }
    }

    #[test]
    fn backup_path_retargeted_to_document_is_not_deleted_after_save() {
        let f = Fixture::new();
        let mut doc = f.document();
        doc.write_backup(f.path("backup")).unwrap();
        fs::remove_file(f.path("backup")).unwrap();
        symlink(f.path("document"), f.path("backup")).unwrap();
        doc.save().unwrap();
        assert_eq!(fs::read_link(f.path("backup")).unwrap(), f.path("document"));
        assert_eq!(fs::read(f.path("document")).unwrap(), b"original");
    }
}

#[test]
fn missing_document_rejects_same_directory_backup_when_names_may_alias() {
    let f = Fixture::new();
    let mut doc = f.document();
    fs::remove_file(f.path("document")).unwrap();
    assert_write_rejected(&mut doc, &f.path("backup"));
    assert!(!f.path("backup").exists());
}

#[test]
fn missing_document_rejects_case_alias_on_case_insensitive_filesystems() {
    let f = Fixture::new();
    if fs::read(f.path("DOCUMENT")).ok().as_deref() != Some(b"original") {
        return; // Filesystem does not provide this filename alias.
    }
    let mut doc = f.document();
    fs::remove_file(f.path("document")).unwrap();
    assert_write_rejected(&mut doc, &f.path("DOCUMENT"));
    assert!(!f.path("document").exists());
}

#[test]
fn missing_document_rejects_unicode_alias_on_normalizing_filesystems() {
    let f = Fixture::new();
    let composed = f.path("café");
    let decomposed = f.path("cafe\u{301}");
    fs::write(&composed, b"original").unwrap();
    if fs::read(&decomposed).ok().as_deref() != Some(b"original") {
        return; // Filesystem does not provide this filename alias.
    }
    let mut doc = Document::open(&composed).unwrap();
    fs::remove_file(&composed).unwrap();
    assert_write_rejected(&mut doc, &decomposed);
    assert!(!composed.exists());
}

#[test]
fn recovery_written_by_another_document_is_borrowed_and_retained() {
    let f = Fixture::new();
    let mut writer = f.document();
    writer.replace_text("recovered edits");
    writer.write_backup(f.path("backup")).unwrap();
    drop(writer);
    let mut restored = f.document();
    restored.restore_backup(f.path("backup")).unwrap();
    assert!(restored.is_dirty());
    restored.save().unwrap();
    assert!(!restored.is_dirty());
    assert_eq!(fs::read(f.path("document")).unwrap(), b"recovered edits");
    assert_eq!(fs::read(f.path("backup")).unwrap(), b"recovered edits");
}
