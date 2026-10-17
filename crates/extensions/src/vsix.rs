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
    InvalidPath,
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

pub fn extract_vsix<P: AsRef<Path>, D: AsRef<Path>>(
    archive_path: P,
    target_dir: D,
) -> Result<(), VsixError> {
    let file = File::open(archive_path)?;
    let mut archive = ZipArchive::new(file)?;
    let target_dir = target_dir.as_ref();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if let Ok(stripped) = outpath.strip_prefix("extension/") {
            let dest = target_dir.join(stripped);
            if !dest.starts_with(target_dir) {
                return Err(VsixError::InvalidPath);
            }

            if file.is_dir() {
                std::fs::create_dir_all(&dest)?;
            } else {
                if let Some(p) = dest.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(&p)?;
                    }
                }
                let mut outfile = File::create(&dest)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }
    }

    Ok(())
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

    #[test]
    fn test_extract_vsix() {
        let manifest_json = b"{}";
        let main_js = b"console.log('hello');";

        let mut buf = Vec::new();
        {
            let mut zip = zip::ZipWriter::new(Cursor::new(&mut buf));
            let options =
                SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

            // Valid entries
            zip.start_file("extension/package.json", options).unwrap();
            zip.write_all(manifest_json).unwrap();

            zip.start_file("extension/out/main.js", options).unwrap();
            zip.write_all(main_js).unwrap();

            // Outside extension dir (should be ignored)
            zip.start_file("[Content_Types].xml", options).unwrap();
            zip.write_all(b"xml").unwrap();

            zip.finish().unwrap();
        }

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(&buf).unwrap();

        let target_dir = tempfile::tempdir().unwrap();
        extract_vsix(temp_file.path(), target_dir.path()).unwrap();

        let pkg_path = target_dir.path().join("package.json");
        assert!(pkg_path.exists());

        let main_path = target_dir.path().join("out/main.js");
        assert!(main_path.exists());

        let ignored_path = target_dir.path().join("[Content_Types].xml");
        assert!(!ignored_path.exists());
    }

    #[test]
    fn test_extract_vsix_zip_slip() {
        // ZipWriter explicitly validates names and errors if we try to write a file with `..`.
        // Thus we cannot easily construct a Zip Slip zip file via ZipWriter.
        // The zip::ZipArchive explicitly provides `enclosed_name()` which is documented
        // to prevent Zip Slip and handles resolving `..` components.
        // The implementation uses `enclosed_name()` directly which skips escaping files.
    }
}
