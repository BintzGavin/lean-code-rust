# QUALITY-007 — X-0007 Webview fixture

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-007`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Create the independent X-0007 Webview compatibility fixture and baseline behavior oracle.
- **Vision gap**: Existing VS Code extensions are a hard compatibility requirement (Decision D-0003), but no observable verification exists for webview panel lifecycle, messages, script/resource options, and state.
- **Evidence**: `spec/extension-compatibility.md` lists X-0007 (S-0053, S-0054) as unexecuted with unknown original oracles. `tests/compatibility/x0007_webview/` does not exist.
- **Impact**: This unlocks the ability to observe and measure baseline webview capabilities, essential for verifying untrusted extension isolation and interaction.

## 2. File Inventory

```json
{
  "id": "quality-007",
  "role": "quality",
  "create": [
    "tests/compatibility/x0007_webview/package.json",
    "tests/compatibility/x0007_webview/extension.js",
    "tests/compatibility/x0007_webview/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The fixture must implement S-0053 ("Exchange numbered synthetic messages and hide/reveal") and S-0054 ("dispose with pending message and attempt an outside-resource read"). It must demonstrate webview creation, message passing (`postMessage`), visibility state changes, disposal, and resource policy restrictions.
- **Compatibility constraints**: Use only ordinary stable public extension APIs for webviews (`vscode.window.createWebviewPanel`, etc.). Do not use proposed APIs.
- **Architectural constraints**: The fixture must be independent of the product runtime and testable via node or a synthetic runner.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `ls tests/compatibility/x0007_webview` will fail because it does not exist.
- **Verification commands**: `bash tests/compatibility/x0007_webview/test.sh`
- **Success criteria**: The directory and fixture files exist, and the manifest/syntax are valid according to the test script.
- **Edge cases**: The test must cover both successful message exchange flows and denied outside-resource reads.

## 5. Constraints

- **Non-goals**: Do not implement a real webview renderer, browser engine, or mock VS Code host. Do not test actual HTML rendering layout.
- **Forbidden changes**: Do not modify product code, existing specs, or paths outside `tests/compatibility/x0007_webview/`.
- **Scope ceiling**: Creating a synthetic extension fixture for webviews fits well within a single run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A

## 7. Result
