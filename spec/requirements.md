# Requested future product, not observed current behavior

Evidence E-0001 asserts these requirements; none has been runtime-verified in this package. A Rust rewrite is an exploratory direction requested on 2026-10-14. Gavin subsequently answered “Yes for sure” when asked whether existing VS Code extensions must remain a requirement; D-0003 records that hard constraint. That request does not establish that every existing architectural constraint has been replaced.

## Functional contract; extension compatibility explicitly confirmed

An individual developer can open a local folder, navigate, edit/save, search, use tabs/splits, the Command Palette, settings/keybindings, themes/snippets, minimal Git, terminal/tasks, debugging/diagnostics/output/problems, file watching and extension management. Keep accessibility basics and controlled updates. Exact shortcuts, labels, ordering, persistence, error messages, encoding behavior and timing remain to be observed.

Ordinary local VSIX packages must support install/enable/disable/update/uninstall and the pinned stable public API, including Node extension execution, commands, tree views, storage, subprocesses, LSP, DAP and sandboxed webviews. Gavin reconfirmed existing VS Code extension compatibility as a hard requirement (D-0003). See extension-compatibility.md for 12 proposed contract-probe families and four existing-extension candidates; exact package versions are not yet qualified or tested. “Supports LSP” and “installs VSIX” do not individually establish that requirement.

Keep isolation between untrusted extensions/webviews and privileged application operations, workspace trust, narrow validated boundaries and secure signing/update behavior. Specific testable denial outcomes are unknown; a language choice cannot certify isolation.

Exclude bundled AI/chat/agents/voice/model/prompt/MCP surfaces, telemetry/experiments/promotions, account sign-in/sync/cloud/remote collaboration, and non-core feature packs except where a named compatibility fixture establishes necessity. Requested absence must be measured; it is not an assertion that the current product already achieves it.

## Performance and network acceptance requested by E-0001

Measure against an identified unmodified Code - OSS 1.133.0 reference on the same Mac and fixture, with clean profiles and at least 20 warm and 5 cold samples. Report full process-tree private memory, not one process. Cold/warm preparation and the tool-specific private-memory definition need to be fixed before comparing runs.

| Requested gate | Value |
| --- | --- |
| Warm launch improvement | At least 40% faster to accepted keystroke |
| Settled memory improvement | At least 35% lower after 60 seconds |
| Signed bundle reduction | At least 30% smaller |
| Warm launch p50 / p95 | ≤600 / ≤900 ms |
| Cold launch p50 / p95 | ≤1.2 / ≤1.7 s |
| Idle private memory | ≤250 MB; 200 MB stretch |
| Idle CPU median | ≤0.5% over five settled minutes |
| Typing/scroll/save/quick-open/search | No statistically significant regression; statistical design unresolved |
| Idle editor-owned network | Zero requests; explicit update/gallery actions and user-invoked extension-owned activity are separate cases |

No measured value exists in this package. The existing allowance for a measured safe Electron floor is specific to the old architecture; its meaning for a Rust replacement is an open decision. Security must not be weakened for an absolute number.

## Requested direction versus remaining decisions

The prior product strategy specified a small downstream patch stack over a pinned upstream. An independent Rust implementation is a different implementation strategy. Keep its product contract visible while Gavin chooses the new strategy. A first vertical slice can establish folder → edit → save → close/cancel/reopen behavior; it is not a replacement for the full alpha contract above.
