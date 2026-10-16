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
pub struct ExtensionTheme {
    pub label: Option<String>,
    #[serde(rename = "uiTheme")]
    pub ui_theme: Option<String>,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionLanguage {
    pub id: String,
    pub aliases: Option<Vec<String>>,
    pub extensions: Option<Vec<String>>,
    pub configuration: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionGrammar {
    pub language: Option<String>,
    #[serde(rename = "scopeName")]
    pub scope_name: Option<String>,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionSnippet {
    pub language: Option<String>,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionView {
    pub id: String,
    pub name: String,
    pub when: Option<String>,
    pub icon: Option<String>,
    #[serde(rename = "contextualTitle")]
    pub contextual_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionViewContainer {
    pub id: String,
    pub title: String,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionDebugger {
    #[serde(rename = "type")]
    pub debugger_type: String,
    pub label: Option<String>,
    pub program: Option<String>,
    pub runtime: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionBreakpoint {
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionTaskDefinition {
    #[serde(rename = "type")]
    pub task_type: String,
    pub required: Option<Vec<String>>,
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionContributes {
    pub commands: Option<Vec<ExtensionCommand>>,
    pub configuration: Option<serde_json::Value>,
    pub keybindings: Option<Vec<ExtensionKeybinding>>,
    pub menus: Option<HashMap<String, Vec<ExtensionMenuItem>>>,
    pub themes: Option<Vec<ExtensionTheme>>,
    pub languages: Option<Vec<ExtensionLanguage>>,
    pub grammars: Option<Vec<ExtensionGrammar>>,
    pub snippets: Option<Vec<ExtensionSnippet>>,
    pub views: Option<HashMap<String, Vec<ExtensionView>>>,
    #[serde(rename = "viewsContainers")]
    pub views_containers: Option<HashMap<String, Vec<ExtensionViewContainer>>>,
    pub debuggers: Option<Vec<ExtensionDebugger>>,
    pub breakpoints: Option<Vec<ExtensionBreakpoint>>,
    #[serde(rename = "taskDefinitions")]
    pub task_definitions: Option<Vec<ExtensionTaskDefinition>>,
    #[serde(rename = "problemMatchers")]
    pub problem_matchers: Option<Vec<serde_json::Value>>,
    #[serde(rename = "problemPatterns")]
    pub problem_patterns: Option<Vec<serde_json::Value>>,
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
    pub capabilities: Option<serde_json::Value>,
    #[serde(rename = "extensionKind")]
    pub extension_kind: Option<Vec<String>>,
    #[serde(rename = "extensionDependencies")]
    pub extension_dependencies: Option<Vec<String>>,
    #[serde(rename = "extensionPack")]
    pub extension_pack: Option<Vec<String>>,
    #[serde(rename = "enabledApiProposals")]
    pub enabled_api_proposals: Option<Vec<String>>,
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
    fn test_parse_themes_languages_grammars_snippets() {
        let json = r#"{
            "name": "my-extension",
            "publisher": "my-publisher",
            "version": "1.0.0",
            "engines": {
                "vscode": "^1.60.0"
            },
            "contributes": {
                "themes": [
                    {
                        "label": "Synthetic Theme",
                        "uiTheme": "vs-dark",
                        "path": "./synthetic-theme.json"
                    }
                ],
                "languages": [
                    {
                        "id": "synthetic",
                        "aliases": ["Synthetic", "synth"],
                        "extensions": [".synth"],
                        "configuration": "./synthetic-language-configuration.json"
                    }
                ],
                "grammars": [
                    {
                        "language": "synthetic",
                        "scopeName": "source.synthetic",
                        "path": "./synthetic-grammar.json"
                    }
                ],
                "snippets": [
                    {
                        "language": "synthetic",
                        "path": "./synthetic-snippets.json"
                    }
                ]
            }
        }"#;

        let manifest = ExtensionManifest::parse(json).unwrap();
        let contributes = manifest.contributes.unwrap();

        let themes = contributes.themes.unwrap();
        assert_eq!(themes.len(), 1);
        assert_eq!(themes[0].label, Some("Synthetic Theme".to_string()));
        assert_eq!(themes[0].ui_theme, Some("vs-dark".to_string()));
        assert_eq!(themes[0].path, "./synthetic-theme.json");

        let languages = contributes.languages.unwrap();
        assert_eq!(languages.len(), 1);
        assert_eq!(languages[0].id, "synthetic");
        assert_eq!(
            languages[0].aliases,
            Some(vec!["Synthetic".to_string(), "synth".to_string()])
        );
        assert_eq!(languages[0].extensions, Some(vec![".synth".to_string()]));
        assert_eq!(
            languages[0].configuration,
            Some("./synthetic-language-configuration.json".to_string())
        );

        let grammars = contributes.grammars.unwrap();
        assert_eq!(grammars.len(), 1);
        assert_eq!(grammars[0].language, Some("synthetic".to_string()));
        assert_eq!(grammars[0].scope_name, Some("source.synthetic".to_string()));
        assert_eq!(grammars[0].path, "./synthetic-grammar.json");

        let snippets = contributes.snippets.unwrap();
        assert_eq!(snippets.len(), 1);
        assert_eq!(snippets[0].language, Some("synthetic".to_string()));
        assert_eq!(snippets[0].path, "./synthetic-snippets.json");
    }

    #[test]
    fn test_parse_views_and_views_containers() {
        let json = r#"{
            "name": "my-extension",
            "publisher": "my-publisher",
            "version": "1.0.0",
            "engines": {
                "vscode": "^1.60.0"
            },
            "contributes": {
                "viewsContainers": {
                    "activitybar": [
                        {
                            "id": "package-explorer",
                            "title": "Package Explorer",
                            "icon": "media/dep.svg"
                        }
                    ]
                },
                "views": {
                    "explorer": [
                        {
                            "id": "nodeDependencies",
                            "name": "Node Dependencies",
                            "when": "explorerResourceIsFolder",
                            "icon": "media/dep.svg",
                            "contextualTitle": "Package Explorer"
                        }
                    ]
                }
            }
        }"#;

        let manifest = ExtensionManifest::parse(json).unwrap();
        let contributes = manifest.contributes.unwrap();

        let views_containers = contributes.views_containers.unwrap();
        assert_eq!(views_containers.len(), 1);
        let activitybar = views_containers.get("activitybar").unwrap();
        assert_eq!(activitybar.len(), 1);
        assert_eq!(activitybar[0].id, "package-explorer");
        assert_eq!(activitybar[0].title, "Package Explorer");
        assert_eq!(activitybar[0].icon, "media/dep.svg");

        let views = contributes.views.unwrap();
        assert_eq!(views.len(), 1);
        let explorer = views.get("explorer").unwrap();
        assert_eq!(explorer.len(), 1);
        assert_eq!(explorer[0].id, "nodeDependencies");
        assert_eq!(explorer[0].name, "Node Dependencies");
        assert_eq!(
            explorer[0].when,
            Some("explorerResourceIsFolder".to_string())
        );
        assert_eq!(explorer[0].icon, Some("media/dep.svg".to_string()));
        assert_eq!(
            explorer[0].contextual_title,
            Some("Package Explorer".to_string())
        );
    }

    #[test]
    fn test_parse_debuggers_and_breakpoints() {
        let json = r#"{
            "debuggers": [
                {
                    "type": "node",
                    "label": "Node Debug",
                    "program": "./out/debugAdapter.js",
                    "runtime": "node"
                }
            ],
            "breakpoints": [
                {
                    "language": "javascript"
                }
            ]
        }"#;

        let contributes: ExtensionContributes = serde_json::from_str(json).unwrap();

        let debuggers = contributes.debuggers.unwrap();
        assert_eq!(debuggers.len(), 1);
        assert_eq!(debuggers[0].debugger_type, "node");
        assert_eq!(debuggers[0].label, Some("Node Debug".to_string()));
        assert_eq!(
            debuggers[0].program,
            Some("./out/debugAdapter.js".to_string())
        );
        assert_eq!(debuggers[0].runtime, Some("node".to_string()));

        let breakpoints = contributes.breakpoints.unwrap();
        assert_eq!(breakpoints.len(), 1);
        assert_eq!(breakpoints[0].language, "javascript");
    }

    #[test]
    fn test_parse_task_definitions() {
        let json = r#"{
            "taskDefinitions": [
                {
                    "type": "npm",
                    "required": [
                        "script"
                    ],
                    "properties": {
                        "script": {
                            "type": "string",
                            "description": "The npm script to customize or the custom task to run."
                        },
                        "path": {
                            "type": "string",
                            "description": "The path to the folder of the package.json file that provides the script. Can be omitted."
                        }
                    }
                }
            ]
        }"#;
        let contributes: ExtensionContributes = serde_json::from_str(json).unwrap();
        let task_defs = contributes.task_definitions.unwrap();
        assert_eq!(task_defs.len(), 1);
        assert_eq!(task_defs[0].task_type, "npm");
        assert_eq!(task_defs[0].required, Some(vec!["script".to_string()]));
        assert!(task_defs[0].properties.is_some());
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
        assert_eq!(manifest.enabled_api_proposals, None);
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

    #[test]
    fn test_parse_enabled_api_proposals() {
        let json = r#"{
            "name": "my-extension",
            "publisher": "my-publisher",
            "version": "1.0.0",
            "engines": {
                "vscode": "^1.60.0"
            },
            "enabledApiProposals": [
                "proposedApi1",
                "proposedApi2"
            ]
        }"#;

        let manifest = ExtensionManifest::parse(json).unwrap();
        assert_eq!(manifest.name, "my-extension");

        let proposals = manifest.enabled_api_proposals.unwrap();
        assert_eq!(proposals, vec!["proposedApi1".to_string(), "proposedApi2".to_string()]);
    }


    #[test]
    fn test_parse_contributes_problem_matchers() {
        let json = r#"{
            "name": "my-extension",
            "publisher": "my-publisher",
            "version": "1.0.0",
            "engines": {
                "vscode": "^1.60.0"
            },
            "contributes": {
                "problemMatchers": [
                    {
                        "name": "my-matcher"
                    }
                ],
                "problemPatterns": [
                    {
                        "name": "my-pattern"
                    }
                ]
            }
        }"#;

        let manifest: ExtensionManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.name, "my-extension");

        let contributes = manifest.contributes.unwrap();

        let matchers = contributes.problem_matchers.unwrap();
        assert_eq!(matchers.len(), 1);
        assert_eq!(matchers[0].get("name").unwrap().as_str().unwrap(), "my-matcher");

        let patterns = contributes.problem_patterns.unwrap();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].get("name").unwrap().as_str().unwrap(), "my-pattern");
    }

    #[test]
    fn test_parse_workspace_trust_capabilities_and_extension_kind() {
        let json = r#"{
            "name": "my-extension",
            "publisher": "my-publisher",
            "version": "1.0.0",
            "engines": {
                "vscode": "^1.60.0"
            },
            "capabilities": {
                "untrustedWorkspaces": {
                    "supported": "limited",
                    "description": "Requires trusted workspace"
                }
            },
            "extensionKind": ["ui", "workspace"]
        }"#;

        let manifest: ExtensionManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.name, "my-extension");

        let capabilities = manifest.capabilities.unwrap();
        let untrusted = capabilities.get("untrustedWorkspaces").unwrap();
        assert_eq!(
            untrusted.get("supported").unwrap().as_str().unwrap(),
            "limited"
        );

        let extension_kind = manifest.extension_kind.unwrap();
        assert_eq!(
            extension_kind,
            vec!["ui".to_string(), "workspace".to_string()]
        );
    }
}
