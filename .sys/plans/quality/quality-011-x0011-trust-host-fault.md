# quality-011 — X-0011 Trust and host fault compatibility fixture

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-011`

## 1. Context & Goal

- **Objective**: Implement the X-0011 Trust and host fault compatibility fixture to establish the public boundary for untrusted-workspace support and host isolation.
- **Vision gap**: Existing VS Code extension compatibility is mandatory. The exact boundary for workspace trust choices, actual activation/process/network effects, crash scope, and retained text recovery is unknown.
- **Evidence**: `spec/catalog.json` lists scenarios S-0061 and S-0062 as blocked with missing independently authored/pinned VSIX fixture and runnable identified reference.
- **Impact**: Unlocks the observation of workspace trust behaviors, host isolation, and recovery mechanisms required by the public extension contract.

## 2. File Inventory

```json
{
  "id": "quality-011",
  "role": "quality",
  "create": [
    "tests/compatibility/x0011_trust_host_fault/package.json",
    "tests/compatibility/x0011_trust_host_fault/extension.js",
    "tests/compatibility/x0011_trust_host_fault/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/catalog.json",
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The fixture must supply an extension declaring untrusted-workspace support with a harmless task/activation marker. It must establish a separate host fault-control method in a disposable profile. It must test opening an untrusted folder, granting trust, invoking controlled actions, making a buffer dirty, and triggering a host fault.
- **Compatibility constraints**: Must use only non-excluded stable APIs documented for VS Code extension development.
- **Architectural constraints**: Must operate independently of other fixtures; recreate a complete setup for each scenario (S-0061, S-0062).
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Currently, no fixture or tests exist for X-0011 in `tests/compatibility/`.
- **Verification commands**: `bash tests/compatibility/x0011_trust_host_fault/test.sh`
- **Success criteria**: The test script completes execution successfully, verifying that workspace trust flows and host fault recovery behaviors match the desired expectations without throwing unhandled exceptions.
- **Edge cases**: Declining trust initially, triggering faults when files are unsaved/dirty.

## 5. Constraints

- **Non-goals**: Not testing global network isolation or other unrelated features not part of X-0011.
- **Forbidden changes**: Do not modify product code, `VISION.md`, or existing test fixtures.
- **Scope ceiling**: Can be completed by creating standard extension files and a standalone bash test script in a single run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
