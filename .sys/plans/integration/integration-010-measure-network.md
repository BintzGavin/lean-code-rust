# integration-010 — Measure idle network activity

**Role**: `integration`
**Backlog entry**: `.sys/backlogs/integration.md#integration-010`

## 1. Context & Goal

- **Objective**: Create a script to measure idle network activity of the desktop application.
- **Vision gap**: VISION.md requires measuring network activity alongside startup, memory, and artifact size, but no network measurement script exists.
- **Evidence**: `tooling/` contains scripts for memory and startup measurement, but none for network. `spec/requirements.md` mandates "Idle editor-owned network: Zero requests".
- **Impact**: Enables automated verification that the local-first application makes no unauthorized external network requests while idle.

## 2. File Inventory

```json
{
  "id": "integration-010",
  "role": "integration",
  "create": [
    "tooling/measure_network.sh"
  ],
  "modify": [],
  "read_only": [
    "VISION.md",
    "spec/requirements.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: `tooling/measure_network.sh` must launch the desktop application (e.g., using `xvfb-run`), monitor its network activity (e.g., using `strace -e trace=network` or checking network namespaces), and report whether any unexpected network requests were made over a short idle period (e.g., 5 seconds).
- **Compatibility constraints**: Must run in a standard Linux headless environment (like CI) without modifying the product code.
- **Architectural constraints**: Script should reside in `tooling/` and be executable.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `ls tooling/measure_network.sh` fails.
- **Verification commands**:
  - `bash packaging/build_release.sh`
  - `bash tooling/measure_network.sh`
- **Success criteria**: The script successfully executes, runs the app, monitors network activity, and outputs the result (e.g., "Network requests: 0").
- **Edge cases**: The application crashes before the measurement period ends.

## 5. Constraints

- **Non-goals**: Modifying the application to fix any discovered network activity; testing extension-owned network traffic.
- **Forbidden changes**: Modifying source code in `crates/` or changing dependencies in `Cargo.toml`.
- **Scope ceiling**: Confined to writing a single bash script for external observation.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: The environment lacks necessary tools (like `strace` or network monitoring utilities) and they cannot be installed.

## 7. Result
