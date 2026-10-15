# extensions-007 — Parse contributes.themes, languages, grammars, snippets

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-007`

## 1. Context & Goal

- **Objective**: Extend `ExtensionContributes` to parse `themes`, `languages`, `grammars`, and `snippets` from VS Code extension manifests.
- **Vision gap**: Existing VS Code extensions require compatibility with theme and language contributions, per the `x0004_theme_grammar_snippet` compatibility fixture requirements in `tests/compatibility/`.
- **Evidence**: `crates/extensions/src/manifest.rs` currently parses `commands`, `configuration`, `keybindings`, and `menus` in `ExtensionContributes`, but lacks `themes`, `languages`, `grammars`, and `snippets` parsing.
- **Impact**: Unlocks the ability to read and process extension theme and language-related contribution points, bringing us closer to VSIX compatibility.

## 2. File Inventory

```json
{
  "id": "extensions-007",
  "role": "extensions",
  "create": [],
  "modify": [
    "crates/extensions/src/manifest.rs"
  ],
  "read_only": [],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: `ExtensionContributes` must deserialize the `themes`, `languages`, `grammars`, and `snippets` fields as arrays of objects according to VS Code's extension manifest format (e.g. `themes` containing `label`, `uiTheme`, `path`).
- **Compatibility constraints**: Must continue parsing all currently supported manifest fields exactly as before. The new fields should be optional to support manifests without them.
- **Architectural constraints**: No external dependencies beyond standard library and `serde` / `serde_json`. Keep implementations in `crates/extensions/src/manifest.rs`.
- **Public contract changes**: Adds `themes`, `languages`, `grammars`, and `snippets` to the `ExtensionContributes` struct and introduces new structs for these items.

## 4. Test Plan

- **Failing evidence**: Run `cargo test --manifest-path crates/extensions/Cargo.toml` with a newly added test for these fields; it will fail to compile or pass because they are not yet defined.
- **Verification commands**:
  - `cargo test --manifest-path crates/extensions/Cargo.toml`
  - `python3 tooling/check_scaffold.py --role extensions`
  - `bash tooling/validate_repo.sh`
- **Success criteria**: Tests pass successfully and parsing correctly extracts the new fields.
- **Edge cases**: Missing fields, empty arrays, missing optional fields in items.

## 5. Constraints

- **Non-goals**: No implementation of theme rendering or grammar highlighting; this is strictly about parsing the manifest.
- **Forbidden changes**: Do not modify crates outside of `crates/extensions`.
- **Scope ceiling**: This task focuses purely on updating the manifest parser, which is isolated to one file and its tests.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
