# extensions-017 — Parse missing manifest metadata fields

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-017`

## 1. Context & Goal

- **Objective**: Parse remaining missing metadata fields from the package.json extension manifest.
- **Vision gap**: Bounded first fixture matrix X-0010 VSIX lifecycle specifies parsing manifest engines/dependencies. We must have a complete manifest.
- **Evidence**: `crates/extensions/src/manifest.rs` `ExtensionManifest` is missing fields like `license`, `galleryBanner`, `preview`, `badges`, `qna`, `sponsor`, `pricing`, `bugs`, `repository`, and `homepage`.
- **Impact**: Enables reading all remaining required UI and Marketplace fields of a VS Code Extension.

## 2. File Inventory

```json
{
  "id": "extensions-017",
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

- **Required behavior**: `ExtensionManifest` parses `license` (Option<String>), `galleryBanner` (Option<serde_json::Value>), `preview` (Option<bool>), `badges` (Option<Vec<serde_json::Value>>), `qna` (Option<serde_json::Value>), `sponsor` (Option<serde_json::Value>), `pricing` (Option<String>), `bugs` (Option<serde_json::Value>), `repository` (Option<serde_json::Value>), and `homepage` (Option<String>). Notice `qna` can be boolean or string so serde_json::Value is safer. Use `serde(rename="galleryBanner")` where appropriate based on the manifest API.
- **Compatibility constraints**: Keep using serde for optional fields.
- **Architectural constraints**: Same as before, place in `ExtensionManifest`.
- **Public contract changes**: Added fields to `ExtensionManifest`.

## 4. Test Plan

- **Failing evidence**: Check `cargo test --manifest-path crates/extensions/Cargo.toml` without changes.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml` and `python3 tooling/check_scaffold.py --role extensions` and `bash tooling/validate_repo.sh`.
- **Success criteria**: Tests pass, struct successfully parses a package.json mock containing these fields.
- **Edge cases**: Missing fields shouldn't fail.

## 5. Constraints

- **Non-goals**: No other structs modified.
- **Forbidden changes**: Do not break existing parsing.
- **Scope ceiling**: Fits in a single executor run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result

Successfully parsed the missing manifest metadata fields into .
Successfully parsed the missing manifest metadata fields into `ExtensionManifest`.
