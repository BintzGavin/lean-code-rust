# extensions-008 — Parse contributes.views and contributes.viewsContainers

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-008`

## 1. Context & Goal

- **Objective**: Parse `contributes.views` and `contributes.viewsContainers` to support the VS Code extensions API.
- **Vision gap**: Vision promise 2 dictates ordinary local VS Code extensions are a hard compatibility requirement, and specifically mentions "tree views". This requires parsing `contributes.views` and `contributes.viewsContainers`.
- **Evidence**: `crates/extensions/src/manifest.rs` does not currently parse `views` or `viewsContainers` in `ExtensionContributes`.
- **Impact**: Enables future architectural support for extension tree views.

## 2. File Inventory

```json
{
  "id": "extensions-008",
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

- **Required behavior**: `ExtensionContributes` correctly parses `views` and `viewsContainers` as described in the VS Code API.
- **Compatibility constraints**: Follow existing `serde` usage.
- **Architectural constraints**: Pure JSON struct definitions inside `manifest.rs`. No logic or UI implementation yet.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `cargo test --manifest-path crates/extensions/Cargo.toml` without changes will fail if tests for views and viewsContainers are added.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: All tests pass.
- **Edge cases**: Missing `views` and `viewsContainers`.

## 5. Constraints

- **Non-goals**: Rendering the views, activating the tree view API.
- **Forbidden changes**: UI code, external crate additions.
- **Scope ceiling**: Just data structures and tests.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
