use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

use crate::manifest::ExtensionManifest;

#[derive(Debug)]
pub enum VsixError {
    Io(std::io::Error),
    Zip(zip::result::ZipError),
    Json(serde_json::Error),
    MissingManifest,
}

impl From<std::io::Error> for VsixError {
    fn from(err: std::io::Error) -> Self {
        VsixError::Io(err)
    }
}

impl From<zip::result::ZipError> for VsixError {
    fn from(err: zip::result::ZipError) -> Self {
        VsixError::Zip(err)
    }
}

impl From<serde_json::Error> for VsixError {
    fn from(err: serde_json::Error) -> Self {
        VsixError::Json(err)
    }
}

pub fn read_manifest<P: AsRef<Path>>(archive_path: P) -> Result<ExtensionManifest, VsixError> {
    let file = File::open(archive_path)?;
    let mut archive = ZipArchive::new(file)?;

    let mut file = match archive.by_name("extension/package.json") {
        Ok(file) => file,
        Err(zip::result::ZipError::FileNotFound) => return Err(VsixError::MissingManifest),
        Err(e) => return Err(VsixError::Zip(e)),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    ExtensionManifest::parse(&contents).map_err(VsixError::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};
    use tempfile::NamedTempFile;
    use zip::write::SimpleFileOptions;

    #[test]
    fn test_read_manifest_from_vsix() {
        let manifest_json = r#"{
            "name": "my-extension",
            "publisher": "my-publisher",
            "version": "1.0.0",
            "engines": {
                "vscode": "^1.60.0"
            }
        }"#;

        let mut buf = Vec::new();
        {
            let mut zip = zip::ZipWriter::new(Cursor::new(&mut buf));
            let options =
                SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
            zip.start_file("extension/package.json", options).unwrap();
            zip.write_all(manifest_json.as_bytes()).unwrap();
            zip.finish().unwrap();
        }

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(&buf).unwrap();

        let manifest = read_manifest(temp_file.path()).unwrap();
        assert_eq!(manifest.name, "my-extension");
        assert_eq!(manifest.publisher, "my-publisher");
        assert_eq!(manifest.version, "1.0.0");
    }

    #[test]
    fn test_missing_manifest() {
        let mut buf = Vec::new();
        {
            let mut zip = zip::ZipWriter::new(Cursor::new(&mut buf));
            let options =
                SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
            zip.start_file("extension/README.md", options).unwrap();
            zip.write_all(b"Hello").unwrap();
            zip.finish().unwrap();
        }

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(&buf).unwrap();

        let result = read_manifest(temp_file.path());
        assert!(matches!(result, Err(VsixError::MissingManifest)));
    }
}
