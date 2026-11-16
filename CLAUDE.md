# Project context

An iOS Klondike solitaire built around statistics. The differentiator is a solver that
knows the optimal solution to every deal before a player sees it. Efficiency scoring,
honest difficulty ratings, blunder detection and fair leaderboards all depend on it.

The owner is not a professional developer. Explain decisions in plain language, show
commands to run rather than assuming they are obvious, and never leave the repo in a
state where `cargo test` fails.

## Decisions already made

| Question | Call |
| --- | --- |
| Draw modes at launch | Draw-1 only. Draw-3 waits until the catalog pipeline is proven. |
| First release | Offline, end of Phase 5. No server, no leaderboards at launch. |
| Pricing | Free game with core stats, one-time unlock for advanced analytics and coach mode. |
| Backend | Supabase, built only when Phase 6 arrives. |
| Name | Open. Do not invent one. |

## Build phases

Each phase has one exit test. Do not start a phase before the previous exit test passes.

| Phase | What gets built | Exit test |
| --- | --- | --- |
| 0 | Command-line Klondike solver. No game, no UI. | Solves 1,000 random draw-1 deals, reporting winnability and minimum moves, averaging under 30 seconds per deal |
| 1 | Batch catalog pipeline, difficulty ratings, database output | 50,000 deals solved and rated, difficulty distribution sane |
| 2 | SwiftUI Klondike with draw-1, undo, hints, move log | 10 testers play 50 hands with no rule bugs and no crashes |
| 3 | SQLite stats schema and the three stat surfaces | Every stat in the catalog renders from real play data |
| 4 | Efficiency scoring wired to minimum-move data | A perfect game is detected and celebrated correctly |
| 5 | Customization, contrast guard, accessibility. Ship here. | A deliberately bad background photo is corrected or refused with a reason |
| 6 | Supabase backend, deal service, verifier, leaderboards | A forged submission is rejected, a real one ranks correctly |
| 7 | Blunder detection, rewind, replay viewer, coach patterns | The losing move is named in 95% of lost winnable deals |

**Current phase: 0.** The spec is in `docs/phase-0-solver-spec.md`. Read it before writing code.

## Working agreements

- Rust for the solver. It is fast enough for the catalog and can be called from Swift
  later through a C ABI, so this code is not throwaway.
- Determinism is not negotiable. The same seed must produce the same deal forever, on
  every platform. The shuffle spec is pinned in the Phase 0 spec; do not change it.
- The move counting rules in the spec define every efficiency number the product will
  ever show. Treat them as frozen.
- No networking, no database, no UI in Phase 0.
- Commit after each working milestone with a message that says what now works.
- When something in the spec is ambiguous or looks wrong, stop and ask rather than
  picking an interpretation quietly.
- Prefer boring, readable code over clever code. This will be read by someone learning.

## What to report back

After each milestone, say in three or four sentences: what runs now, how to try it, what
the numbers look like, and what is next. Include the exact command to run.
