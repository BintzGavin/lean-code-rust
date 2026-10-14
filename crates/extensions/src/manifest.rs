use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionCommand {
    pub command: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionContributes {
    pub commands: Option<Vec<ExtensionCommand>>,
}

/// Represents a VS Code extension manifest (`package.json`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionManifest {
    pub name: String,
    pub publisher: String,
    pub version: String,
    pub engines: HashMap<String, String>,
    pub main: Option<String>,
    #[serde(rename = "activationEvents")]
    pub activation_events: Option<Vec<String>>,
    pub contributes: Option<ExtensionContributes>,
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
            "main": "./out/extension.js",
            "activationEvents": [
                "onCommand:extension.helloWorld"
            ],
            "contributes": {
                "commands": [
                    {
                        "command": "extension.helloWorld",
                        "title": "Hello World"
                    }
                ]
            }
        }"#;

        let manifest = ExtensionManifest::parse(json).unwrap();
        assert_eq!(manifest.name, "my-extension");
        assert_eq!(manifest.publisher, "my-publisher");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.engines.get("vscode").unwrap(), "^1.60.0");
        assert_eq!(manifest.main, Some("./out/extension.js".to_string()));

        let activation_events = manifest.activation_events.unwrap();
        assert_eq!(activation_events.len(), 1);
        assert_eq!(activation_events[0], "onCommand:extension.helloWorld");

        let contributes = manifest.contributes.unwrap();
        let commands = contributes.commands.unwrap();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].command, "extension.helloWorld");
        assert_eq!(commands[0].title, "Hello World");
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
        assert_eq!(manifest.activation_events, None);
        assert_eq!(manifest.contributes, None);
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
