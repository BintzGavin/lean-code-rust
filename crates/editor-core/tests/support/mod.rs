use lean_editor_core::{CloseOutcome, Document};
use std::fs::{self, File, FileTimes};
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

struct Fixture {
    directory: PathBuf,
    original: PathBuf,
    collision: PathBuf,
    unrelated: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let mut attempt = 0;
        let directory = loop {
            let candidate = Path::new("/tmp").join(format!(
                "lean-editor-core-collision-{}-{attempt}",
                std::process::id()
            ));
            match fs::create_dir(&candidate) {
                Ok(()) => break candidate,
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => attempt += 1,
                Err(error) => panic!("create synthetic fixture: {error}"),
            }
        };
        Self {
            original: directory.join("document.txt"),
            // Each integration-test binary performs exactly one save, so its
            // first temporary path is deterministic even with parallel tests.
            collision: directory.join(format!(".document.txt.{}-0.tmp", std::process::id())),
            unrelated: directory.join("unrelated.txt"),
            directory,
        }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        for path in [&self.original, &self.collision, &self.unrelated] {
            let _ = fs::remove_file(path);
        }
        let _ = fs::remove_dir(&self.directory);
    }
}

pub fn assert_collision_preserved(create_collision: impl FnOnce(&Path, &Path)) {
    let fixture = Fixture::new();
    fs::write(&fixture.original, b"original").unwrap();
    fs::write(&fixture.unrelated, b"unrelated data").unwrap();
    let original_file = File::options().write(true).open(&fixture.original).unwrap();
    original_file
        .set_times(FileTimes::new().set_modified(SystemTime::UNIX_EPOCH + Duration::from_secs(10)))
        .unwrap();
    let mut document = Document::open(&fixture.original).unwrap();
    document.replace_text("first edit");
    document.replace_text("second edit");
    assert!(document.undo());

    // Leave an external modification pending to verify a failed save does not
    // replace the document's remembered modification time.
    original_file
        .set_times(FileTimes::new().set_modified(SystemTime::UNIX_EPOCH + Duration::from_secs(20)))
        .unwrap();
    drop(original_file);
    assert!(document.has_external_changes().unwrap());
    let modified = fs::metadata(&fixture.original).unwrap().modified().unwrap();
    create_collision(&fixture.collision, &fixture.unrelated);
    let collision_bytes = fs::read(&fixture.collision).unwrap();
    let collision_link = fs::read_link(&fixture.collision).ok();

    let error = document.save().unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
    assert_eq!(fs::read(&fixture.original).unwrap(), b"original");
    assert_eq!(fs::read(&fixture.unrelated).unwrap(), b"unrelated data");
    assert_eq!(fs::read(&fixture.collision).unwrap(), collision_bytes);
    assert_eq!(fs::read_link(&fixture.collision).ok(), collision_link);
    assert_eq!(
        fs::metadata(&fixture.original).unwrap().modified().unwrap(),
        modified
    );
    assert!(document.has_external_changes().unwrap());
    assert_eq!(document.path(), fixture.original);
    assert_eq!(document.text(), "first edit");
    assert!(document.is_dirty());
    assert_eq!(document.request_close(), CloseOutcome::Cancelled);

    assert!(document.undo());
    assert_eq!(document.text(), "original");
    assert!(!document.is_dirty());
    assert!(!document.undo());
    assert!(document.redo());
    assert_eq!(document.text(), "first edit");
    assert!(document.is_dirty());
    assert!(document.redo());
    assert_eq!(document.text(), "second edit");
    assert!(!document.redo());
}
