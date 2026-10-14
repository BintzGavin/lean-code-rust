# Lean editor independent behavior package

Checkpoint: 2026-10-14. Status: **partial documentation-derived catalog; not an observed parity oracle or an implemented editor**.

Lean is a local-first desktop code editor for one developer, with a deliberately small interface and ordinary local VSIX extension support. Its requested first platform is Apple Silicon macOS 26.x. This package separates the requested future product from the behavior of the existing product. Unknown current behavior is not an invitation to invent parity.

Start with [scope and baseline](scope.md), then [product requirements](requirements.md), the [hard extension-compatibility contract and fixture matrix](extension-compatibility.md), the canonical [catalog](catalog.json), [deep interaction cases](interaction-cases.md), [evidence](evidence.md), [gaps and decisions](gaps.md), and [parity procedure](parity.md). The catalog owns behavior/scenario IDs and statuses. Other documents explain those same records; they do not independently certify them.

An independent implementer may use only this directory, public interface documentation identified here, and independently selected dependencies. Do not load the original repository, analyst notes, or source-aware conversation history. No original implementation, original test source, assets, or translated internal algorithms are supplied. This is a technical information boundary, not a legal certification.

**Starting point:** use the supplied synthetic fixture to prepare the B-0001 edit/save and B-0019 interruption probes. Obtain current-product observations before claiming parity. A future intentionally different behavior needs a recorded decision. Implementation may begin only under a separate scoped implementation instruction; the cataloging skill itself does not build the replacement.

## Vocabulary

- Workspace: a local folder opened by the user. Initial fixture is a single folder, not a multi-root workspace.
- Document: text shown in an editor tab; disk bytes and unsaved buffer state are separate observables.
- Dirty: visible indication that a document has unsaved changes; exact presentation is unobserved.
- Profile: independent application settings and extension state used for a run; no personal profile is supplied.
- VSIX: an installable extension package; its installation alone does not prove its API behavior works.
- Stable API: the versioned public `vscode` extension interface associated with the requested 1.133.0 compatibility baseline.
- Oracle: captured expected current-product result for a concrete scenario. A product wish or a successful replacement test is not an oracle.

## Coverage at this checkpoint

20 cataloged behavior areas; 64 declared scenario cells; **0/64 directly observed**, 0 product tests, 0 model executions. All 64 cells are blocked on current-product observation and some also require missing fixtures or decisions. Requirements and public documentation supply candidate contracts only. No full-product, accessibility, visual, performance, security, or VSIX parity claim is made.

Run `python3 checks/check_package.py` for package/fixture integrity. It does not launch or validate either editor.

See [verification](verification.md) for the independent handoff review, repaired procedure findings and the exact validation limits.
