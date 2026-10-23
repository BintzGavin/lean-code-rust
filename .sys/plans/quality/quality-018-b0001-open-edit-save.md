# quality-018 — B-0001 Open, edit, save text document

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-018`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Author the test fixture and implement scenario S-0001 (B-0001) to verify opening, editing, and saving a local text document.
- **Vision gap**: `VISION.md` and `spec/requirements.md` require basic editing functionality (Open local folder, navigate, edit/save). This behavior (B-0001) lacks a test fixture and remains unexecuted in `spec/catalog.json` (execution_status: blocked).
- **Evidence**: `spec/catalog.json` lists S-0001 as blocked and unexecuted.
- **Impact**: Provides measured evidence of fundamental editing capability.

## 2. File Inventory

```json
{
  "id": "quality-018",
  "role": "quality",
  "create": [
    "tests/reference/b0001_open_edit_save/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/catalog.json",
    "spec/requirements.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Implement the reference test script `tests/reference/b0001_open_edit_save/test.sh` to test scenario S-0001. The script should verify that `alpha.txt` can be opened, "cedar" can be replaced with "maple", and the file can be saved.
- **Compatibility constraints**: Must be deterministic and isolated.
- **Architectural constraints**: Test script must reside in `tests/reference/b0001_open_edit_save/`.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `bash tests/reference/b0001_open_edit_save/test.sh` fails because it does not exist.
- **Verification commands**: `bash tests/reference/b0001_open_edit_save/test.sh` and `python3 tooling/check_scaffold.py --role quality`.
- **Success criteria**: `test.sh` executes, correctly sets up the fixture scenario and verifies the required test flow.
- **Edge cases**: Missing target file.

## 5. Constraints

- **Non-goals**: Modifying core Rust code.
- **Forbidden changes**: Do not modify files outside `tests/reference/b0001_open_edit_save/`.
- **Scope ceiling**: Setting up one behavior test fixture fits in one run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result

Completed implementation of test script for B-0001. Test verifies that opening, replacing text and saving behaves as expected using lean-editor-core. Verified script executes correctly and scaffold check passes.
