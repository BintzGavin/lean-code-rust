# quality-004 — X-0004 Theme/grammar/snippet

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-004`

## 1. Context & Goal

- **Objective**: Create a compatibility fixture for theme, grammar, and snippet contributions (X-0004).
- **Vision gap**: D-0003 requires existing VS Code extensions to work. Theme, grammar, and language configurations are unverified existing behavior (S-0047, S-0048).
- **Evidence**: `spec/extension-compatibility.md` lists `X-0004 Theme/grammar/snippet` covering `S-0047` and `S-0048` as unexecuted/unknown.
- **Impact**: Enables verification of declarative theme and language syntax boundary parsing.

## 2. File Inventory

```json
{
  "id": "quality-004",
  "role": "quality",
  "create": [
    "tests/compatibility/x0004_theme_grammar_snippet/package.json",
    "tests/compatibility/x0004_theme_grammar_snippet/extension.js",
    "tests/compatibility/x0004_theme_grammar_snippet/test.sh",
    "tests/compatibility/x0004_theme_grammar_snippet/synthetic-theme.json",
    "tests/compatibility/x0004_theme_grammar_snippet/synthetic-grammar.json",
    "tests/compatibility/x0004_theme_grammar_snippet/synthetic-language-configuration.json",
    "tests/compatibility/x0004_theme_grammar_snippet/synthetic-snippets.json"
  ],
  "modify": [],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The fixture must contribute a theme, a language grammar, a language configuration, and snippets. The test script must validate the package and structure of these files.
- **Compatibility constraints**: Must use stable standard VS Code contribution schema.
- **Architectural constraints**: No external dependencies. Run locally as a VSIX.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `bash tests/compatibility/x0004_theme_grammar_snippet/test.sh` should fail because it doesn't exist.
- **Verification commands**: `bash tests/compatibility/x0004_theme_grammar_snippet/test.sh` and `python3 tooling/check_scaffold.py --role quality`.
- **Success criteria**: The script successfully validates the structure of the package and contributed files.
- **Edge cases**: Missing contribution files.

## 5. Constraints

- **Non-goals**: Implementing Rust parsing of these formats. This is just the fixture.
- **Forbidden changes**: No changes to core implementation.
- **Scope ceiling**: Scoped exactly to X-0004 requirements.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
