# extensions-016 — Parse contributes.submenus

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-016`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Parse `contributes.submenus` from VS Code extension `package.json` manifests.
- **Vision gap**: Vision Promise 3 requires "Real VS Code extension compatibility". Current manifest parsing ignores the `submenus` contribution point.
- **Evidence**: `crates/extensions/src/manifest.rs` lacks the `submenus` field in the `ExtensionContributes` struct. `spec/extension-compatibility.md` indicates command palette and menu requirements which rely on submenu contributions. The official VS Code extension manifest documentation specifies `submenus` with `id`, `label`, and an optional `icon` field.
- **Impact**: Enables the extension host to determine submenu structures for placing menu items correctly.

## 2. File Inventory

```json
{
  "id": "extensions-016",
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

- **Required behavior**: Add a new `ExtensionSubmenu` struct with `id` (String), `label` (String), and `icon` (Option<String>). Update `ExtensionContributes` to include a `submenus` field, which is an `Option<Vec<ExtensionSubmenu>>`. Update `ExtensionManifest`'s test valid JSON string with a sample `submenus` array and assert it is parsed correctly.
- **Compatibility constraints**: Must not break existing parsing of other contribution points.
- **Architectural constraints**: Maintain the use of `serde` and `serde_json` for manifest parsing inside `crates/extensions/src/manifest.rs`.
- **Public contract changes**: Adds `ExtensionSubmenu` struct and `submenus` field to `ExtensionContributes`.

## 4. Test Plan

- **Failing evidence**: Currently, a manifest with `contributes.submenus` drops the data silently.
- **Verification commands**:
  - `cargo test --manifest-path crates/extensions/Cargo.toml`
  - `python3 tooling/check_scaffold.py --role extensions`
- **Success criteria**: A new unit test in `manifest.rs` parses a JSON block containing `contributes.submenus`, and successfully asserts its parsed values.
- **Edge cases**: Missing `submenus` should parse successfully as `None`. Missing optional fields like `icon` should parse as `None`.

## 5. Constraints

- **Non-goals**: Not implementing the actual UI layout or menu tree execution for submenus; this is strictly about parsing the manifest.
- **Forbidden changes**: Do not modify existing parsed structures beyond adding new fields.
- **Scope ceiling**: Fits in a single execution run by focusing purely on JSON parsing and struct definitions.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: None

## 7. Result
Parsed contributes.submenus from package.json manifest.
