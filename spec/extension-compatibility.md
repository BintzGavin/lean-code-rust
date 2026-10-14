# Existing VS Code extensions are required

**Decision D-0003, 2026-10-14:** Gavin explicitly confirmed that existing VS Code extensions must remain a requirement for the Rust direction. This is a hard product requirement. A smaller LSP/DAP-only editor is not an accepted substitute. The recommended direction is a Rust core with compatible Node and isolated webview capabilities; the confirmation does not select a UI toolkit, authorize copying runtime implementation, or prove feasibility/performance.

The compatibility goal remains ordinary local VSIX extensions using the pinned stable 1.133.0 public contract on Apple Silicon macOS. The bounded matrix below tests representative behavior; it is not the supported-API allow-list and cannot establish universal VSIX parity. An unimplemented, non-excluded stable contract remains a compatibility defect, even if none of the first fixtures exercises it.

## Required observable boundary

- **Package and host:** manifest identity/version/engine requirements, dependencies, activation, extension entry point, subscriptions/disposal, host errors, shutdown/restart, and Node execution required by ordinary desktop extensions. Match the documented public interface and observable lifecycle, not original internals. Node built-ins, subprocess behavior and native add-on ABI/platform constraints need separate evidence; JavaScript evaluation alone is insufficient.
- **Editor and workspace:** document/URI/range semantics, read/edit/save, document-change events, configuration and settings scopes, file access/watching, cancellation, diagnostics and output. Candidate public APIs include `workspace.openTextDocument`, `workspace.applyEdit`, `workspace.getConfiguration`, `workspace.fs`, and `languages.createDiagnosticCollection`; validate their exact pinned signatures before claiming support.
- **Contributions:** commands and argument/results, palette/menu visibility, keybindings and context conditions; settings, tree views, themes, TextMate grammars, language configuration and snippets. Registration alone does not prove their UI and persistence behavior.
- **State and processes:** workspace/global extension state, storage URIs, secret-storage behavior without real secrets, terminal/task participation and explicitly invoked local child processes. Workspace trust is not a claim that trusted Node extensions cannot access the filesystem.
- **Webviews:** HTML/script configuration, extension↔view message payloads, local-resource policy, content restrictions, focus, visibility, disposal and state restoration. Observe `postMessage`, receive handlers and view-side messaging through a synthetic contract. Do not assume delivery acknowledgment, ordering across disposed views, or retained state without evidence.
- **Language/debug integration:** existing extension entry points and clients must work alongside LSP language servers and DAP debug adapters. Validate cancellation, errors, process lifecycle and editor-visible results. A stand-alone protocol client that bypasses the extension is not extension parity.

These are desired acceptance boundaries derived from D-0003 and public documentation. The exact current-product results remain unknown. Online documentation is moving; record the versioned stable contract and fixture manifests matching 1.133.0 before execution. No original runtime code or upstream sample implementation is included.

## Bounded first fixture matrix

Recommend 12 independently authored contract-probe families and four existing-extension qualification candidates. The probe families add 24 declared scenarios, S-0041–S-0064, to the canonical catalog. All are blocked/unexecuted with unknown original oracles; no VSIX binaries or extension code were created or installed. The original 40 scenario IDs remain.

| Fixture family | Public contract probes | Successful flow / failure or lifecycle flow | Canonical scenarios |
| --- | --- | --- | --- |
| X-0001 Node host lifecycle | Manifest `main`, activation, context subscriptions, disposal; Node `fs`/`path` | Activate on a named command, read synthetic file / activation exception then independent subsequent command | S-0041, S-0042 |
| X-0002 Document/workspace | Public document, edit, URI, file and event interfaces | Edit/save and capture events/bytes / denied or missing disposable file and cancellation | S-0043, S-0044 |
| X-0003 Commands/settings/keys | Command registration/execution, contribution schema, scoped configuration, keybinding context | Invoke with exact arguments from command/palette/keybinding / change configuration and context, reload, inspect invocation and persistence | S-0045, S-0046 |
| X-0004 Theme/grammar/snippet | Theme and grammar contributions, language configuration, snippets | Select theme, open independently defined language, expand snippet / disable/re-enable contributions and inspect fallback/restoration | S-0047, S-0048 |
| X-0005 Tree view | Tree provider, item identity, selection, reveal, refresh, commands | Expand/select/refresh ordered synthetic items / remove selected item and dispose/recreate view | S-0049, S-0050 |
| X-0006 State/processes | Extension storage interfaces, synthetic secret, Node child process, terminal/task boundaries | Store distinct workspace/global values and invoke bounded child / restart, switch workspace and inspect persistence/process cleanup | S-0051, S-0052 |
| X-0007 Webview | Panel lifecycle, messages, script/resource options, state | Exchange numbered synthetic messages and hide/reveal / dispose with pending message and attempt an outside-resource read | S-0053, S-0054 |
| X-0008 Language server | Existing extension client boundary, LSP negotiation, edits/diagnostics/completion | Start local deterministic server; observe language features / cancel request and disconnect/restart server | S-0055, S-0056 |
| X-0009 Debug adapter | Extension debug contributions/configuration and DAP session | Breakpoint/step/variable/output sequence / disconnect while paused and start a fresh session | S-0057, S-0058 |
| X-0010 VSIX lifecycle | Manifest engines/dependencies, install/enable/disable/update/uninstall | Install v1, invoke, update v2, disable/re-enable, uninstall/restart / invalid package or incompatible engine/dependency, then valid retry | S-0059, S-0060 |
| X-0011 Trust/host fault | Declared untrusted-workspace behavior, trust transition, host isolation | Open untrusted synthetic workspace and explicitly trust / extension-host fault with dirty editor buffer, inspect recovery | S-0061, S-0062 |
| X-0012 Compatibility boundary | Declared excluded/proposed APIs and non-excluded stable control | Invoke an explicitly documented excluded capability and capture precise response/side effects / exercise supported control without starting excluded services | S-0063, S-0064 |

