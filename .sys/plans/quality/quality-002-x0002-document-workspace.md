# QUALITY-002 — X-0002 Document/workspace fixture

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-002`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Create a compatibility fixture to validate the public document, edit, URI, file, and event interfaces.
- **Vision gap**: Vision promise #2 requires existing VS Code extension compatibility, including stable public APIs for documents and workspaces, which are not currently observed or proven by existing fixtures.
- **Evidence**: `spec/extension-compatibility.md` lists `X-0002 Document/workspace` as an unexecuted fixture for testing edit/save and capture events/bytes, denied or missing disposable file, and cancellation.
- **Impact**: Unlocks testing of editor core functions (document edits, saves, and event handling) through the extension host API.

## 2. File Inventory

```json
{
  "id": "quality-002",
  "role": "quality",
  "create": [
    "tests/compatibility/x0002_document_workspace/package.json",
    "tests/compatibility/x0002_document_workspace/extension.js",
    "tests/compatibility/x0002_document_workspace/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The fixture must register commands or activation events that exercise `vscode.workspace.textDocuments`, `vscode.workspace.openTextDocument`, `vscode.window.showTextDocument`, and `TextEdit` functionality. It must cover successful edit/save flows and failure/cancellation flows.
- **Compatibility constraints**: Must use only pinned stable public VS Code APIs. Node host dependencies must be restricted.
- **Architectural constraints**: Fixture must be self-contained in `tests/compatibility/x0002_document_workspace/`.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Currently, no fixture exists to test the document/workspace API, so tests do not exist to pass or fail.
- **Verification commands**: `bash tests/compatibility/x0002_document_workspace/test.sh` and `python3 tooling/check_scaffold.py --role quality`.
- **Success criteria**: The `test.sh` script exits with 0 and validates the syntax and basic structure of the fixture files.
- **Edge cases**: Missing files during open requests, cancelled edits.

## 5. Constraints

- **Non-goals**: Not implementing the actual Rust extension host or Rust core document edits. This is only the validation fixture.
- **Forbidden changes**: Do not modify product code (`crates/*`). Do not modify other fixtures.
- **Scope ceiling**: Fits in one run as it requires creating a single extension fixture directory with three small files.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: Node.js is not available for syntax validation.

## 7. Result
