# integration-011 — Measure Settled CPU

**Role**: `integration`
**Backlog entry**: `.sys/backlogs/integration.md#integration-011`

## 1. Context & Goal

- **Objective**: Create a script to measure idle CPU usage of the desktop process.
- **Vision gap**: `VISION.md` requests measurement of "settled CPU" (Idle CPU median ≤0.5% over five settled minutes), but there is currently no tooling to measure CPU activity.
- **Evidence**: `tooling/` contains `measure_memory.sh`, `measure_startup.sh`, and `measure_network.sh`, but no script for CPU.
- **Impact**: Unlocks the ability to observe the idle CPU median performance gate for the product.

## 2. File Inventory

```json
{
  "id": "integration-011",
  "role": "integration",
  "create": [
    "tooling/measure_cpu.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/requirements.md",
    "VISION.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: A new script `tooling/measure_cpu.sh` that launches the desktop application (using `xvfb-run` to satisfy the display requirement), allows it to settle, and then samples its CPU usage over a defined period, reporting the median or average CPU usage.
- **Compatibility constraints**: Must work in headless CI/CD environments (e.g., using `xvfb-run`).
- **Architectural constraints**: Retain existing Bash scripting patterns and explicitly handle process cleanup (killing the application) when the measurement is done or if it crashes.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `bash tooling/measure_cpu.sh` currently fails (file does not exist).
- **Verification commands**:
  - `bash packaging/build_release.sh`
  - `bash tooling/measure_cpu.sh`
- **Success criteria**: The script successfully runs, outputs a CPU measurement, and correctly terminates the desktop process without leaving stray background jobs.
- **Edge cases**: The application crashes during measurement; the script is interrupted.

## 5. Constraints

- **Non-goals**: Optimizing the actual CPU usage of the desktop application or running for a full 5 minutes in standard tests (the script can accept a duration parameter or use a shorter default for verification).
- **Forbidden changes**: Do not modify product code (`crates/desktop/` or `crates/editor-core/`).
- **Scope ceiling**: Adding a single measurement script matches the scale of previous `measure_*.sh` tasks and fits within one execution run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
