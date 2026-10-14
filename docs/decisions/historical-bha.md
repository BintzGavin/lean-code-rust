# Historical scaffold provenance

The user's “before we changed them” did not identify exact commits. The selected interpretation uses Helios before autonomous renderer research and Apastra before strategic-planner expansion. The actual repository is named **Apastra**.

| Repository | Selected historical anchor | Subsequent change |
| --- | --- | --- |
| BintzGavin/helios | `e3a3ea432649eff8a912d77257501b3e82031198` | parent of `f7bababaf31d5de67adf97e11bb5c43294001cbb`, autonomous renderer planner |
| BintzGavin/apastra | `0d86253931972524ddcd76c91fb1af8e008b373b` | parent of `5d5e10336508b59c0727a62283183abb4a757af9`, strategic analysis/expanded Vision |

Additional bounds: Helios `135a231094d9e04f6ba8017704d61109e9f65486` precedes `0045419d5020e3f3ecd5b9f320c6c38549169959` refactoring-pain repairs; `951e9eab98764db5032969e7ac6bde22f73555b5` precedes global-adversarial-architect addition `cf1987c320361d5db33c74f20fd3d4aa5fedefe2`. Apastra full Helios-style prompts began at `68060ac5cfc3e0408c5fc522221db1e9edcc1438`; `cce491f5e71fb18ff86e5d70232b29f459c02ba2` is the parent of removal `2dfc8ee685852451e39f367c46737892403ef5e7` and already contains the expanded direction. Local histories only; no fetch or claim of remote freshness.

## Principle-to-scaffold mapping

| BHA principle | Concrete choice |
| --- | --- |
| Written Vision attracts bounded work | One `VISION.md`; one eligible gap and bounded plan per role |
| Stateless agents, durable repository memory | Plans, role backlogs, evidence and critical lessons in Git; no service or hidden database |
| Disjoint ownership prevents coordination overhead | Five path-disjoint lanes; peer needs are explicit dependencies |
| Separate thinking and doing | Planner writes scope/ledger only; executor persists claim then tests and implements |
| Repeated evidence narrows the gap | Behavioral checks and measured reference evidence; static scaffolding is not convergence |
| Honest failure and idle outcomes | Recoverable blocked/needs_input work, preserved negative results, no fabricated work or empty-PR suppression |

Source principles: [article](https://gavinbintz.dev/blog/black-hole-architecture/) and [agents.md companion](https://gavinbintz.dev/blog/black-hole-architecture/agents.md). Templates: local `roll-out-black-hole-architecture` skill's generic seven files, compared with its full Helios core prompts. Historical shape retained: concrete role pairs, exact file inventories, verifiable bounded work, role-local status and critical Learning/Action entries. Historical shared section writes, competing Vision authorities and broad “DELETE IT” cleanup were deliberately not imported.

This provenance describes governance only. No original editor code, tests, assets or internal algorithms are included in the independent implementation input.
