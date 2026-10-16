# Lean Rust editor Vision

This file is the sole desired-state authority. Gavin's new-repository instruction authorizes exploring an independent Rust implementation; the former downstream patch-stack strategy is superseded here. Imported `spec/` is read-only requirement and observation evidence, not a competing Vision. Unknown behavior remains unknown.

## Product promises

1. A local-first desktop code editor for one developer on Apple Silicon macOS 26.x: open folders, navigate, edit/save, search, tabs/splits, Command Palette, settings/keybindings, themes/snippets, minimal Git, terminal/tasks, debugging/diagnostics/output/problems, watching, accessibility basics and controlled updates.
2. Existing ordinary local VS Code extensions are a hard compatibility requirement: VSIX lifecycle and the pinned stable public API, including a Node extension host, commands, settings, tree views, storage, subprocesses, sandboxed webviews, LSP and DAP. A Rust core, VSIX installation or LSP/DAP support alone does not satisfy this promise. Version qualification and fixtures remain open work.
3. User data and security boundaries survive errors and interruption. Workspace trust and untrusted extension/webview isolation must be explicit and tested.
4. Exclude bundled AI/chat/agents/voice/model/prompt/MCP surfaces, telemetry/experiments/promotions, sign-in/sync/cloud collaboration and non-core feature packs except where a named compatibility fixture proves necessity.
5. Produce a real build and desktop launch, then measure startup, full process-tree private memory, network and artifact size. Retain the requested performance targets in `spec/requirements.md`; none is yet observed. The approved branded VS Code 1.133.0 interaction reference is distinct from the still-needed unmodified Code - OSS performance reference.

## Evidence and scope

The initial document-core slice established a small independently authored edit/save/cancel/reopen contract. It was a starting point, not the definition of a finished editor or a claim of observed VS Code parity. The imported catalog has 64 scenarios and zero directly observed current-product outcomes. Later evidence must identify the product, revision and environment actually exercised; completion of a plan or a synthetic fixture does not convert those unknowns into observations.

Use small testable plans and independent roles. Preserve lessons and negative experiments as evidence. No coordinator service, external database, hidden state or fabricated progress. Empty runs and empty PRs remain visible diagnostic outcomes; do not install no-op publication suppression.

## The experience we are working toward

A developer can choose a local project and do an ordinary edit-build-debug session without another editor carrying the missing work. The project, open documents, active selection, unsaved changes, diagnostics and running tools are understandable on screen. Keyboard input changes the text the developer sees at the intended location. Saving persists the intended document; cancelling a close preserves the session. Search, navigation and workspace changes remain useful as the project grows beyond one file. Failures explain what happened and leave a clear way to retry, recover or stop without losing work.

The application feels like a focused local tool. Core editing is available without an account, network service or extension download. Optional extension work does not hold basic input hostage. UI responsiveness, keyboard access, readable text, focus and accessible names are part of usability, not polish deferred beyond a nominally complete feature list. The exclusions above remain product boundaries; compatibility does not silently reintroduce bundled cloud or AI services.

A credible daily-driver candidate connects the promises together: the files visible in the desktop are the files safely edited and persisted by the core; extension commands and diagnostics reach the same live workspace; trust decisions govern the processes and webviews actually running; and the packaged application behaves like the tested build. Separate libraries, mock hosts and a window that receives key events are useful intermediate results. They do not by themselves establish that experience.

## Role outcomes within one Vision

These are desired states and standards of evidence, not work orders, a sequence, or prefilled queues. Each planner compares current Reality with this Vision and derives its own bounded work under the existing lifecycle and ownership rules. The role map remains the sole path authority. An outcome spanning roles does not grant any lane permission to edit another lane's files or state.

### Editor core — work survives ordinary mistakes and failures

