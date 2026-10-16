# quality-010 — Implement X-0010 VSIX lifecycle compatibility fixture

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-010`

## 1. Context & Goal

- **Objective**: Implement the X-0010 VSIX lifecycle compatibility fixture.
- **Vision gap**: VSIX lifecycle (install, update, uninstall, disable, invalid package constraints) is a hard compatibility requirement (D-0003), but no reference test fixture or observation currently exists for it.
- **Evidence**: `spec/extension-compatibility.md` lists X-0010 (S-0059, S-0060) as a bounded requirement; `tests/compatibility/x0010_vsix_lifecycle/` is absent.
- **Impact**: Provides measured evidence and a synthetic contract boundary for VSIX lifecycle management.

## 2. File Inventory

```json
{
  "id": "quality-010",
  "role": "quality",
  "create": [
    "tests/compatibility/x0010_vsix_lifecycle/package.json",
    "tests/compatibility/x0010_vsix_lifecycle/extension.js",
    "tests/compatibility/x0010_vsix_lifecycle/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The `test.sh` script must validate the synthetic extension package structure for VSIX lifecycle capabilities (install v1, invoke, update v2, disable/re-enable, uninstall, invalid engine/dependency).
- **Compatibility constraints**: Test must remain independent and synthetic; it does not test a real marketplace or use `vsce`.
- **Architectural constraints**: Must conform to the `tests/compatibility` mock structure without relying on external VS Code binaries.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `ls tests/compatibility/x0010_vsix_lifecycle` fails.
- **Verification commands**:
  - `bash tests/compatibility/x0010_vsix_lifecycle/test.sh`
  - `python3 tooling/check_scaffold.py --role quality`
- **Success criteria**: The test correctly sets up node scripts that exit with a non-zero code on missing package dependencies and exit 0 when the lifecycle steps are correctly satisfied.
- **Edge cases**: Graceful failure for incompatible engines.

## 5. Constraints

- **Non-goals**: No Rust core integration. No actual package registry server.
- **Forbidden changes**: Do not alter `spec/extension-compatibility.md` or any other role's domain.
- **Scope ceiling**: Fits in one execution.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
