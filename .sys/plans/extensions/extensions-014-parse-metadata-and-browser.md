# extensions-014 — Parse metadata and browser entry point

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-014`

## 1. Context & Goal

- **Objective**: Parse extension metadata (`displayName`, `description`, `icon`, `categories`, `keywords`) and the `browser` entry point from `package.json`.
- **Vision gap**: The `ExtensionManifest` lacks properties necessary to support web extensions (`browser`) and UI presentation during the VSIX lifecycle.
- **Evidence**: `crates/extensions/src/manifest.rs` defines `ExtensionManifest` without `displayName`, `description`, `icon` (on the top-level manifest), `categories`, `keywords`, or `browser`.
- **Impact**: Enables reading web extension entry points and providing rich information during extension management.

## 2. File Inventory

```json
{
  "id": "extensions-014",
  "role": "extensions",
  "create": [],
  "modify": [
    "crates/extensions/src/manifest.rs"
  ],
  "read_only": [
    "VISION.md",
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: `ExtensionManifest` successfully deserializes `displayName`, `description`, `icon`, `categories`, `keywords`, and `browser` when present in `package.json`.
- **Compatibility constraints**: Fields must be strictly typed but optional to avoid rejecting manifests where they are omitted. `categories` and `keywords` should be `Option<Vec<String>>`.
- **Architectural constraints**: Extend the existing `ExtensionManifest` struct.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: No tests check these properties on `ExtensionManifest`.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: Tests in `manifest.rs` successfully parse these fields and correctly handle their absence.
- **Edge cases**: Missing optional fields, empty string values.

## 5. Constraints

- **Non-goals**: Modifying extension installation or UI; only manifest parsing is in scope.
- **Forbidden changes**: Do not alter existing successfully parsed fields like `main` or `activationEvents`.
- **Scope ceiling**: A single struct update and accompanying unit test.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: N/A

## 7. Result
- **Result:** Parsed metadata and browser entry point from package.json manifest.
