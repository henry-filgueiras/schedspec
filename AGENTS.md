# AGENTS.md

Guidance for agents editing this repository.

## Project Identity

- `ChronOS` is a proposed temporal operating system for stateful workflows.
- `chrono flow` is its proposed durable, deterministic, time-aware workflow language.
- This repo is design-first. Do not imply a finished implementation unless a file in the repo proves it.

Core thesis to preserve:

- once time, replay, recovery, migration, operator intervention, and external effects become first-class, workflow execution stops looking like "background jobs plus retries" and starts looking like an operating-systems problem

## Canonical Doc Stack

The intended stack is:

- `README.md`: concise front door
- `docs/CHRONOS_README.md`: long-form vision and thesis
- `docs/GLOSSARY.md`: shared vocabulary and invariants
- `docs/SPEC.md`: semantic contract
- `docs/LANGUAGE.md`: language sketch and determinism model
- `docs/EXAMPLES.md`: worked scenarios
- `docs/ARCHITECTURE.md`: proposed runtime shape
- `docs/DIAGRAMS.md`: canonical Mermaid diagrams and conventions
- `docs/GOOD_FIRST_DRAGONS.md`: contributor-scale hard problems
- `SAMEDIFF.md`: adjacent replay-diff lineage

Top-level `CHRONOS_README.md` and `CHRONOS_SPEC.md` are intentional redirect stubs. Do not expand them into competing full documents unless explicitly asked.

## Editing Priorities

When editing docs in this repo:

- preserve the strongest existing phrasing where it already works
- keep README concise and navigational
- keep proposal wording explicit: use forms like "proposed", "intended", and "a conforming implementation would..."
- reduce duplication across docs; each file should have a clear job
- prefer elegant compression over bloated explanation
- keep links repo-relative, not absolute filesystem paths

## Invariants To Preserve

These ideas should remain consistent across the repo:

- history is normative; materialized state is derived
- flow identity persists across time
- replay is a primitive, not merely a debugging trick
- effects cross an explicit nondeterminism boundary
- timers are durable data, not sleep calls
- operator actions are auditable events
- migration is normal life for long-lived workflows
- observability should be structural, not log archaeology

## Tone

Desired tone:

- serious systems-language / PL / distributed-systems flavor
- sharp and enticing, but not hypey
- ambitious, but technically grounded

Avoid:

- generic startup copy
- hand-wavy "agent framework" language
- fabricated benchmarks, deployments, or implementation status
- faux certainty where the design is still open

## Language And Spec Boundaries

Keep these boundaries clear:

- `docs/CHRONOS_README.md` explains why the project exists
- `docs/GLOSSARY.md` defines terms and invariants
- `docs/SPEC.md` states the semantic contract
- `docs/LANGUAGE.md` sketches syntax and determinism constraints without prematurely freezing the surface
- `docs/ARCHITECTURE.md` explains proposed runtime shape without pretending the runtime exists

## Diagrams

- Mermaid is canonical.
- Rendered PNGs under `docs/reference-images/` are supporting reference material, not the source of truth.
- Keep diagram terminology aligned with the rest of the docs.

## Repo Conventions

- Use repo-relative Markdown links.
- Prefer text-first artifacts.
- Keep `SAMEDIFF.md` integrated where replay-diff and divergence explanation matter, but do not let it overshadow ChronOS.
- Preserve the root-level structure unless there is a clear organizational improvement.

## If You Add New Material

- Make it fit the existing doc stack instead of creating parallel narratives.
- Cross-link it intentionally from the most relevant canonical doc.
- Keep examples consistent with the current `chrono flow` notation style.
