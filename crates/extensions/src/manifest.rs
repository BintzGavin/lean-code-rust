use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a VS Code extension manifest (`package.json`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionManifest {
    pub name: String,
    pub publisher: String,
    pub version: String,
    pub engines: HashMap<String, String>,
    pub main: Option<String>,
}

impl ExtensionManifest {
    /// Parses an ExtensionManifest from a JSON string.
    pub fn parse(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_manifest() {
        let json = r#"{
            "name": "my-extension",
            "publisher": "my-publisher",
            "version": "1.0.0",
            "engines": {
                "vscode": "^1.60.0"
            },
            "main": "./out/extension.js"
        }"#;

        let manifest = ExtensionManifest::parse(json).unwrap();
        assert_eq!(manifest.name, "my-extension");
        assert_eq!(manifest.publisher, "my-publisher");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.engines.get("vscode").unwrap(), "^1.60.0");
        assert_eq!(manifest.main, Some("./out/extension.js".to_string()));
    }

    #[test]
    fn test_parse_missing_optional_fields() {
        let json = r#"{
            "name": "my-extension",
            "publisher": "my-publisher",
            "version": "1.0.0",
            "engines": {
                "vscode": "^1.60.0"
            }
        }"#;

        let manifest = ExtensionManifest::parse(json).unwrap();
        assert_eq!(manifest.name, "my-extension");
        assert_eq!(manifest.publisher, "my-publisher");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.engines.get("vscode").unwrap(), "^1.60.0");
        assert_eq!(manifest.main, None);
    }

    #[test]
    fn test_parse_invalid_json() {
        let json = r#"{
            "name": "my-extension",
            "publisher": "my-publisher",
            "version": "1.0.0",
            "engines": {
                "vscode": "^1.60.0"
        }"#;

        let result = ExtensionManifest::parse(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_required_fields() {
        let json = r#"{
            "name": "my-extension",
            "publisher": "my-publisher",
            "version": "1.0.0"
        }"#; // Missing engines

        let result = ExtensionManifest::parse(json);
        assert!(result.is_err());
    }
}
