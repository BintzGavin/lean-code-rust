# Lean Rust editor Vision

Build a local code editor whose first priorities are performance, a low memory footprint, and immediate access to raw file views and diffs. The developer should reach useful, visible file content and inspect changes with as little waiting as possible; opening a window alone is not success. The enduring ambition is to shorten the distance between wanting to inspect or change code and understanding it, without sacrificing trust or control.

This is an enduring aspiration. No release, feature catalogue or passing test suite exhausts it. As capabilities become real, their use together reveals deeper questions of usefulness, clarity, reliability, performance and design. Completing a feature establishes a stronger starting point for the next comparison with this ideal. Progress may mean new capability, a more coherent experience, stronger guarantees or a simpler architecture.

`VISION.md` remains the sole desired-state authority. The direction is an independent Rust implementation, superseding the former downstream patch-stack strategy. Existing product commitments remain obligations; they are foundations for improvement, not the limit of legitimate work.

## An editor that grows with its use

The developer can understand, change and diagnose a project through one coherent workspace. Fluent keyboard use, accessibility and discoverable actions make increasing capability feel natural. Everyday work and difficult cases reveal where friction, uncertainty or disconnected behavior still stand between intention and result.

Trust deepens as work survives mistakes, interruption and conflicting changes. Recovery preserves user choices, document destinations, borrowed recovery sources and unrelated data, including when paths alias. Security boundaries contain misbehaving extensions. Failures remain understandable and recoverable.

Performance, low memory use and speed to useful file and diff content lead architectural tradeoffs and autonomous gap selection. Capability earns its cost across startup and sustained use as files, projects and changes grow. Architectural improvement should reduce waiting, retained memory and accidental complexity while preserving correctness, safety and real extension compatibility. Smaller numbers obtained by hiding content, dropping promised behavior or weakening guarantees do not establish progress.

## High-level architecture

A Rust document core supplies dependable editing and persistence semantics. A native desktop presents and operates that same state, so text, history, files and recovery agree across complete user actions. An isolated Node extension host connects existing VS Code extensions to the live workspace through explicit public contracts and enforced trust boundaries.

Build, packaging and observation bring these components together as one identifiable application. The product people run is the product whose behavior is described and measured. Internal designs can evolve toward these relationships; this Vision prescribes no algorithms, libraries or implementation sequence.

## Commitments and confidence

The target remains a local-first editor for one developer on Apple Silicon macOS 26.x, with core editing available without an account, network service or extension download. The full functional, security, exclusion and performance commitments in `spec/requirements.md` remain in force. This includes its restrictions on bundled AI, telemetry and cloud/account features and its rules for compatibility exceptions. Real VS Code extension compatibility remains a hard requirement: the VSIX lifecycle and pinned stable public API described in `spec/extension-compatibility.md`, not merely parsing packages, installing them or supporting LSP/DAP. Missing non-excluded stable APIs remain gaps. Qualification and conflicts between compatibility and exclusions retain their existing human-controlled decisions.

Evidence identifies what actually ran. Product observations, reference observations, fixture self-tests and unknown outcomes stay distinct; mock success never establishes product compatibility. Safety claims cover the relevant operation sequences and failure model. Performance claims retain raw observations and the measurement definitions in `spec/requirements.md`; the branded interaction reference remains distinct from the unmodified Code - OSS performance reference. Failures and reusable lessons remain evidence. Human judgment complements tests when judging usefulness and clarity.

## Autonomous discovery

Black Hole Architecture lets planners discover consequential, testable gaps between this aspiration and current Reality; executors implement bounded plans. Completed plans and finite scenarios do not exhaust the Vision. Real use, interactions between capabilities and architectural limitations continue to offer meaningful questions after today's features work.

`AGENTS.md` and `.sys/black-hole/role-map.md` retain their existing clean-room information boundary, disjoint ownership, repository-owned state, lifecycle and authority rules. No coordinator or hidden control state is introduced. Only Gavin changes the Vision or locked governance; existing publication, qualification and other human-controlled decisions remain intact.

A continuing aspiration does not require manufactured work or arbitrary churn. A run without a safe eligible gap can finish unchanged without creating a no-op work order. Plans can be complete and releases ready for people while the aspiration remains open.
