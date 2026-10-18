# [extensions-024] — Implement Node Host IPC

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-024`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Implement basic Inter-Process Communication (IPC) for the `ExtensionHost`.
- **Vision gap**: X-0001 Node host lifecycle: "Manifest `main`, activation, context subscriptions, disposal" and "read synthetic file / activation exception". Currently we can start the Node process but cannot communicate with it.
- **Evidence**: `crates/extensions/src/host.rs` manages the process but has no mechanism to send or receive structured messages from `stdin`/`stdout`.
- **Impact**: Establishes the communication layer needed for the editor to send lifecycle commands (e.g., activation) to the extension host and receive responses.

## 2. File Inventory

```json
{
  "id": "extensions-024",
  "role": "extensions",
  "create": [],
  "modify": [
    "crates/extensions/src/host.rs",
    "crates/extensions/Cargo.toml"
  ],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Extend `ExtensionHost` to allow sending a string message to the Node process via `stdin` and receiving string messages from the Node process via `stdout`.
- **Compatibility constraints**: The communication mechanism must be compatible with Node.js standard I/O streams.
- **Architectural constraints**: Maintain strict isolation; message contents are treated as strings at this layer (serialization/deserialization into complex types is a non-goal for this basic IPC step).
- **Public contract changes**: New methods on `ExtensionHost` to send and receive messages.

## 4. Test Plan

- **Failing evidence**: Currently, `ExtensionHost` lacks methods to send or receive messages.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: Tests verify that a string can be sent to a synthetic Node script and a response string can be received.
- **Edge cases**: Node process exits prematurely before responding; large messages.

## 5. Constraints

- **Non-goals**: Full JSON-RPC protocol implementation; defining specific extension API message types.
- **Forbidden changes**: Do not modify existing manifest parsing or VSIX lifecycle logic.
- **Scope ceiling**: Strictly bounded to sending/receiving strings over standard I/O pipes.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
