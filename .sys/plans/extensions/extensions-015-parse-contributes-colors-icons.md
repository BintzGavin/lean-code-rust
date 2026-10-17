# EXTENSIONS-015 — Parse colors, icons, and icon themes

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-015`

## 1. Context & Goal

- **Objective**: Parse `colors`, `icons`, `iconThemes`, and `productIconThemes` from the `contributes` section of the extension manifest.
- **Vision gap**: The `ExtensionContributes` struct does not fully implement the VSIX lifecycle and pinned stable VS Code public extension contract for all relevant contribution points as verified by `spec/extension-compatibility.md` referring to https://code.visualstudio.com/api/references/contribution-points.
- **Evidence**: `ExtensionContributes` in `crates/extensions/src/manifest.rs` lacks fields for `colors`, `icons`, `iconThemes`, and `productIconThemes`. `grep` confirms these terms are not found in `crates/extensions/`.
- **Impact**: This unlocks support for extensions that contribute custom colors and icon themes.

## 2. File Inventory

```json
{
  "id": "extensions-015",
  "role": "extensions",
  "create": [],
  "modify": [
    "crates/extensions/src/manifest.rs"
  ],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The `ExtensionContributes` struct correctly parses the `colors`, `icons`, `iconThemes`, and `productIconThemes` contribution points into `Option<Vec<serde_json::Value>>` or similar loosely typed structures initially, matching how `problemMatchers` and `problemPatterns` are handled.
- **Compatibility constraints**: Must correctly parse standard `package.json` manifests from VS Code extensions containing these contribution points.
- **Architectural constraints**: Maintain the current pattern of using `serde` for parsing the manifest in `crates/extensions/src/manifest.rs`.
- **Public contract changes**: Adds `colors`, `icons`, `iconThemes`, and `productIconThemes` fields to `ExtensionContributes`.

## 4. Test Plan

- **Failing evidence**: Write a test with a manifest containing these fields and show that they are not parsed/available on `ExtensionContributes`.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: All tests pass, including the new ones verifying the parsing of the new fields.
- **Edge cases**: Missing fields should be parsed as `None`. Malformed inputs should fail parsing gracefully.

## 5. Constraints

- **Non-goals**: Do not implement the actual rendering or application logic for these themes/colors.
- **Forbidden changes**: Do not modify existing parsed fields.
- **Scope ceiling**: Parsing these specific contribution points is bounded and fits easily in one run.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: None

## 7. Result
