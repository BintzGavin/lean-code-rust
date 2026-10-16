# extensions-012 — Parse enabledApiProposals

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-012`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Parse `enabledApiProposals` from VS Code extension `package.json` manifests.
- **Vision gap**: Vision Promise 2 requires VSIX lifecycle support. Vision Promise 4 states exclusions apply to non-core features and proposals. Tracking what API proposals an extension enables allows the host to isolate capabilities or flag non-compliant ones, directly relating to scenarios S-0063 and S-0064 where excluded/proposed capabilities are tested.
- **Evidence**: `crates/extensions/src/manifest.rs` currently lacks `enabledApiProposals` in the `ExtensionManifest` struct. The VS Code extension manifest API defines `enabledApiProposals` as a read-only string array.
- **Impact**: Enables the extension host to track proposed APIs used by extensions, which is required to enforce compatibility boundaries and API exclusion rules.

## 2. File Inventory

```json
{
  "id": "extensions-012",
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

- **Required behavior**: The `ExtensionManifest` struct correctly parses `enabledApiProposals` as an `Option<Vec<String>>`.
- **Compatibility constraints**: Must not break existing parsing of commands, configurations, views, debuggers, capabilities, etc.
- **Architectural constraints**: Maintain the use of `serde` and `serde_json` for manifest parsing inside `crates/extensions/src/manifest.rs`. Keep structures exact and properly typed using `Option`. Ensure to use `#[serde(rename = "enabledApiProposals")]`.
- **Public contract changes**: Adds `enabled_api_proposals` field to `ExtensionManifest`.

## 4. Test Plan

- **Failing evidence**: Currently, a manifest with `enabledApiProposals` drops the data silently.
- **Verification commands**:
  - `cargo test --manifest-path crates/extensions/Cargo.toml`
  - `python3 tooling/check_scaffold.py --role extensions`
- **Success criteria**: A new unit test in `manifest.rs` parses a JSON block containing `enabledApiProposals` and successfully asserts its value.
- **Edge cases**: Missing `enabledApiProposals` should parse successfully as `None`.

## 5. Constraints

- **Non-goals**: Not implementing the logic to reject or approve proposed APIs. Just parsing the manifest declarations.
- **Forbidden changes**: Do not modify existing parsed structures beyond adding new fields to `ExtensionManifest`.
- **Scope ceiling**: Fits in a single execution run by focusing purely on JSON parsing and struct definitions.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: None

## 7. Result
