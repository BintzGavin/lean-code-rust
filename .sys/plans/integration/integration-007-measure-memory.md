# integration-007 — Measure process memory

**Role**: `integration`
**Backlog entry**: `.sys/backlogs/integration.md#integration-007`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Create a script to measure the full process-tree private memory of the running desktop application.
- **Vision gap**: The Vision requires measuring full process-tree private memory against the idle private memory target, but no measurement tooling exists yet.
- **Evidence**: `packaging/` has no script for memory measurement, and `dist/desktop` memory usage is currently unobserved.
- **Impact**: Unlocks the ability to observe and track idle private memory against targets specified in `spec/requirements.md`.

## 2. File Inventory

```json
{
  "id": "integration-007",
  "role": "integration",
  "create": [
    "tooling/measure_memory.sh"
  ],
  "modify": [],
  "read_only": [
    "VISION.md",
    "packaging/build_release.sh",
    "spec/requirements.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The script `tooling/measure_memory.sh` must launch `dist/desktop` in the background, wait for it to settle, measure its full process-tree private memory (e.g., using `ps` or a similar tool to sum RSS/private memory of the process and its children), print the result, and cleanly terminate the process.
- **Compatibility constraints**: Must run on the local development environment.
- **Architectural constraints**: Must operate independently of the build script but rely on the `dist/desktop` artifact.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: No script exists to measure memory.
- **Verification commands**:
  ```sh
  bash packaging/build_release.sh
  bash tooling/measure_memory.sh
  ```
- **Success criteria**: The script successfully launches the desktop artifact, outputs a memory measurement in MB, and terminates the process without leaving dangling background jobs.
- **Edge cases**: The desktop process might crash on startup, so the script should handle or report if the PID is no longer running.

## 5. Constraints

- **Non-goals**: Optimizing the memory usage or measuring startup time and network in this specific script.
- **Forbidden changes**: Modifying product code in `crates/` or the build script.
- **Scope ceiling**: A single bash script for memory measurement fits within one execution run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: The desktop application cannot launch in a headless or automated way, preventing measurement.

## 7. Result
