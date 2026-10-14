# Lean Rust editor Vision

This file is the sole desired-state authority. Gavin's new-repository instruction authorizes exploring an independent Rust implementation; the former downstream patch-stack strategy is superseded here. Imported `spec/` is read-only requirement and observation evidence, not a competing Vision. Unknown behavior remains unknown.

## Product promises

1. A local-first desktop code editor for one developer on Apple Silicon macOS 26.x: open folders, navigate, edit/save, search, tabs/splits, Command Palette, settings/keybindings, themes/snippets, minimal Git, terminal/tasks, debugging/diagnostics/output/problems, watching, accessibility basics and controlled updates.
2. Existing ordinary local VS Code extensions are a hard compatibility requirement: VSIX lifecycle and the pinned stable public API, including a Node extension host, commands, settings, tree views, storage, subprocesses, sandboxed webviews, LSP and DAP. A Rust core, VSIX installation or LSP/DAP support alone does not satisfy this promise. Version qualification and fixtures remain open work.
3. User data and security boundaries survive errors and interruption. Workspace trust and untrusted extension/webview isolation must be explicit and tested.
4. Exclude bundled AI/chat/agents/voice/model/prompt/MCP surfaces, telemetry/experiments/promotions, sign-in/sync/cloud collaboration and non-core feature packs except where a named compatibility fixture proves necessity.
5. Produce a real build and desktop launch, then measure startup, full process-tree private memory, network and artifact size. Retain the requested performance targets in `spec/requirements.md`; none is yet observed. The approved branded VS Code 1.133.0 interaction reference is distinct from the still-needed unmodified Code - OSS performance reference.

## Evidence and scope

The first slice is an intentionally scoped, dependency-free Rust document core with synthetic edit/save/cancel/reopen tests. It is a new requested contract, not a claim of observed VS Code parity. The imported catalog has 64 scenarios and zero directly observed current-product outcomes. A green unit suite is not a running desktop editor, extension compatibility or a performance baseline.

Use small testable plans and independent roles. Preserve lessons and negative experiments as evidence. No coordinator service, external database, hidden state or fabricated progress. Empty runs and empty PRs remain visible diagnostic outcomes; do not install no-op publication suppression.

## Human-controlled decisions

Only Gavin may change this Vision or the locked governance. Remote repository visibility, publication, extension package qualification, release signing and broader product-contract changes require their respective decisions. This scaffold installs no scheduler and changes no external task cadence.
