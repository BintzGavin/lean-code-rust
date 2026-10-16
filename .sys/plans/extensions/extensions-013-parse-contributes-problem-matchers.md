# extensions-013 — Parse contributes.problemMatchers and contributes.problemPatterns

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-013`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Parse `contributes.problemMatchers` and `contributes.problemPatterns` from a VS Code extension manifest (`package.json`).
- **Vision gap**: D-0003 and public documentation promise compatibility with existing local VSIX packages. The public API docs (e.g. https://code.visualstudio.com/api/references/contribution-points) specify `contributes.problemMatchers` and `contributes.problemPatterns` for defining custom problem matchers and patterns. The Rust `ExtensionContributes` struct currently does not parse these fields, which are needed to fulfill the debugging/diagnostics/output/problems product requirement.
- **Evidence**: `crates/extensions/src/manifest.rs` struct `ExtensionContributes` lacks fields for `problemMatchers` and `problemPatterns`. `spec/extension-compatibility.md` lists debugging/diagnostics/output/problems under product promises.
- **Impact**: Enables recognizing custom problem matchers and patterns defined by extensions, allowing the system to properly extract diagnostics from task output in the future.

## 2. File Inventory

```json
{
  "id": "extensions-013",
  "role": "extensions",
  "create": [],
  "modify": [
    "crates/extensions/src/manifest.rs"
  ],
  "read_only": [
    "VISION.md",
    "spec/extension-compatibility.md",
    ".sys/black-hole/work-order-template.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Add `problem_matchers` (renamed from `problemMatchers`) and `problem_patterns` (renamed from `problemPatterns`) fields to the `ExtensionContributes` struct in `crates/extensions/src/manifest.rs`. Both should be parsed as `Option<Vec<serde_json::Value>>` since their schema can be complex and it's sufficient to parse them as raw JSON values for now, consistent with how `configuration` and `taskDefinitions.properties` are handled. Write a unit test `test_parse_contributes_problem_matchers` in the same file to verify parsing these fields from a sample `package.json` string.
- **Compatibility constraints**: Follow the serde mapping conventions already established in `crates/extensions/src/manifest.rs`.
- **Architectural constraints**: Maintain the `Option<Vec<T>>` pattern for top-level contribution arrays. Use `serde_json::Value` to defer strict schema validation, as problem matcher configurations can be complex objects.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: A JSON string containing `contributes: { "problemMatchers": [{ "name": "my-matcher" }] }` will silently ignore the `problemMatchers` field when parsed into `ExtensionContributes`, making it unavailable to the application.
- **Verification commands**:
  - `cargo test --manifest-path crates/extensions/Cargo.toml`
  - `python3 tooling/check_scaffold.py --role extensions`
  - `bash tooling/validate_repo.sh`
- **Success criteria**: The tests pass, demonstrating that `problem_matchers` and `problem_patterns` are correctly parsed and accessible as `serde_json::Value` objects on the `ExtensionContributes` struct.
- **Edge cases**: Missing `problemMatchers` or `problemPatterns` fields should gracefully default to `None`. Malformed JSON should result in a deserialization error.

## 5. Constraints

- **Non-goals**: Do not implement a full typed schema for problem matchers and patterns at this time; `serde_json::Value` is sufficient. Do not integrate problem matchers into the task runner.
- **Forbidden changes**: Do not modify existing struct fields in `ExtensionContributes`.
- **Scope ceiling**: Adding two fields and a corresponding test is a single, bounded operation easily achievable in one run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
