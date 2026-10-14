# Uncertainties, frontier and change decisions

| ID | Known facts / missing evidence or decision | Smallest next action | Consequence |
| --- | --- | --- | --- |
| U-0001 | No runnable identified Lean product observed. Local build attempt previously stopped before compilation because build dependencies were absent. | Obtain an existing identified Lean artifact, or schedule a resource-budgeted build later; do not transfer branded-reference results to Lean. | All64 scenario cells lack an original-product oracle. |
| U-0002 | Requirements describe broad behavior, not exact current defaults, labels, errors, ordering or durability. | Execute S-0001 and S-0037 on an identified disposable profile; retain the linked traces. | Exact parity is blocked even if a replacement implements the desired happy path. |
| U-0003 | Existing VS Code extension compatibility is confirmed as hard requirement D-0003. Twelve contract-probe families and four existing-extension candidates are now proposed; exact package versions and the complete10–15package qualification matrix remain unselected. | Name/version/license the fixture matrix, including command/tree/storage, language tooling, debugger, subprocess and webview coverage. | Full compatibility cannot be estimated or claimed from LSP/DAP or install success. |
| U-0004 | Rust rewrite direction differs from prior downstream-patch strategy. | Compatibility is settled by D-0003. Select a compliant core/UI/runtime strategy and record how it replaces the old patch-stack constraint; do not reduce compatibility. | Implementation strategy remains open; no source copying or compatibility reduction is authorized. |
| U-0005 | macOS first is explicit; native UI toolkit and exact visual fidelity are unspecified. | Decide whether classic workflows/keyboard/accessibility are sufficient or historical pixel fidelity is required. | Affects UI selection and visual evidence required; no toolkit committed. |
| U-0006 | No measured baseline or signed artifact; cold start, private-memory measurement and statistics are underspecified. | Fix a reproducible measurement protocol, then collect original raw samples on a quiet reference Mac. | Performance targets are goals, not forecasted gains from Rust. |
| U-0007 | Branded VS Code1.133.0 is installed but is a different product baseline. | V-0002 selected for two bounded reference variants under D-0004; record separate runs/configuration and resume after UI unlock. | Useful observations cannot prove Lean behavior or unmodified Code-OSS performance. |
| U-0008 | No current visual/accessibility trace; R-0002 native UI preflight again reported the Mac locked before product launch. | With UI access restored, record keyboard focus and representative safe screenshots on a disposable profile. | Visual/accessibility parity remains unknown; no screenshots supplied. |
| U-0009 | Update/signing, marketplace choice and package delivery behavior are unspecified for an independent replacement. | Specify controlled local update failure and supported local VSIX sources before any live service interaction. | Do not assume permission or ability to reuse branded services. |
| U-0010 | Package separation is procedural; source-aware analyst produced the catalog. | Fresh implementation context receives only this directory, with no source or discovery mounts. | Cannot call this analyst source-isolated or claim legal certification. |

## Discovery frontier

| ID | Candidate / source | Next probe / disposition |
| --- | --- | --- |
| F-0002 | Encodings/EOL/Unicode beyond simple LF; fixture offers CRLF and Unicode seeds | Add separately identified cells for round-trip, BOM, invalid bytes and normalization; unobserved |
| F-0003 | Permissions/symlinks/multi-root/external mutation; file-edit workflow | Read-only/missing target and denied save probes; external conflict seeded S-0038; remaining combinations open |
| F-0004 | IME, screen reader, high contrast, reduced motion; accessibility requirement | Establish input/accessibility matrix after keyboard seed; all unobserved |
| F-0005 | Extension versions/combinations/native modules and failure modes; VSIX requirement | Select fixtures and pin stable public API contract; verify package rejection and host crashes separately |
| F-0006 | Multi-window/restart/crash and large files; editor lifecycle | Expand after continuous save/interruption oracle; not satisfied by two fresh successful sessions |
| F-0007 | Excluded features versus retained compatibility requirements | Per-fixture product decisions; preserve actual unsupported behavior and errors |
| F-0008 | Public CLI/file configuration/import interfaces; desktop editor entry points | Inventory supported commands, flags, exit channels, schema versions and persistence through public help/UI |
| F-0009 | Layout/fonts/icons/palette/menu exhaustive inventory | Capture representative states and reconcile controls to catalog; initial20areas do not close the frontier |

## Decision log

D-0001,2026-10-14: User requested clean-room Rust rewrite exploration. Scope in this package is assessment/cataloging. Existing product/repository and earlier local repair experiments remain separate. No rewrite implementation or strategy revision is represented as complete.

D-0002, pending: choose exact first implementation milestone after the first interaction oracles and mandatory VSIX compatibility risks are understood. Recommended candidate is open folder → edit → save → cancel-close → reopen, with a separately evaluated extension-compatibility proof before treating it as an alpha. This is a proposal, not evidence of approval or current product behavior.

D-0003, confirmed2026-10-14: Existing VS Code extensions must remain a requirement for the Rust direction. No LSP/DAP-only scope reduction is accepted. Node and isolated webview compatibility remain required capabilities; runtime implementation reuse and toolkit choice are not authorized by this confirmation. Added S-0041–S-0064 for twelve contract-probe families, preserving all earlier IDs.

U-0011: Current public documentation is not a pinned complete1.133.0 API oracle. Establish the versioned public contract, fixture engine ranges and dependency versions before execution. This blocks exact API compatibility claims, not the requirement itself.

F-0010: Qualify four existing-extension candidates and identify webview/tree-heavy candidates; add explicit cells before execution. Their current candidacy is not included as executed evidence in the64-cell denominator.

D-0004,2026-10-14: Select installed branded VS Code1.133.0 as a separately identified reference for bounded core behavior discovery within the authorized assessment. This operational reference selection does not reduce product scope, replace the Lean baseline or establish Code-OSS performance. Initial native UI check blocked before launch; manually unlocking the Mac is the required next environmental step.
