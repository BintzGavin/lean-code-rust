# extensions-009 — Parse contributes.debuggers and contributes.breakpoints

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-009`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Parse `contributes.debuggers` and `contributes.breakpoints` from `package.json`.
- **Vision gap**: D-0003 and public documentation promise DAP debug adapter integration. A Rust core alone does not satisfy this promise; we must correctly load debugger contribution metadata to register debug adapters.
- **Evidence**: `spec/extension-compatibility.md` names DAP and debug adapters as supported boundaries (X-0009). The current `ExtensionContributes` struct in `crates/extensions/src/manifest.rs` lacks `debuggers` and `breakpoints` fields.
- **Impact**: Enables future execution of DAP and debug configuration workflows for existing extensions.

## 2. File Inventory

```json
{
  "id": "extensions-009",
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

- **Required behavior**: The `ExtensionContributes` struct and its JSON parsing correctly extract `debuggers` (a list of debugger configurations with `type`, `label`, `program`, `runtime`, etc.) and `breakpoints` (a list of languages).
- **Compatibility constraints**: Must not break existing parsing of views, keybindings, commands, languages, themes, grammars, and snippets.
- **Architectural constraints**: Maintain use of `serde` and `serde_json` for manifest parsing inside `crates/extensions/src/manifest.rs`. Keep structures exact and typed appropriately (e.g. `Option<String>`, `Option<Vec<String>>`).
- **Public contract changes**: Adds new public structs `ExtensionDebugger` and `ExtensionBreakpoint` and adds them as `Option<Vec<...>>` to `ExtensionContributes`.

## 4. Test Plan

- **Failing evidence**: Currently, a manifest with `contributes.debuggers` will parse but drop the debuggers data silently.
- **Verification commands**:
  - `cargo test --manifest-path crates/extensions/Cargo.toml`
  - `python3 tooling/check_scaffold.py --role extensions`
- **Success criteria**: A new unit test in `manifest.rs` parses a JSON block containing both `debuggers` and `breakpoints` and successfully asserts the values.
- **Edge cases**: Missing `debuggers` or `breakpoints` blocks should still parse successfully as `None`. Missing optional fields inside a debugger definition should parse successfully as `None`.

## 5. Constraints

- **Non-goals**: Not implementing DAP process launching or debug protocol communication. Just parsing the manifest.
- **Forbidden changes**: Do not change VS Code manifest schemas. Do not modify existing parsed structures.
- **Scope ceiling**: Fits in a single execution run by focusing purely on JSON parsing and struct definitions.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: None

## 7. Result
