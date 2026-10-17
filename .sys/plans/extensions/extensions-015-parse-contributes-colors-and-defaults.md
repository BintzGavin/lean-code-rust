# [extensions-015] — Parse contributes.colors and contributes.configurationDefaults

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-015`

## 1. Context & Goal

- **Objective**: Extend ExtensionContributes parsing to support `colors` and `configurationDefaults`.
- **Vision gap**: VS Code extensions can contribute custom colors and language-specific default configurations via `contributes.colors` and `contributes.configurationDefaults`. The current Rust parser discards these fields.
- **Evidence**: `ExtensionContributes` in `crates/extensions/src/manifest.rs` does not include `colors` or `configurationDefaults`.
- **Impact**: Unlocks themes and extensions that specify color palettes and default formatter/editor settings.

## 2. File Inventory

```json
{
  "id": "extensions-015",
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

- **Required behavior**: `ExtensionContributes` correctly parses `colors` (as a list of color definitions) and `configurationDefaults` (as a JSON value or hash map) from `package.json`.
- **Compatibility constraints**: Unknown or omitted fields must be gracefully handled (e.g., using `Option`).
- **Architectural constraints**: Maintain serde derive macros and maintain current struct visibility.
- **Public contract changes**: Exposes new public fields on `ExtensionContributes`.

## 4. Test Plan

- **Failing evidence**: A test attempting to parse a JSON string containing `colors` and `configurationDefaults` into `ExtensionContributes` would not capture those values.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: Tests pass, verifying that `ExtensionContributes` has populated `colors` and `configurationDefaults`.
- **Edge cases**: Missing `colors` or `configurationDefaults`, invalid formats.

## 5. Constraints

- **Non-goals**: Applying these colors or defaults to the core editor state or theme engine.
- **Forbidden changes**: Modifying non-extensions crate logic.
- **Scope ceiling**: Parsing these two contribution points requires small, localized struct updates and one test.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: N/A

## 7. Result
