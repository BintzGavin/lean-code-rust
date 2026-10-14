# Scope and baseline

## Baselines

**V-0001 — Lean repository baseline.** Revision `381d626b81f0cac2e5fe2fc83bfbf2de71c8af69`, identified 2026-10-14. The requested dependency baseline is Code - OSS 1.133.0. No running Lean build was identified. No UI, extension, network, memory, startup, or crash-recovery observation was performed. Configuration, selected built-ins, runtime version, viewport, theme, locale, accessibility setup and actual product state are unknown. Unpublished local governance experiments are outside this baseline.

**V-0002 — available reference candidate.** Installed branded Visual Studio Code (commit `a5b500951314efd502d07465bd138dfbd714a960`, bundle identifier `com.microsoft.VSCode`) reports 1.133.0 in application metadata on Apple Silicon macOS 26.3 (build 25D125). This is not Lean or an identified unmodified Code - OSS build. Metadata is not a UI observation. Do not transfer behavior, performance, enabled services, licensing or network conclusions between V-0001 and V-0002. Historical 1.10.2 and 1.20.1 are requested behavior references only and have not been executed.

**R-0001 — discovery run.** Repository-assisted, source-exposed analyst; 2026-10-14 UTC. Read current product requirements and release assertions; public interface documentation; inspected available environment. No product session or product screenshot. Native UI access was unavailable because the Mac was locked and automatic unlock failed; no bypass attempted. Browser-accessible Jules state is unrelated to editor behavior and is excluded from the handoff.

## Finite first-pass boundary

The catalog contains 20 areas with 40 initial scenarios and 24 additional extension-contract scenarios. The denominator grew because Gavin confirmed existing-extension compatibility as a hard requirement; no earlier IDs were retired. These are a risk-prioritized discovery seed, not an exhaustive cross-product. Initial exact fixture: UTF-8 local text, LF and CRLF files, two source files, one nested path, no secrets. First probes target one individual, no account, local single folder, clean profile, keyboard and pointer, macOS arm64. Use an explicit en-US locale, default theme and recorded viewport for future runs; those settings have not been observed yet.

Included surfaces: launch and folder opening; editing and saving; navigation; search; tabs/splits; palette and settings; snippets/themes; Git; terminal/tasks; diagnostics/debugging; VSIX lifecycle and selected API boundaries; webviews; LSP/DAP; workspace trust; exclusions; network; performance; interruption/recovery; controlled updates/accessibility.

Remaining variants are frontier items, not covered combinations: multi-root, symlinks, file permission denial, Unicode normalization, IME, large/binary files, remote/mounted filesystems, simultaneous edits, multiple windows, screen reader/reduced motion/high contrast, localization, extension combinations, malformed packages, native extension modules, long-running child processes, signing/update failure and OS restart. Some have seed cases; no claim extends beyond an actually executed case.

Windows/Linux signed distributions, mobile, web/server edition, remote SSH/containers/WSL, cloud accounts/sync, AI, notebooks and testing UI are excluded from this first target by requested product scope. This is not proof that the existing product lacks those surfaces. Historical unsafe runtimes and production-user data are excluded from execution. Pixel-identical historical visual reproduction is undecided; no font/icon/screenshot assets are supplied.

Authorized operations for this pass: local read-only discovery and creation/validation of this package and synthetic fixtures. No deployment, publication, original-product modification, installer execution, account changes, live update or external messages are part of the parity procedure.

**R-0002 — bounded branded-reference preflight,2026-10-14.** V-0002 is selected for S-0001 and S-0037 as separately labeled reference variants, not a product-scope reduction or Lean parity. Native app inventory reported the Mac locked; no explicit unlock action, retry, app launch or GUI scenario followed. Two independent synthetic workspace/profile/empty-extension-directory setups were prepared and their bytes/configuration verified. Product observation remains0/64Lean cells and0/2branded-reference variants. Source-free prepared data stays outside this handoff; procedures and evidence are recorded here.
