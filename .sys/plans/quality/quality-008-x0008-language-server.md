# QUALITY-008 — X-0008 Language server fixture

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-008`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Create the independent X-0008 Language server compatibility fixture and baseline behavior oracle.
- **Vision gap**: Existing VS Code extensions are a hard compatibility requirement (Decision D-0003), but no observable verification exists for LSP negotiation, language features, and server lifecycle (S-0055, S-0056).
- **Evidence**: `spec/extension-compatibility.md` lists X-0008 (S-0055, S-0056) as unexecuted with unknown original oracles. `tests/compatibility/x0008_language_server/` does not exist.
- **Impact**: This unlocks the ability to observe and measure baseline LSP client/server capabilities, essential for verifying language extensions.

## 2. File Inventory

```json
{
  "id": "quality-008",
  "role": "quality",
  "create": [
    "tests/compatibility/x0008_language_server/package.json",
    "tests/compatibility/x0008_language_server/extension.js",
    "tests/compatibility/x0008_language_server/server.js",
    "tests/compatibility/x0008_language_server/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The fixture must implement S-0055 ("Start local deterministic server; observe language features") and S-0056 ("cancel request and disconnect/restart server"). It must demonstrate starting a node-based LSP server, exchanging initialization and basic feature messages, and handling server restart/cancellation.
- **Compatibility constraints**: Use ordinary stable public extension APIs (e.g., `vscode-languageclient` patterns, child process spawning for the server). Do not use proposed APIs.
- **Architectural constraints**: The fixture must be independent of the product runtime and testable via node or a synthetic runner.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `ls tests/compatibility/x0008_language_server` will fail because it does not exist.
- **Verification commands**: `bash tests/compatibility/x0008_language_server/test.sh`
- **Success criteria**: The directory and fixture files exist, and the manifest/syntax are valid according to the test script, successfully exchanging mock LSP messages.
- **Edge cases**: The test must cover both successful feature requests and request cancellation/server disconnection.

## 5. Constraints

- **Non-goals**: Do not implement a fully compliant LSP server or advanced language features. Do not test actual editor integration.
- **Forbidden changes**: Do not modify product code, existing specs, or paths outside `tests/compatibility/x0008_language_server/`.
- **Scope ceiling**: Creating a synthetic extension fixture for a basic LSP server fits well within a single run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A

## 7. Result
