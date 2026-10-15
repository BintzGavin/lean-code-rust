# extensions-004 — Parse extensionDependencies and extensionPack

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-004`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Extend `ExtensionManifest` to parse `extensionDependencies` and `extensionPack` arrays.
- **Vision gap**: VS Code extensions are a hard compatibility requirement (VISION.md). Specifically, the VSIX lifecycle requires correctly understanding and handling extension packs and functional dependencies.
- **Evidence**: `crates/extensions/src/manifest.rs` currently lacks support for `extensionDependencies` and `extensionPack` which are defined in the public VS Code Extension manifest documentation.
- **Impact**: Unlocks accurate VSIX dependency resolution when executing lifecycle steps such as installation and updates.

## 2. File Inventory

```json
{
  "id": "extensions-004",
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
      "plan": ".sys/plans/extensions/extensions-003-parse-extension-contributions.md",
      "unblock": "cargo test --manifest-path crates/extensions/Cargo.toml"
    }
  ]
}
```

## 3. Implementation Spec

- **Required behavior**: Introduce `extensionDependencies` (an optional list of strings) and `extensionPack` (an optional list of strings) to the `ExtensionManifest` struct. Update `parse` tests to verify these new fields.
- **Compatibility constraints**: Fields must match the public VS Code `package.json` manifest documentation schema.
- **Architectural constraints**: Maintain isolation in `lean-extensions` using `serde` for parsing.
- **Public contract changes**: Enhance the `ExtensionManifest` struct in the public API.

## 4. Test Plan

- **Failing evidence**: A JSON payload with `extensionDependencies` and `extensionPack` arrays is not fully mapped in `ExtensionManifest`. Tests in `crates/extensions/src/manifest.rs` do not currently test these properties.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml --locked`
- **Success criteria**: Unit tests successfully parse valid `extensionDependencies` and `extensionPack` arrays from JSON strings. Existing tests remain passing, and the struct provides `None` when the properties are omitted.
- **Edge cases**: Missing `extensionDependencies` or `extensionPack`, or empty lists for either.

## 5. Constraints

- **Non-goals**: Not implementing the actual installation or lifecycle execution logic, only the parsing.
- **Forbidden changes**: Do not modify files outside `crates/extensions/`.
- **Scope ceiling**: Adding two new optional arrays to an existing struct is well-bounded.

## 6. Dependencies

- **Required first**: `.sys/plans/extensions/extensions-003-parse-extension-contributions.md`
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
