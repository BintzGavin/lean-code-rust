# extensions-011 — Parse workspace trust capabilities and extension kind

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-011`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Parse `capabilities` and `extensionKind` from VS Code extension `package.json` manifests.
- **Vision gap**: Vision Promise 3 states "Workspace trust and untrusted extension/webview isolation must be explicit and tested." Current manifest parsing ignores these trust declarations.
- **Evidence**: `crates/extensions/src/manifest.rs` lacks the `capabilities` and `extensionKind` fields in the `ExtensionManifest` struct. `spec/extension-compatibility.md` lists workspace trust requirements. The official VS Code extension manifest documentation specifies `capabilities` (describing limited workspace capabilities like `untrustedWorkspaces`) and `extensionKind` (array indicating where the extension runs like `ui` or `workspace`).
- **Impact**: Enables the extension host to determine if an extension supports untrusted workspaces and where it should run, foundational for enforcing workspace trust boundaries.

## 2. File Inventory

```json
{
  "id": "extensions-011",
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

- **Required behavior**: The `ExtensionManifest` struct correctly extracts `capabilities` and `extensionKind`. Since `capabilities` is an object with varying schema (e.g., `untrustedWorkspaces` can be a boolean or an object), parse `capabilities` as `Option<serde_json::Value>`. Parse `extensionKind` as `Option<Vec<String>>` (renamed with `#[serde(rename = "extensionKind")]`).
- **Compatibility constraints**: Must not break existing parsing of commands, configurations, keybindings, menus, themes, languages, grammars, snippets, views, viewsContainers, debuggers, breakpoints, and tasks.
- **Architectural constraints**: Maintain the use of `serde` and `serde_json` for manifest parsing inside `crates/extensions/src/manifest.rs`. Keep structures exact and properly typed using `Option`.
- **Public contract changes**: Adds `capabilities` and `extension_kind` fields to `ExtensionManifest`.

## 4. Test Plan

- **Failing evidence**: Currently, a manifest with `capabilities` and `extensionKind` drops the data silently.
- **Verification commands**:
  - `cargo test --manifest-path crates/extensions/Cargo.toml`
  - `python3 tooling/check_scaffold.py --role extensions`
- **Success criteria**: A new unit test in `manifest.rs` parses a JSON block containing `capabilities` (with `untrustedWorkspaces`) and `extensionKind`, and successfully asserts their values.
- **Edge cases**: Missing `capabilities` or `extensionKind` should parse successfully as `None`.

## 5. Constraints

- **Non-goals**: Not implementing the actual workspace trust enforcement or extension host isolation logic. Just parsing the manifest declarations.
- **Forbidden changes**: Do not modify existing parsed structures beyond adding new fields to `ExtensionManifest`.
- **Scope ceiling**: Fits in a single execution run by focusing purely on JSON parsing and struct definitions.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: None

## 7. Result