The core gives the desktop dependable document semantics: text, edit locations, undo/redo history, dirty state and persisted state agree through editing, saving, save-as, cancellation, external changes and recovery. Supported text is preserved exactly unless the developer changes it. Unsupported inputs and conflicting disk changes fail explicitly without silently normalizing, truncating or replacing user data. A failed operation leaves a state the caller can understand and safely retry.

Recovery preserves a usable copy of unsaved work without treating an arbitrary path as disposable. Document destinations, recovery sources and unrelated files remain distinct even when different path spellings, symbolic links or hard links identify the same file. Ownership of a path is not inferred merely because it was supplied to a read operation. Successful recovery and later persistence must preserve the intended document and any copy whose disposal was not authorized. Retention and cleanup limits are explicit, including failures during backup creation and changes outside the process.

Acceptance evidence relates return values and in-memory state to bytes and entries on disk across complete operation sequences. It includes normal edits and deterministic failure cases with writable synthetic originals, not only permission-dependent tests or mocks. A green test of one method leaves a gap when its interaction with another method can lose data. Claims of crash recovery or durability need evidence for the stated failure model; syncing a file or preserving one buffer alone does not establish the whole promise.

### Desktop — an editor the developer can see and control

The desktop presents the actual document and makes the editing state visible: text, caret, selection, focus, dirty state, active file and errors agree with what commands will affect. A developer can open a folder, navigate among documents, edit at chosen positions, use tabs and splits, search and follow results, and save, cancel or recover work through discoverable controls. Unicode input, ordinary keyboard editing, scrolling and resizing remain coherent; a key handler that changes an invisible buffer is not a usable editing experience.

The Command Palette, keybindings and settings expose the same functioning actions as visible controls. Themes and snippets support the focused editing workflow. Minimal Git, terminal/tasks, diagnostics/problems/output and debugging have understandable states and cancellation paths. Core document handling remains usable when a peer capability is unavailable; the UI describes the real limitation instead of implying that an unwired action succeeded. Accessibility includes keyboard reachability, visible focus and meaningful exposed control state.

Acceptance includes reproducible interaction with the built application on the target Mac: what was visible before input, the action taken, the resulting UI and persisted bytes, and behavior when a flow is cancelled or fails. Rendering, hit testing, focus and integration need runtime evidence in addition to pure state tests. Human judgment still determines whether the result is clear and pleasant to use. A launched window, title update, compiled event loop or mocked interaction alone leaves the connected editing outcome open.

### Extensions — ordinary extensions work in the real editor

The compatibility promise is useful existing local VS Code extensions operating against a declared stable API version, not just accepting their manifests. A developer can install, enable, disable, update and uninstall a qualified local VSIX and understand compatibility or permission failures. Enabled extensions activate in the actual Node host under their declared conditions, contribute working commands and configuration, present tree views and sandboxed webviews, retain appropriate state, and use supported subprocess, language-server and debug-adapter capabilities. Disabling, removing or failing an extension has predictable effects on the running application and its stored data.

Extension-visible workspace and document behavior matches the editor the user is operating. Trust boundaries apply to real host, process and webview operations; invalid messages, rejected capabilities and host faults cannot silently acquire privilege or corrupt the editing session. Unsupported APIs fail precisely. A missing non-excluded stable API remains a compatibility gap; a precise rejection or passing fixture subset does not satisfy the full promised contract. An extension's dependency on an excluded product service is explicit and never silently enables that service. Where compatibility and exclusion conflict, the affected behavior remains unresolved pending the existing human-controlled product decision. Version and package qualification remain explicit human-controlled decisions under the existing rules.

Acceptance distinguishes manifest parsing, protocol tests, authored probes and execution of named, versioned packages in the real host. Evidence covers lifecycle transitions and representative public API behavior, including denied and failed operations, and identifies the build and supported API/package versions. A stubbed `vscode` module can validate a probe's own logic; it cannot establish this editor's compatibility. Parsing a package, an LSP/DAP connection or a collection of independently green mocks leaves the host-to-desktop contract open.

