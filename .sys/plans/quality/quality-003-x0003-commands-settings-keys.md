# quality-003 — X-0003 Commands/settings/keys

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-003`

## 1. Context & Goal

- **Objective**: Create a compatibility fixture for command registration, configuration schema, and keybinding context (X-0003).
- **Vision gap**: D-0003 and product promises require VS Code extension parity. We need a verifiable test for command execution, scoped configuration, and keybinding contexts, which are currently unknown on the Rust implementation.
- **Evidence**: `spec/extension-compatibility.md` lists `X-0003 Commands/settings/keys` covering `S-0045` and `S-0046` as unexecuted/unknown.
- **Impact**: Unlocks the ability to measure parity for essential extension UI contributions (commands, settings, keybindings).

## 2. File Inventory

```json
{
  "id": "quality-003",
  "role": "quality",
  "create": [
    "tests/compatibility/x0003_commands_settings_keys/package.json",
    "tests/compatibility/x0003_commands_settings_keys/extension.js",
    "tests/compatibility/x0003_commands_settings_keys/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The fixture must register a command, contribute a configuration schema, and declare a keybinding with a context condition. It should prove invocation with arguments, configuration changes, and context switching.
- **Compatibility constraints**: Must use stable 1.133.0 public APIs (`vscode.commands.registerCommand`, `vscode.workspace.getConfiguration`, etc.).
- **Architectural constraints**: No external dependencies. Must execute locally as a standard VSIX.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `bash tests/compatibility/x0003_commands_settings_keys/test.sh` should report an initial state (likely absent or failing on the bare Rust core).
- **Verification commands**: `bash tests/compatibility/x0003_commands_settings_keys/test.sh` and `python3 tooling/check_scaffold.py --role quality`
- **Success criteria**: The script successfully validates the structure of the package and exercises the expected API calls.
- **Edge cases**: Configuration defaults, missing context, command invocation without arguments.

## 5. Constraints

- **Non-goals**: Implementing the actual Rust support for commands/settings. This is just the fixture.
- **Forbidden changes**: No changes to core implementation (`crates/`).
- **Scope ceiling**: Scoped exactly to X-0003 requirements from `spec/extension-compatibility.md`.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