Each run must pin fixture package bytes/hash, source provenance for independently authored probes, manifest engine range, extension dependencies, Node/runtime version, OS/architecture, configuration, clean profile, workspace trust and exact reset procedure. Supply synthetic inputs/expected outputs independently; preserve exact reference results separately. The matrix describes tests to prepare, not artifacts already supplied.

## Existing-extension qualification candidates

These are real maintainer-documented candidates, not approved versions or tested compatibility claims. Select exact local VSIX versions compatible with the baseline, record redistribution/use terms, dependency artifacts and hashes, then add separately identified scenario cells before execution. Their source is not supplied to the independent editor implementer.

| Candidate | Why include it | Qualification dependency |
| --- | --- | --- |
| [rust-analyzer](https://rust-analyzer.github.io/book/vs_code.html) | Existing Rust language extension/client/server behavior | Pin extension/server/toolchain and tiny offline synthetic project; separately inspect runtime requests |
| [VS Code ESLint](https://github.com/microsoft/vscode-eslint) | Existing diagnostics, configuration and code-action workflow | Pin extension and project-local ESLint/config; no global dependency assumption |
| [CodeLLDB](https://github.com/vadimcn/codelldb) | Existing native debugger extension and adapter | Select compatible macOS arm64 package and tiny prebuilt synthetic debug target; no compilation in this pass |
| [GitHub Theme](https://github.com/primer/github-vscode-theme) | Existing declarative theme contribution and selection | Pin VSIX and theme variant; measure appearance separately from accessibility |

These four candidates do not cover webviews, all grammars, every stable API or extension combinations. Existing webview/tree-heavy extension selection and the full product's 10–15 package goal remain open frontier work. Synthetic probes isolate contracts; existing packages expose integration assumptions. Both are needed, and neither establishes universal compatibility.

## Supported, excluded and unresolved are distinct

1. **Required:** ordinary local extensions using non-excluded stable APIs, compatible engine/dependency/platform requirements and the specified macOS target. Passing fixtures is evidence only for their versions and exercised paths.
2. **Previously excluded product categories:** bundled AI/chat/agent/MCP, remote/cloud/account-sync surfaces and named non-core packs remain exclusions in the existing product requirements. An extension depending on one may conflict with the compatibility promise. Record the exact API/category and user-visible effect; obtain a product decision where retention versus exclusion is unresolved. Do not secretly launch excluded services or pretend success.
3. **Not implicitly promised:** proposed/private/internal APIs, platform-specific binaries for other operating systems, branded service access, marketplace access or licensing rights. Identify these dependencies explicitly; do not use this category to dismiss missing non-excluded stable APIs.
4. **Unknown:** native-module compatibility, extensions depending on missing built-ins, undocumented behavior, extra network/service requirements and combinations not exercised. Unknown is not unsupported or passed. Exact unsupported errors and absent side effects still require observation; a generic rejection is not a compatibility proof.

A clean-room implementation may reuse independently selected runtimes/libraries only with declared provenance and appropriate rights; this requirement grants no permission to silently transplant VS Code's extension-host or webview implementation. No architecture internals, copied runtime, sample code or original test source are in this handoff.

## Official public contract sources

Accessed 2026-10-14. These links support the named public surfaces, not Lean results or an exact frozen 1.133.0 oracle: [Extension host](https://code.visualstudio.com/api/advanced-topics/extension-host), [API reference](https://code.visualstudio.com/api/references/vscode-api), [manifest](https://code.visualstudio.com/api/references/extension-manifest), [contribution points](https://code.visualstudio.com/api/references/contribution-points), [commands](https://code.visualstudio.com/api/extension-guides/command), [tree views](https://code.visualstudio.com/api/extension-guides/tree-view), [syntax highlighting](https://code.visualstudio.com/api/language-extensions/syntax-highlight-guide), [webviews](https://code.visualstudio.com/api/extension-guides/webview), [workspace trust](https://code.visualstudio.com/api/extension-guides/workspace-trust), [proposed APIs](https://code.visualstudio.com/api/advanced-topics/using-proposed-api), [extension management](https://code.visualstudio.com/docs/configure/extensions/extension-marketplace), [LSP](https://microsoft.github.io/language-server-protocol/), [DAP](https://microsoft.github.io/debug-adapter-protocol/).
