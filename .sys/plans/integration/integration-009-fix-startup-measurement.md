# integration-009 — Update startup measurement to use xdotool

**Role**: `integration`
**Backlog entry**: `.sys/backlogs/integration.md#integration-009`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Update the startup measurement script to reliably poll for window creation using xdotool.
- **Vision gap**: The Vision requires measuring startup performance. The current strace-based approach is unreliable for observing the exact moment the winit UI window is created.
- **Evidence**: `tooling/measure_startup.sh` uses `strace -e poll,epoll_wait,epoll_pwait` to guess when the app is ready, rather than verifying window creation.
- **Impact**: Unlocks accurate measurement of UI startup time for the winit-based desktop application in a headless environment without modifying product code.

## 2. File Inventory

```json
{
  "id": "integration-009",
  "role": "integration",
  "create": [],
  "modify": [
    "tooling/measure_startup.sh"
  ],
  "read_only": [
    "VISION.md",
    "spec/requirements.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: `tooling/measure_startup.sh` must run the application under `xvfb-run` and use `xdotool search --onlyvisible --name "Lean Editor"` in a loop to accurately poll for the window's creation. Once found, it calculates the startup time in milliseconds and safely terminates the process.
- **Compatibility constraints**: Must work on a headless local development environment with `xvfb` and `xdotool` installed.
- **Architectural constraints**: Measurement must be done externally without modifying product code.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Run `bash tooling/measure_startup.sh` and observe it relies on unreliable system call traces instead of window visibility.
- **Verification commands**:
  ```sh
  bash packaging/build_release.sh
  bash tooling/measure_startup.sh
  ```
- **Success criteria**: The script successfully prints a startup time in milliseconds using `xdotool` to verify window creation.
- **Edge cases**: Handles process crash or window never appearing gracefully with a timeout.

## 5. Constraints

- **Non-goals**: Optimizing startup time, network measurement, or memory measurement.
- **Forbidden changes**: Do not modify product code in `crates/` or the build script.
- **Scope ceiling**: Modifying a single bash script fits within one execution run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: The desktop application cannot be launched or profiled automatically.

## 7. Result
