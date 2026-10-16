# extensions-010 — Parse contributes.taskDefinitions

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-010`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Parse `contributes.taskDefinitions` from VS Code extension `package.json` manifests.
- **Vision gap**: Extension integration is a hard compatibility requirement (Vision 2). Terminal/tasks are requested features. The `spec/catalog.json` includes scenarios defining tasks (like `S-0051`, `S-0052`). The current `ExtensionContributes` struct does not parse `taskDefinitions`.
- **Evidence**: `crates/extensions/src/manifest.rs` lacks the `taskDefinitions` field in the `ExtensionContributes` struct. `spec/extension-compatibility.md` lists `terminal/task boundaries` under X-0006.
- **Impact**: Enables support for defining and executing local task boundaries provided by extensions by successfully parsing their definitions.

## 2. File Inventory

```json
{
  "id": "extensions-010",
  "role": "extensions",
  "create": [],
  "modify": [
    "crates/extensions/src/manifest.rs"
  ],
  "read_only": [
    "spec/catalog.json"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The `ExtensionContributes` struct correctly extracts `taskDefinitions` (a list of task definition structures containing `type`, `required`, and `properties` at a minimum). Since properties are dynamically typed based on JSON Schema syntax, we can parse `required` as `Option<Vec<String>>` and `properties` as `Option<serde_json::Value>`.
- **Compatibility constraints**: Must not break existing parsing of commands, configurations, keybindings, menus, themes, languages, grammars, snippets, views, viewsContainers, debuggers, and breakpoints.
- **Architectural constraints**: Maintain the use of `serde` and `serde_json` for manifest parsing inside `crates/extensions/src/manifest.rs`. Keep structures exact and properly typed using `Option` where fields may not always exist.
- **Public contract changes**: Adds a new public struct `ExtensionTaskDefinition` and adds it as `task_definitions: Option<Vec<ExtensionTaskDefinition>>` to `ExtensionContributes` (renamed with `#[serde(rename = "taskDefinitions")]`).

## 4. Test Plan

- **Failing evidence**: Currently, a manifest with `contributes.taskDefinitions` drops the data silently.
- **Verification commands**:
  - `cargo test --manifest-path crates/extensions/Cargo.toml`
  - `python3 tooling/check_scaffold.py --role extensions`
- **Success criteria**: A new unit test in `manifest.rs` parses a JSON block containing `taskDefinitions` and successfully asserts the values of `type`, `required`, and `properties`.
- **Edge cases**: Missing `taskDefinitions` should still parse successfully as `None`. Missing optional fields inside a `taskDefinition` should parse successfully as `None`.

## 5. Constraints

- **Non-goals**: Not implementing task execution, UI task running, or task resolving logic. Just parsing the manifest.
- **Forbidden changes**: Do not modify existing parsed structures beyond adding new fields to `ExtensionContributes`.
- **Scope ceiling**: Fits in a single execution run by focusing purely on JSON parsing and struct definitions.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: None

## 7. Result
