[← Back to projects](../README.md)

# Project 19: Invariant Verification and Failure Reduction

**Status:** New.

## Goal

Choose verification methods from component contracts, find or inject a failure, reduce it, and retain a reproducible regression case with an explicit statement of what each tool checked.

## Prerequisites and Setup

- Reuse a parser or rule library from Projects 06/08, closure logic from Project 09, and Miri results from Project 18.
- Study invariant-based testing, fuzzing, and small-model checking before running tools.
- Use proptest, cargo-fuzz, Loom, and the already recorded Miri toolchain. Record exact resolved versions, installation commands, required toolchains, and platform conditions.
- Keep nightly-only tooling separate from normal stable builds. Establish finite local fuzzing and model-exploration budgets before running.

## Focus

- Properties derived from behavior contracts, input generators, shrinking, and regression seeds
- Compile-pass/fail tests for API restrictions and borrowing contracts
- Fuzz harnesses, minimized corpora, and deterministic replay
- Loom primitive substitution, state-space bounds, scheduling, and memory-model limitations
- Fault injection, evidence organization, and tool-specific coverage

## Required Work

1. State at least two useful properties for the selected parser/library. Examples include valid span boundaries, agreement with a reference, or a round trip when the format actually defines one. Do not duplicate the implementation inside the assertion.
2. Add property tests with relevant Unicode, empty, malformed, and boundary inputs. Save failure seeds or minimized cases and document replay.
3. Add a fuzz target for one bounded parsing/analysis entrypoint. Define input-size and time/run budgets, retain interesting failures, minimize one real failure or controlled injected defect, and verify its repair.
4. Extract a tiny queue-close or state-handoff model. Replace the modeled synchronization with Loom primitives and state its capacity, thread count, assertions, and exploration limits.
5. Introduce a deliberate synchronization defect into the isolated model, demonstrate detection within the recorded bounds, then verify the corrected model. Do not leave the defect enabled in normal checks.
6. Turn a borrowed-interface or trait-bound restriction into a compile-fail case and retain a corresponding valid caller. Assert the intended restriction without relying on every word of a diagnostic.
7. Summarize the buffer's Miri results alongside property/fuzz/Loom results, explaining why these methods do not replace one another.

## Deliverables

- Property tests, fuzz harness, compile tests, and a bounded Loom model
- Reproduction commands, seeds/corpus or injected cases, and before/after failure evidence
- A diagnostic report separating observed facts, model assumptions, explanations, and remaining uncertainty

## Completion Standard

- Failures can be replayed from saved artifacts and reduced to a useful counterexample.
- The selected properties follow public contracts and expose a meaningful defect when deliberately violated.
- The Loom report identifies primitive replacements, checked invariants, and exploration limits; it does not claim to verify the entire real queue or runtime.
- Passing checks are reported as evidence for exercised paths or explored states, not universal correctness proofs.
- Normal shared checks remain usable independently of bounded specialist-tool runs.

## Boundaries

Do not model an entire async runtime or require continuous large-scale fuzzing. Keep intentionally broken variants isolated from the normal passing suite. Provide a small tooling setup/model example when teaching the course; the learner defines the component properties and analyzes failures.