### Quality — confidence follows observations, not labels

A reader can tell which promises were observed in the independent editor, which were observed in an identified reference, which have only synthetic or mocked evidence, and which remain blocked or unknown. The evidence spans connected editing and recovery, extension lifecycle and public API behavior, trust denials and host failure, accessibility basics, and the excluded surfaces. Failures and surprising outcomes remain available for later reasoning; an administrative completion does not erase a missing observation.

Compatibility evidence names the fixture or qualified package, version, environment, exact actions and expected and actual results. A fixture running solely against its own substitute host is explicitly a fixture self-test. A comparison is credible only when the claimed systems were exercised under comparable conditions. Reference discovery stays separate from independent implementation inputs: externally observable contracts and safe synthetic fixtures may cross that boundary; original implementation, tests, assets and source-aware notes do not.

Performance and network confidence comes from raw samples and an explained method on the target Mac. The requested gates in `spec/requirements.md` remain the goals: accepted-keystroke startup, full process-tree private memory, settled CPU, signed bundle size, interaction latency and idle editor-owned network. The baseline, warm/cold preparation, sample count and attribution are explicit. Process existence or window-title appearance is not accepted input; RSS of selected processes is not full-tree private memory; a syscall pattern is not proof of zero network traffic. An unavailable reference, denied instrument or resource limit remains a visible limitation, not a zero or a pass.

### Integration — the tested product is the product a person can run

The repository can produce an identifiable application for Apple Silicon macOS from a documented revision and toolchain. Its artifact contains the needed desktop, core and extension runtime pieces and starts as the intended product outside the development checkout. Build, validation and packaging results identify what was actually built and exercised. Licensing, notices, provenance and signing/update requirements are explicit; an unsigned development binary is useful evidence but is not presented as an approved signed release.

Validation reliably exposes broken owned components and their integration, and its fixtures remain meaningful as planners add or complete work. Passing checks correspond to the claimed behavior; a script merely existing, swallowing an error or testing a stand-in is not a successful product check. Product and release documentation accurately describe available capabilities, unresolved gaps and reproducible use. A failure in another lane remains attributed to that owner, without editing peer state or declaring the whole product complete because local tooling is done.

Measurement and packaging support the same acceptance definitions used by quality. A measurement result identifies the artifact, machine, workload, process scope and raw observations; missing tools, unsupported platforms and failed launches are explicit. The desired result is a reproducible build, real launch and trustworthy baseline from which performance work can be judged. Adding scripts, shrinking a binary or obtaining one successful command does not by itself close that outcome. Publication, signing, deployment and changes to automation still follow the existing approval boundaries.

## Recognizing remaining gravity

An empty execution queue says that no work is currently assigned; it does not establish product completeness. Completed work orders remain evidence of their bounded scope and are never reset merely to keep a lane busy. A role's next gap can be a missing behavior, a disconnected user journey, an untested failure boundary, an unsupported compatibility claim, or evidence too weak to support a claimed result. Planners judge these against current code and durable observations rather than repeating an old feature checklist or taking a status heading as proof.

Dependencies remain local, inspectable repository evidence associated with the actual owning lane. A missing peer capability is a boundary to describe, not permission to implement it elsewhere or to fabricate a successful integration. Existing claims and recoverable blockers retain their lifecycle protections. When no safe eligible gap can be planned, a successful unchanged run remains correct. Richer desired outcomes do not impose a quota of work or suppress empty PRs.

The architecture is successful when repeated bounded work makes the connected user experience and its evidence stronger while preserving these boundaries. Repository structure, plan counts and merged PR counts alone cannot demonstrate convergence. Readiness for real users remains Gavin's product decision.

## Human-controlled decisions

Only Gavin may change this Vision or the locked governance. Remote repository visibility, publication, extension package qualification, release signing and broader product-contract changes require their respective decisions. This scaffold installs no scheduler and changes no external task cadence.
