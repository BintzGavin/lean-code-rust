# integration-015 — Measure startup percentiles (p50/p95)

**Role**: `integration`
**Backlog entry**: `.sys/backlogs/integration.md#integration-015`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Measure and report startup time p50 and p95 across 20 warm and 5 cold samples.
- **Vision gap**: `spec/requirements.md` dictates "Warm launch p50 / p95" and "Cold launch p50 / p95" metrics over 20 warm and 5 cold samples, but `tooling/measure_startup.sh` currently runs just one iteration and doesn't compute percentiles or distinguish warm/cold runs.
- **Evidence**: `tooling/measure_startup.sh` executes the desktop process once. `spec/requirements.md` requires p50 and p95 across 20 warm and 5 cold samples.
- **Impact**: Provides accurate, requirement-aligned startup profiling that distinguishes between warm and cold runs.

## 2. File Inventory

```json
{
  "id": "integration-015",
  "role": "integration",
  "create": [],
  "modify": [
    "tooling/measure_startup.sh"
  ],
  "read_only": [
    "spec/requirements.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: `tooling/measure_startup.sh` must execute the desktop application 25 times total. The first 5 runs are cold (cache clearing via `sync; echo 3 > /proc/sys/vm/drop_caches` before each run, requiring sudo access if possible, or failing gracefully and printing a warning if not running as root). The next 20 are warm. It must calculate and report the p50 and p95 for both cold and warm runs.
- **Compatibility constraints**: Keep outputting clear millisecond measurements.
- **Architectural constraints**: Keep it as a bash script.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `bash tooling/measure_startup.sh` only outputs a single "Startup time: X ms" value.
- **Verification commands**:
  - `bash tooling/measure_startup.sh`
- **Success criteria**: Output includes p50 and p95 for cold (5 samples) and warm (20 samples) runs.
- **Edge cases**: Missing `desktop` executable, script interruption, missing `bc` or `awk` utilities, permissions for cache dropping.

## 5. Constraints

- **Non-goals**: No changes to desktop code or other measurement scripts.
- **Forbidden changes**: Modifying application logic.
- **Scope ceiling**: Fits in one run; modifying a single bash script.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: `desktop` binary cannot be built.

## 7. Result
