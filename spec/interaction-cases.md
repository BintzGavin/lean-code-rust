# First continuous interaction probes

These deepen S-0001 and S-0037. They are prepared procedures, not executed product tests. The target statements below are requirements; all current-product outcomes remain **unknown**. Preserve unexpected behavior rather than correcting the oracle to match the target.

## F-0001 setup and reset

Copy the four text files from fixtures/ into a disposable empty workspace outside this package. Keep manifest.json in the package, not the editor workspace. Verify each copied file's SHA-256 against the manifest. Use a new disposable product profile with optional extensions absent. Record the exact supported profile-launch procedure once a runnable product is available; none is guessed here. Verify autosave/trust/encoding settings rather than assuming defaults. Do not open a personal project. Reset by replacing only the disposable copy from these fixtures; preserve captured outputs elsewhere.

## S-0001 — folder to edited bytes and reopened document

Actor: local individual developer. Baseline V-0001. Starting fixture alpha.txt is exactly UTF-8 the escaped representation `cedar\nline two\n` (where `\n` denotes byte0A, not two literal characters), 15 bytes, without BOM. No unsaved document state should exist; verify this through the identified product.

| Step | Action | Capture / current oracle | Requested target |
| --- | --- | --- | --- |
| 1 | Launch the identified build using the disposable profile; open the fixture folder through its supported UI. | Window readiness, folder identity, trust/onboarding prompts; unknown | Folder is available without unwanted network or excluded onboarding |
| 2 | Open alpha.txt through visible file navigation. | Full text, encoding/EOL indicators, focus; unknown | Text corresponds to fixture bytes |
| 3 | Select only `cedar`; type `maple`. Do not save yet. | Buffer text, dirty indicator, caret/selection, disk bytes; unknown | Buffer changes; preservation semantics must be observed |
| 4 | Invoke the visible Save command. | Transitional/error state, dirty indicator, disk bytes; unknown | Successful save yields exactly `maple\nline two\n` with the original line ending in this fixture |
| 5 | Close the document view and reopen alpha.txt. | Any prompt, tab state, full reopened text, caret; unknown | Reopened text matches saved disk bytes |

Use exact-byte comparison for this fixture after the original behavior is established. Do not infer restart persistence from close/reopen. Modified utf8/CRLF/Unicode behavior needs its own cells before generalizing.

## S-0037 — interrupt close with dirty work

Reset F-0001 and profile; open alpha.txt, replace only `cedar` with `maple`, do not save. Check whether autosave changed disk and record that configuration; if so the intended dirty-state precondition was not established and the attempt is inconclusive.

1. Capture buffer text, file bytes and dirty presentation.
2. Request closing the document view through the UI. Capture every offered option and current focus before selecting anything.
3. If a cancel option is offered, select it. If it is absent, record the actual outcome and missing branch, then end this attempt as branch unavailable. Do not select an unknown destructive option or continue through an unresolved dialog. Do not invent a cancel dialog.
4. Capture whether the view remains, exact buffer text, dirty state and disk bytes. Desired behavior is no loss of unsaved edits after cancellation; current behavior unknown.
5. Continue only if cancellation left the same edited buffer accessible and no dialog active. Invoke Save, close the document view, then reopen it; capture bytes and text. If the view/buffer is unavailable or edits were lost, stop and preserve that result as a discrepancy rather than repairing the run silently.

This sequence tests interruption in one linked session. It does not test crash recovery or OS shutdown. Crash injection is a later, separately scoped disposable-profile probe after save/recovery storage has been characterized.

## Missing visual/accessibility evidence

No screenshot, icon/font asset, shortcut mapping, dialog string, focus order or screen-reader announcement has been observed. Record window size, scale, theme, locale and input mode when capturing them. No pixel parity or historical appearance claim is currently possible.

## Prepared V-0002 reference variants — R-0002

Reuse S-0001 and S-0037 actions against the separately labeled branded reference; never write their outcomes into the Lean V-0001 oracle. Each case has its own disposable workspace, user-data directory and empty extensions directory. Before launch, recheck native UI availability and resource contention. If locked, stop; do not attempt unlocking.

Candidate documented launch options are `--new-window`, `--user-data-dir <case-user-data>`, `--extensions-dir <case-extensions>`, `--disable-extensions`, `--locale en-US`, and `--skip-add-to-recently-opened <case-workspace>`. They have not been executed or validated against the installed CLI. Preserve existing user windows; never use reuse-window or quit the whole app. Identify the disposable window by its workspace before every edit.

Prepared isolated `User/settings.json`: `telemetry.telemetryLevel: off`, `update.mode: none`, `extensions.autoUpdate: off`, `extensions.autoCheckUpdates: false`, `files.autoSave: off`, `files.hotExit: off`. These are intended settings documented publicly, not observed effective runtime settings. Verify their recognition when UI becomes available. They affect only the disposable profile, not user defaults. Record these explicit settings as part of V-0002; do not infer default hot-exit behavior from a run with hot exit disabled. No network-isolation or zero-network claim follows from profile settings. Do not enable networking, sync, sign-in, extensions or trust bypasses. Stop for any confirmation requiring user action.

Capture exact baseline version, fixture path identity, editor text, dirty state, disk bytes, save result, close prompt/options and reopen result in order. Filesystem reads may check disposable bytes; GUI transitions must be observed through supported UI controls. On completion close only the disposable window after handling its synthetic changes explicitly; do not terminate existing user sessions. No captures were taken during the locked R-0002 preflight.
