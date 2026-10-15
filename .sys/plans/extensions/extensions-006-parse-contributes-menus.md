# extensions-006 — Parse contributes.menus

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-006`

## 1. Context & Goal

- **Objective**: Extend `ExtensionContributes` to parse `menus` from VS Code extension manifests.
- **Vision gap**: Existing VS Code extensions require compatibility with `menus` contributions for command visibility and palette placement, per the `X-0003` compatibility fixture requirements in `spec/extension-compatibility.md`.
- **Evidence**: `crates/extensions/src/manifest.rs` currently parses `commands`, `configuration`, and `keybindings` in `ExtensionContributes`, but lacks `menus` parsing.
- **Impact**: Unlocks the ability to read and process extension menu and command palette contribution points, bringing us closer to VSIX compatibility.

## 2. File Inventory

```json
{
  "id": "extensions-006",
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

- **Required behavior**: `ExtensionContributes` must deserialize the `menus` field, which is an object where keys are menu IDs (like `commandPalette`, `editor/context`) and values are arrays of menu items containing `command`, `when`, `group`, and `alt`.
- **Compatibility constraints**: Must continue parsing all currently supported manifest fields exactly as before. The new `menus` field should be optional to support manifests without it.
- **Architectural constraints**: No external dependencies beyond standard library and `serde` / `serde_json`. Keep implementations in `crates/extensions/src/manifest.rs`.
- **Public contract changes**: Adds `menus` to the `ExtensionContributes` struct and introduces new structs for menu items.

## 4. Test Plan

- **Failing evidence**: Run `cargo test --manifest-path crates/extensions/Cargo.toml` with a newly added test for `menus`; it will fail to compile or pass because `menus` is not yet defined.
- **Verification commands**:
  - `cargo test --manifest-path crates/extensions/Cargo.toml`
  - `python3 tooling/check_scaffold.py --role extensions`
  - `bash tooling/validate_repo.sh`
- **Success criteria**: Tests pass successfully and parsing correctly extracts `menus`.
- **Edge cases**: Missing `menus`, empty `menus` arrays, missing optional fields in menu items.

## 5. Constraints

- **Non-goals**: No implementation of UI layout or command execution for menus; this is strictly about parsing the manifest.
- **Forbidden changes**: Do not modify crates outside of `crates/extensions`.
- **Scope ceiling**: This task focuses purely on updating the manifest parser, which is isolated to one file and its tests.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
