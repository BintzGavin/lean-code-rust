use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionCommand {
    pub command: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionKeybinding {
    pub command: String,
    pub key: String,
    pub mac: Option<String>,
    pub linux: Option<String>,
    pub win: Option<String>,
    pub when: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionMenuItem {
    pub command: String,
    pub when: Option<String>,
    pub group: Option<String>,
    pub alt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionContributes {
    pub commands: Option<Vec<ExtensionCommand>>,
    pub configuration: Option<serde_json::Value>,
    pub keybindings: Option<Vec<ExtensionKeybinding>>,
    pub menus: Option<HashMap<String, Vec<ExtensionMenuItem>>>,
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
    #[serde(rename = "extensionDependencies")]
    pub extension_dependencies: Option<Vec<String>>,
    #[serde(rename = "extensionPack")]
    pub extension_pack: Option<Vec<String>>,
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
                ],
                "configuration": {
                    "type": "object",
                    "title": "My config",
                    "properties": {
                        "my.setting": {
                            "type": "boolean",
                            "default": true
                        }
                    }
                },
                "keybindings": [
                    {
                        "command": "extension.helloWorld",
                        "key": "ctrl+f1",
                        "mac": "cmd+f1",
                        "when": "editorTextFocus"
                    }
                ],
                "menus": {
                    "commandPalette": [
                        {
                            "command": "extension.helloWorld",
                            "when": "editorTextFocus"
                        }
                    ]
                }
            },
            "extensionDependencies": [
                "some.dependency"
            ],
            "extensionPack": [
                "some.pack1",
                "some.pack2"
            ]
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

        assert!(contributes.configuration.is_some());

        let keybindings = contributes.keybindings.unwrap();
        assert_eq!(keybindings.len(), 1);
        assert_eq!(keybindings[0].command, "extension.helloWorld");
        assert_eq!(keybindings[0].key, "ctrl+f1");
        assert_eq!(keybindings[0].mac, Some("cmd+f1".to_string()));
        assert_eq!(keybindings[0].when, Some("editorTextFocus".to_string()));
        assert_eq!(keybindings[0].linux, None);

        let menus = contributes.menus.unwrap();
        assert_eq!(menus.len(), 1);
        let palette_menus = menus.get("commandPalette").unwrap();
        assert_eq!(palette_menus.len(), 1);
        assert_eq!(palette_menus[0].command, "extension.helloWorld");
        assert_eq!(palette_menus[0].when, Some("editorTextFocus".to_string()));
        assert_eq!(palette_menus[0].group, None);
        assert_eq!(palette_menus[0].alt, None);

        let dependencies = manifest.extension_dependencies.unwrap();
        assert_eq!(dependencies.len(), 1);
        assert_eq!(dependencies[0], "some.dependency");

        let pack = manifest.extension_pack.unwrap();
        assert_eq!(pack.len(), 2);
        assert_eq!(pack[0], "some.pack1");
        assert_eq!(pack[1], "some.pack2");
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
        assert_eq!(manifest.extension_dependencies, None);
        assert_eq!(manifest.extension_pack, None);
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
