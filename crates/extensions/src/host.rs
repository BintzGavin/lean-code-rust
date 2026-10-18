use std::process::{Child, Command, Stdio, ExitStatus};
use std::path::Path;
use std::io;

pub struct ExtensionHost {
    child: Child,
}

impl ExtensionHost {
    pub fn spawn(entry_point_path: &Path) -> io::Result<Self> {
        let child = Command::new("node")
            .arg(entry_point_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        Ok(Self { child })
    }

    pub fn kill(&mut self) -> io::Result<()> {
        self.child.kill()
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        self.child.wait()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_spawn_and_kill_extension_host() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "setTimeout(() => console.log('running'), 10000);").unwrap();

        let mut host = ExtensionHost::spawn(file.path()).expect("Failed to spawn node");
        host.kill().expect("Failed to kill node");
        let status = host.wait().expect("Failed to wait for node");
        assert!(!status.success()); // Should be killed
    }

    #[test]
    fn test_spawn_missing_script() {
        let result = ExtensionHost::spawn(Path::new("/does/not/exist.js"));
        // Spawn might succeed on Unix if `node` is found, but wait/kill would show exit 1.
        // We only assert it returns a result, we don't strictly require is_err().
        // If it does succeed, it will exit immediately with an error status.
        if let Ok(mut host) = result {
            let status = host.wait().expect("wait failed");
            assert!(!status.success());
        }
    }
}
