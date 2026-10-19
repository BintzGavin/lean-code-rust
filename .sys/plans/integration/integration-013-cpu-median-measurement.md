# integration-013 — Measure idle CPU median over five minutes

**Role**: `integration`
**Backlog entry**: `.sys/backlogs/integration.md#integration-013`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Measure desktop idle CPU median usage over five settled minutes.
- **Vision gap**: `spec/requirements.md` requires idle CPU to be calculated as a median over five settled minutes (`Idle CPU median | ≤0.5% over five settled minutes`). The current script calculates an average over 5 seconds.
- **Evidence**: `tooling/measure_cpu.sh` calculates `AVG_CPU` based on a 5-second `top` sample.
- **Impact**: Provides accurate, requirement-aligned CPU idle profiling for the desktop editor.

## 2. File Inventory

```json
{
  "id": "integration-013",
  "role": "integration",
  "create": [],
  "modify": [
    "tooling/measure_cpu.sh"
  ],
  "read_only": [
    "spec/requirements.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: `tooling/measure_cpu.sh` must start the desktop app, allow it to settle for 2 seconds (or an appropriate short time), then sample CPU usage via `top` for exactly 300 seconds (5 minutes). It must then calculate and report the median CPU usage across those samples instead of the average.
- **Compatibility constraints**: Must continue to use `xvfb-run` and find the `desktop` process correctly.
- **Architectural constraints**: Keep it as a simple bash script inside `tooling/`.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `bash tooling/measure_cpu.sh` currently reports "Average Idle CPU usage" after 5 seconds.
- **Verification commands**:
  - `bash tooling/measure_cpu.sh`
- **Success criteria**: Output indicates sampling over 5 minutes and reports "Median Idle CPU usage: [value]%".
- **Edge cases**: Missing `desktop` executable, script interruption.

## 5. Constraints

- **Non-goals**: No changes to desktop code or other measurement scripts.
- **Forbidden changes**: Modifying application logic to artificially lower CPU.
- **Scope ceiling**: Fits in one run; modifying a single bash script.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: `desktop` binary cannot be built.

## 7. Result

Completed: Modified tooling/measure_cpu.sh to sample over five minutes (300 samples) and calculate the median CPU usage.
