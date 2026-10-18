# DESKTOP Critical Memory

This role-local ledger belongs to `desktop`. The matching planner reads it and the matching executor may update it across runs. Other roles must not write it.

Store only reusable facts that materially change future planning or execution.

Admit an entry only for a recurring domain trap, hidden architectural constraint, material verification lesson, or dependency pattern. Do not record routine work or narration.

## Entry format

```markdown
## [ROLE-LOCAL VERSION OR STABLE PLAN ID] — [Title]
**Learning:** [Reusable fact]
**Action:** [How a later run should apply it]
```

## desktop-019 — Match guards for keyboard shortcuts
**Learning:** When matching `WindowEvent::KeyboardInput` characters for keyboard shortcuts in `main.rs`, the modifier checks (e.g. `self.modifiers.control_key()`) must be included directly in the `match` arm guard (e.g., `if (c == "a" || c == "A") && (self.modifiers.control_key() || self.modifiers.super_key()) =>`) rather than inside the arm block. If the guard only matches the character, typing that character normally without modifiers will enter the block, fail the internal modifier check, do nothing, and exit the `match`, dropping the keypress completely.
**Action:** When adding new keyboard shortcuts in the desktop application, always place modifier checks in the match guard to prevent intercepting and dropping standard typing events for that character.
