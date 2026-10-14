# Evidence register

| ID | Kind and provenance | Supported claim | Limits |
| --- | --- | --- | --- |
| E-0001 | Stakeholder/product requirement assertion at V-0001, read 2026-10-14 | The requested feature, exclusion, compatibility, first-platform and performance contract summarized in requirements.md | Desired future state; no current behavior oracle |
| E-0002 | Current release assessment assertion at V-0001, read 2026-10-14 | Startup/memory/CPU/bundle/VSIX/network and recovery evidence are missing; release is blocked | A document's report; not a fresh measurement or proof every missing check was never run elsewhere |
| E-0003 | Local discovery/environment observation R-0001 | Apple Silicon macOS26.3; installed reference metadata1.133.0; Rust1.97.1/Cargo1.97.1; roughly7.9GiB available disk; no identified runnable Lean build | Tools available do not prove feasibility or product behavior; resource readings are momentary |
| E-0004 | Public documentation assertions, accessed 2026-10-14 | Desktop extensions can execute in a local Node.js host; webviews expose HTML and message-passing behavior | Current web docs not a complete pinned1.133.0 API contract; no copied implementation or verified Lean run |
| E-0005 | Public protocol documentation assertions, accessed 2026-10-14 | LSP and DAP define language-tool and debugger communication boundaries | Neither protocol supplies the entire editor extension API or establishes Lean interoperability |
| E-0006 | User direction, 2026-10-14 | Assess a clean-room Rust rewrite using the local clean-room workflow | No decision to abandon ordinary VSIX compatibility, macOS target or safety guarantees |

Portable substance is included above and in requirements.md; the implementer need not access a private account or original repository. Public references support only the stated generic interface facts:

- [Extension host](https://code.visualstudio.com/api/advanced-topics/extension-host)
- [Webview API](https://code.visualstudio.com/api/extension-guides/webview)
- [Language Server Protocol](https://microsoft.github.io/language-server-protocol/)
- [Debug Adapter Protocol](https://microsoft.github.io/debug-adapter-protocol/)

No product UI captures or traces were produced. No current-product oracle is embedded or implied. When observations become available, assign a new run/evidence ID with build, profile, fixture hash, locale, viewport, modality, exact actions and intermediate/final results; preserve earlier assertions and contradictions.

## Compatibility amendment evidence

- **E-0007 — direct stakeholder requirement,2026-10-14:** asked whether existing VS Code extensions must remain a requirement, Gavin answered “Yes for sure.” This resolves optional compatibility reduction; it is not a runtime observation or approval to copy implementation.
- **E-0008 — official public documentation assertions,2026-10-14:** the sources linked in extension-compatibility.md describe the public host, manifest, API, contribution, webview, trust, lifecycle and protocol surfaces used to design X-0001–X-0012. Moving documentation has not been frozen to1.133.0. These are contract-probe proposals, not observed Lean results.
- **E-0009 — maintainer documentation assertions,2026-10-14:** rust-analyzer is a Rust language extension, VS Code ESLint integrates ESLint, CodeLLDB provides native debugging, and GitHub Theme supplies editor themes. Linked maintainer documentation in extension-compatibility.md supports candidate selection only. No extension artifact was downloaded, installed, inspected as implementation, or executed; version/platform/engine/license qualification remains open.
- **E-0010 — installed artifact metadata observation,2026-10-14:** the available branded reference identifies version1.133.0, commit `a5b500951314efd502d07465bd138dfbd714a960`, bundle identifier `com.microsoft.VSCode`. No launch or product behavior was observed. D-0004 subsequently selects V-0002 for bounded reference observation; it does not establish V-0001 Lean parity.

- **E-0011 — environment preflight observation,R-0002,2026-10-14 around13:34UTC:** supported native-app inventory returned that the Mac was locked and could not be made available. No explicit unlock action, app launch, retry, screenshot or edit/save/cancel/reopen action followed. Both planned V-0002 variants are blocked before product launch, not observed product failures.
- **E-0012 — fixture preparation check,R-0002:** two separate copies of the four synthetic text files matched all eight SHA-256 checks; two independent disposable user-data directories contain the intended settings; separate extension directories are empty. No application read or honored these settings yet. These are setup checks, not editor/product tests.
- **E-0013 — public procedure documentation assertions,2026-10-14:** official [CLI](https://code.visualstudio.com/docs/configure/command-line), [telemetry](https://code.visualstudio.com/docs/configure/telemetry), [update settings](https://code.visualstudio.com/docs/supporting/FAQ), [extension management](https://code.visualstudio.com/docs/configure/extensions/extension-marketplace) and [editing](https://code.visualstudio.com/docs/editing/codebasics) documentation support the prepared profile options and intended save/close settings described in interaction-cases.md. Their actual behavior on this installed build remains unobserved.
