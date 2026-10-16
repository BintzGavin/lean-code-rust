# integration-008 — Measure process startup time

**Role**: `integration`
**Backlog entry**: `.sys/backlogs/integration.md#integration-008`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Create a script to measure the desktop application's startup time (cold and warm launch).
- **Vision gap**: The Vision requires measuring startup performance against the target (Warm launch p50 / p95 ≤600 / ≤900 ms, Cold launch p50 / p95 ≤1.2 / ≤1.7 s), but no measurement tooling exists yet.
- **Evidence**: `packaging/` has no script for startup measurement, and `dist/desktop` startup is currently unobserved.
- **Impact**: Unlocks the ability to observe and track startup time against targets specified in `spec/requirements.md`.

## 2. File Inventory

```json
{
  "id": "integration-008",
  "role": "integration",
  "create": [
    "tooling/measure_startup.sh"
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

- **Required behavior**: The script `tooling/measure_startup.sh` must measure the time it takes to launch `dist/desktop` and reach an accepted keystroke or settled state, or simply the time it takes for the process to exit if a headless run is triggered. It should output the time.
- **Compatibility constraints**: Must run on the local development environment.
- **Architectural constraints**: Must operate independently of the build script but rely on the `dist/desktop` artifact.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: No script exists to measure startup time.
- **Verification commands**:
  ```sh
  bash packaging/build_release.sh
  bash tooling/measure_startup.sh
  ```
- **Success criteria**: The script successfully launches the desktop artifact and outputs startup time measurements.
- **Edge cases**: Handles missing artifacts or a crashing process gracefully.

## 5. Constraints

- **Non-goals**: Optimizing startup time or measuring memory in this specific script.
- **Forbidden changes**: Modifying product code in `crates/` or the build script.
- **Scope ceiling**: A single bash script for startup measurement fits within one execution run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: The desktop application cannot be profiled automatically.

## 7. Result
