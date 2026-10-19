# INTEGRATION-012 — Measure process-tree private memory instead of RSS

**Role**: `integration`
**Backlog entry**: `.sys/backlogs/integration.md#integration-012`

## 1. Context & Goal

- **Objective**: Update `tooling/measure_memory.sh` to report process-tree private memory instead of total RSS.
- **Vision gap**: `spec/requirements.md` explicitly mandates measuring "full process-tree private memory, not one process," but `tooling/measure_memory.sh` currently measures total RSS, which includes shared memory and overestimates actual private usage.
- **Evidence**: `tooling/measure_memory.sh` currently sums `ps -o rss= -p $p`, which measures resident set size including shared libraries. `spec/requirements.md` requires "Report full process-tree private memory".
- **Impact**: Enables accurate memory usage tracking aligned with product performance constraints, preventing false regression failures due to shared memory.

## 2. File Inventory

```json
{
  "id": "integration-012",
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

- **Required behavior**: `tooling/measure_memory.sh` must parse `/proc/$p/smaps` or use an equivalent accurate method (like `pmap -x`) to report total private memory (Private_Clean + Private_Dirty) for the desktop process and its children.
- **Compatibility constraints**: The script output must maintain a parsable format: `Memory: X.XX MB`.
- **Architectural constraints**: Only use standard Unix utilities available on GitHub Actions / Ubuntu.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Current output shows RSS, not private memory.
- **Verification commands**: `bash tooling/measure_memory.sh`
- **Success criteria**: The script outputs a memory value in MB, which should be lower or equal to the previous RSS measurement.
- **Edge cases**: Missing `/proc/$p/smaps` (graceful fallback/error), zombie processes.

## 5. Constraints

- **Non-goals**: Optimizing memory usage of the desktop binary.
- **Forbidden changes**: Do not modify `Cargo.toml` or actual product code.
- **Scope ceiling**: Updating a single shell script for measurement accuracy.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: Linux kernel does not expose `smaps` and no standard utility can measure private memory accurately.

## 7. Result
- Completed: Updated measure_memory.sh to report process-tree private memory instead of RSS.
