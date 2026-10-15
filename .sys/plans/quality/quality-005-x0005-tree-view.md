# quality-005 — X-0005 Tree view fixture

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-005`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Implement the X-0005 Tree view compatibility fixture.
- **Vision gap**: Extension tree view behavior (provider, identity, selection, reveal, refresh, commands) is required but currently unverified.
- **Evidence**: `spec/extension-compatibility.md` names X-0005 and scenarios S-0049, S-0050 as required probes.
- **Impact**: Establishes a verifiable baseline for extension-contributed tree views and lifecycle.

## 2. File Inventory

```json
{
  "id": "quality-005",
  "role": "quality",
  "create": [
    "tests/compatibility/x0005_tree_view/package.json",
    "tests/compatibility/x0005_tree_view/extension.js",
    "tests/compatibility/x0005_tree_view/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The fixture must implement a `vscode.TreeDataProvider`, handle expansion, item identity, selection, reveal, and refresh logic. It must provide ordered synthetic items and test disposing/recreating the view.
- **Compatibility constraints**: Must use Node.js and mock `vscode` to validate the syntax without the full Rust core.
- **Architectural constraints**: Keep logic self-contained in `tests/compatibility/x0005_tree_view/`.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `tests/compatibility/x0005_tree_view/test.sh` does not exist.
- **Verification commands**: `bash tests/compatibility/x0005_tree_view/test.sh` and `python3 tooling/check_scaffold.py --role quality`
- **Success criteria**: The script validates `package.json` structure and `extension.js` syntax using a synthetic `vscode` API mock.
- **Edge cases**: Verify behavior when an item is removed and the view is recreated.

## 5. Constraints

- **Non-goals**: Not implementing the Rust-side tree view host.
- **Forbidden changes**: Do not modify existing fixtures or `spec/`.
- **Scope ceiling**: A standalone fixture setup fits in one execution.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: Node.js validation toolchain is unavailable.

## 7. Result
