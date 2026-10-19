# integration-014 — Measure memory after 60 seconds

**Role**: `integration`
**Backlog entry**: `.sys/backlogs/integration.md#integration-014`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Measure desktop memory usage after a 60-second settle time.
- **Vision gap**: `spec/requirements.md` requires settled memory improvement to be measured "after 60 seconds" (`Settled memory improvement | At least 35% lower after 60 seconds`). The current script `tooling/measure_memory.sh` sleeps for only 2 seconds before measuring.
- **Evidence**: `tooling/measure_memory.sh` contains `sleep 2` before reading from `/proc/$p/smaps`.
- **Impact**: Provides accurate, requirement-aligned memory profiling for the desktop editor.

## 2. File Inventory

```json
{
  "id": "integration-014",
  "role": "integration",
  "create": [],
  "modify": [
    "tooling/measure_memory.sh"
  ],
  "read_only": [
    "spec/requirements.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: `tooling/measure_memory.sh` must start the desktop app under `xvfb-run`, sleep for 60 seconds to let it settle, then measure the private memory usage.
- **Compatibility constraints**: Keep using `/proc/$p/smaps` to calculate private memory and fallback to RSS if smaps is unavailable.
- **Architectural constraints**: Keep it as a simple bash script inside `tooling/`.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `bash tooling/measure_memory.sh` completes in under 3 seconds.
- **Verification commands**:
  - `bash tooling/measure_memory.sh`
- **Success criteria**: Script runs for at least 60 seconds and reports the private memory usage.
- **Edge cases**: Missing `desktop` executable, script interruption.

## 5. Constraints

- **Non-goals**: No changes to desktop code or other measurement scripts.
- **Forbidden changes**: Modifying application logic to artificially lower memory.
- **Scope ceiling**: Fits in one run; modifying a single bash script.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: `desktop` binary cannot be built.

## 7. Result
