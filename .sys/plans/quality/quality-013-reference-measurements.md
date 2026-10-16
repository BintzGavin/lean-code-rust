# quality-013 — Reference measurement scripts

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-013`

## 1. Context & Goal

- **Objective**: Implement reference measurement scripts to observe startup time, process-tree memory, and network activity.
- **Vision gap**: `spec/requirements.md` requires measurement of warm/cold launch, memory footprint, and network requests, but `tests/reference/` tooling does not yet exist.
- **Evidence**: `tests/reference/` path is absent. Vision priorities state "Observe parity and startup, process-tree memory and network before making claims."
- **Impact**: Provides independent, testable measurement tools for evaluating product architecture and establishing the baseline performance.

## 2. File Inventory

```json
{
  "id": "quality-013",
  "role": "quality",
  "create": [
    "tests/reference/measure_startup.sh",
    "tests/reference/measure_memory.sh",
    "tests/reference/measure_network.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/requirements.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**:
  - `measure_startup.sh`: Script to measure UI startup time. It should handle headless environments (e.g., using `xvfb-run` and `xdotool search --onlyvisible` to poll for window creation).
  - `measure_memory.sh`: Script to capture the full process-tree private memory footprint of a given PID.
  - `measure_network.sh`: Script to trace network activity of an application (e.g., using `strace -e trace=network` filtered for `AF_INET` and `AF_INET6` to exclude local socket communication).
- **Compatibility constraints**: Scripts must run in bash on standard Linux environments and gracefully exit with usage instructions if arguments or dependencies (e.g., xdotool) are missing.
- **Architectural constraints**: These are independent observation tools. They do not modify or depend on internal application state.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `ls tests/reference/` fails.
- **Verification commands**:
  `bash tests/reference/measure_startup.sh --help`
  `python3 tooling/check_scaffold.py --role quality`
- **Success criteria**: The scripts exist, are executable, and pass the scaffold check.
- **Edge cases**: Scripts should print usage instructions if required arguments like PID or executable are missing.

## 5. Constraints

- **Non-goals**: Generating the actual `evidence/` data for the application or Code - OSS in this run.
- **Forbidden changes**: Modifying product crates, specs, or other role files.
- **Scope ceiling**: Bounded to creating the reference measurement shell scripts.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
