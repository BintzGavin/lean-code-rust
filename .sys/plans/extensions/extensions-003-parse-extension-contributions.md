# EXTENSIONS-003 — Parse Extension Contributions

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-003`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Extend `ExtensionManifest` to parse `activationEvents` and basic `contributes.commands`.
- **Vision gap**: VS Code extensions are a hard compatibility requirement (VISION.md). Basic execution and lifecycle (X-0001, X-0003) require understanding when an extension activates (`activationEvents`) and what commands it registers (`contributes.commands`).
- **Evidence**: `crates/extensions/src/manifest.rs` currently lacks support for `activationEvents` and `contributes`. The compatibility matrix requires these features.
- **Impact**: Unlocks testing extension lifecycle integration points and provides the necessary data structures to track contribution points and activation triggers.

## 2. File Inventory

```json
{
  "id": "extensions-003",
  "role": "extensions",
  "create": [],
  "modify": [
    "crates/extensions/src/manifest.rs"
  ],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": [
    {
      "role": "extensions",
      "plan": ".sys/plans/extensions/extensions-002-parse-extension-manifest.md",
      "unblock": "cargo test --manifest-path crates/extensions/Cargo.toml"
    }
  ]
}
```

## 3. Implementation Spec

- **Required behavior**: Introduce `activationEvents` (a list of strings) and `contributes` (a nested structure containing at least `commands`, where a command has `command` and `title` string fields) to the `ExtensionManifest` struct. Update `parse` tests to verify these new fields.
- **Compatibility constraints**: Fields must match standard VS Code `package.json` schemas. They are optional and can be missing in some extensions.
- **Architectural constraints**: Maintain isolation in `lean-extensions` using `serde` for parsing.
- **Public contract changes**: Enhance the `ExtensionManifest` struct in the public API.

## 4. Test Plan

- **Failing evidence**: Currently, a JSON with `activationEvents` and `contributes` is not fully mapped in `ExtensionManifest`. Tests in `crates/extensions/src/manifest.rs` do not cover these properties.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml --locked`
- **Success criteria**: Unit tests successfully parse valid `activationEvents` arrays and `contributes.commands` arrays from JSON strings. The existing tests remain green, and the struct supports optional defaults where the fields are absent.
- **Edge cases**: Missing `activationEvents` or `contributes`, or empty lists within them.

## 5. Constraints

- **Non-goals**: Not implementing parsing for other contribution points (e.g., `themes`, `keybindings`, `menus`) in this single run.
- **Forbidden changes**: Do not modify files outside `crates/extensions/`.
- **Scope ceiling**: Updating one existing struct with two new JSON sub-objects is well-bounded.

## 6. Dependencies

- **Required first**: `.sys/plans/extensions/extensions-002-parse-extension-manifest.md`
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
