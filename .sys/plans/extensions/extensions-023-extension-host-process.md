# [extensions-023] — Implement Extension Host Process Manager

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-023`

## 1. Context & Goal

- **Objective**: Implement the `ExtensionHost` process manager to spawn and manage isolated Node extension host processes.
- **Vision gap**: X-0001 Node host lifecycle: The system currently lacks the ability to execute extensions in an isolated Node host environment.
- **Evidence**: `crates/extensions/src/lib.rs` currently only parses manifests and manipulates VSIX packages, but provides no facility to run Node extension hosts.
- **Impact**: Enables executing real VS Code extension code within a secure boundaries and communicating with it.

## 2. File Inventory

```json
{
  "id": "extensions-023",
  "role": "extensions",
  "create": [
    "crates/extensions/src/host.rs"
  ],
  "modify": [
    "crates/extensions/src/lib.rs",
    "crates/extensions/Cargo.toml"
  ],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**:
  - Define an `ExtensionHost` struct representing the spawned Node extension host.
  - Provide a method to spawn a Node child process executing a given entry point.
  - The process must be isolated with controlled inputs and outputs (e.g., stdin/stdout/stderr pipes).
  - Handle basic lifecycle events: starting the host, observing its exit status, and gracefully shutting it down.
- **Compatibility constraints**: The Node host must execute standard extension entry points as expected by VS Code.
- **Architectural constraints**: Maintain strict isolation. The Rust `extensions` crate manages the process boundary, while untrusted code runs in Node.
- **Public contract changes**: New `host` module exposed in `crates/extensions/src/lib.rs`.

## 4. Test Plan

- **Failing evidence**: Currently, attempting to spawn an `ExtensionHost` will fail to compile as the type and module do not exist.
- **Verification commands**:
  - `cargo check --manifest-path crates/extensions/Cargo.toml`
  - `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: `ExtensionHost` is defined, compiles successfully, and tests verify a basic Node process can be spawned and terminated (e.g., using a trivial synthetic script).
- **Edge cases**: Node not installed/in PATH, missing entry point script, unexpected early termination of the Node process.

## 5. Constraints

- **Non-goals**: Implementing full JSON-RPC communication, full Extension API stubs, or complex message handling in this step.
- **Forbidden changes**: Do not modify existing manifest parsing or VSIX lifecycle logic.
- **Scope ceiling**: Strictly bounded to managing the lifecycle (spawn, terminate) of the external Node process.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: `node` executable is not available in the testing environment, preventing reliable test execution.

## 7. Result
- Successfully implemented the ExtensionHost process manager for isolated Node hosts.
