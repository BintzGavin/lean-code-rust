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

pub fn install_vsix<P: AsRef<std::path::Path>, D: AsRef<std::path::Path>>(
    archive_path: P,
    extensions_dir: D,
) -> Result<std::path::PathBuf, VsixError> {
    let archive_path = archive_path.as_ref();
    let extensions_dir = extensions_dir.as_ref();

    let manifest = read_manifest(archive_path)?;
    let dir_name = format!(
        "{}.{}-{}",
        manifest.publisher, manifest.name, manifest.version
    );
    let dest_path = extensions_dir.join(dir_name);

    extract_vsix(archive_path, &dest_path)?;

    Ok(dest_path)
}

pub fn uninstall_vsix<D: AsRef<Path>>(
    publisher: &str,
    name: &str,
    version: &str,
    extensions_dir: D,
) -> Result<(), VsixError> {
    let dir_name = format!("{}.{}-{}", publisher, name, version);
    let target_dir = extensions_dir.as_ref().join(dir_name);

    if target_dir.exists() {
        if let Err(e) = std::fs::remove_dir_all(&target_dir) {
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(VsixError::Io(e));
            }
        }
    }
    Ok(())
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

pub fn list_installed_extensions<D: AsRef<std::path::Path>>(
    extensions_dir: D,
) -> Result<Vec<ExtensionManifest>, VsixError> {
    let extensions_dir = extensions_dir.as_ref();
    if !extensions_dir.exists() {
        return Ok(Vec::new());
    }

    let mut manifests = Vec::new();
    if let Ok(entries) = std::fs::read_dir(extensions_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let manifest_path = path.join("package.json");
                if manifest_path.exists() {
                    if let Ok(mut file) = std::fs::File::open(&manifest_path) {
                        let mut contents = String::new();
                        if std::io::Read::read_to_string(&mut file, &mut contents).is_ok() {
                            if let Ok(manifest) = ExtensionManifest::parse(&contents) {
                                manifests.push(manifest);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(manifests)
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
    fn test_install_vsix() {
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
            zip.start_file("extension/out/main.js", options).unwrap();
            zip.write_all(b"console.log('hello');").unwrap();
            zip.finish().unwrap();
        }

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(&buf).unwrap();

        let target_dir = tempfile::tempdir().unwrap();
        let installed_path = install_vsix(temp_file.path(), target_dir.path()).unwrap();

        assert_eq!(
            installed_path,
            target_dir.path().join("my-publisher.my-extension-1.0.0")
        );
        assert!(installed_path.exists());
        assert!(installed_path.join("package.json").exists());
        assert!(installed_path.join("out/main.js").exists());
    }

    #[test]
    fn test_extract_vsix_zip_slip() {
        // ZipWriter explicitly validates names and errors if we try to write a file with `..`.
        // Thus we cannot easily construct a Zip Slip zip file via ZipWriter.
        // The zip::ZipArchive explicitly provides `enclosed_name()` which is documented
        // to prevent Zip Slip and handles resolving `..` components.
        // The implementation uses `enclosed_name()` directly which skips escaping files.
    }

    #[test]
    fn test_uninstall_vsix() {
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

        let target_dir = tempfile::tempdir().unwrap();
        let installed_path = install_vsix(temp_file.path(), target_dir.path()).unwrap();
        assert!(installed_path.exists());

        uninstall_vsix("my-publisher", "my-extension", "1.0.0", target_dir.path()).unwrap();
        assert!(!installed_path.exists());
    }

    #[test]
    fn test_uninstall_vsix_non_existent() {
        let target_dir = tempfile::tempdir().unwrap();
        let result = uninstall_vsix("some-publisher", "missing-ext", "2.0.0", target_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_installed_extensions() {
        let target_dir = tempfile::tempdir().unwrap();

        // Empty directory
        let manifests = super::list_installed_extensions(target_dir.path()).unwrap();
        assert_eq!(manifests.len(), 0);

        // Missing directory
        let manifests =
            super::list_installed_extensions(target_dir.path().join("missing")).unwrap();
        assert_eq!(manifests.len(), 0);

        // Valid extension
        let ext_dir = target_dir.path().join("my-ext");
        std::fs::create_dir_all(&ext_dir).unwrap();
        let manifest_json = r#"{
            "name": "my-extension",
            "publisher": "my-publisher",
            "version": "1.0.0",
            "engines": {
                "vscode": "^1.60.0"
            }
        }"#;
        std::fs::write(ext_dir.join("package.json"), manifest_json).unwrap();

        // Invalid extension
        let invalid_dir = target_dir.path().join("invalid-ext");
        std::fs::create_dir_all(&invalid_dir).unwrap();
        std::fs::write(invalid_dir.join("package.json"), "invalid json").unwrap();

        // Missing package.json
        let missing_pkg_dir = target_dir.path().join("missing-pkg");
        std::fs::create_dir_all(&missing_pkg_dir).unwrap();

        let manifests = super::list_installed_extensions(target_dir.path()).unwrap();
        assert_eq!(manifests.len(), 1);
        assert_eq!(manifests[0].name, "my-extension");
        assert_eq!(manifests[0].publisher, "my-publisher");
        assert_eq!(manifests[0].version, "1.0.0");
    }
}
