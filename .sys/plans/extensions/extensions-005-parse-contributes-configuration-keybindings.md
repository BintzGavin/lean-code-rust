# extensions-005 — Parse contributes.configuration and contributes.keybindings

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-005`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Extend `ExtensionManifest`'s `contributes` section to parse `configuration` and `keybindings`.
- **Vision gap**: VS Code extensions are a hard compatibility requirement (VISION.md). Specifically, parsing settings (configuration) and keyboard shortcuts (keybindings) is necessary for proper UI integration and extension behavior control, matching the stable API requirement.
- **Evidence**: `crates/extensions/src/manifest.rs` currently only parses `commands` within the `contributes` object.
- **Impact**: Unlocks accurate parsing of VSIX configuration schemas and keybindings for the desktop environment.

## 2. File Inventory

```json
{
  "id": "extensions-005",
  "role": "extensions",
  "create": [],
  "modify": [
    "crates/extensions/src/manifest.rs"
  ],
  "read_only": [
    "VISION.md"
  ],
  "dependencies": [
    {
      "role": "extensions",
      "plan": ".sys/plans/extensions/extensions-004-parse-extension-dependencies.md",
      "unblock": "cargo test --manifest-path crates/extensions/Cargo.toml"
    }
  ]
}
```

## 3. Implementation Spec

- **Required behavior**: Introduce `configuration` (which can be a single object or an array of objects) and `keybindings` (an array of objects) fields to the `ExtensionContributes` struct. Update parsing tests to verify. For `configuration`, an array of `serde_json::Value` or a single `serde_json::Value` is acceptable for now to capture the schema. For `keybindings`, capture at least `command`, `key`, and optionally `mac`, `linux`, `win`, `when`.
- **Compatibility constraints**: Fields must map properly even when omitted.
- **Architectural constraints**: Maintain isolation in `lean-extensions` using `serde` for parsing.
- **Public contract changes**: Enhance the `ExtensionContributes` struct in the public API.

## 4. Test Plan

- **Failing evidence**: Existing `ExtensionManifest` doesn't capture `configuration` or `keybindings` in tests.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml --locked`
- **Success criteria**: Unit tests successfully parse valid `configuration` and `keybindings` from JSON strings. Missing sections return `None`.
- **Edge cases**: Missing fields, empty arrays, or varied structure (like configuration being a single object vs array).

## 5. Constraints

- **Non-goals**: Not implementing the actual application of these configurations or keybindings in the UI.
- **Forbidden changes**: Do not modify files outside `crates/extensions/`.
- **Scope ceiling**: Parsing these static structures fits within one execution run.

## 6. Dependencies

- **Required first**: `.sys/plans/extensions/extensions-004-parse-extension-dependencies.md`
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
Extended ExtensionContributes to parse configuration and keybindings. Verified with unit tests.
