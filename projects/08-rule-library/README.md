[← Back to projects](../README.md)

# Project 8: Extensible Rule Library and Public API Evolution

**Status:** New.

## Goal

Extract a reusable text-analysis rule library, exercise it from independent callers, and evaluate one public API change using downstream compilation and behavior.

## Prerequisites and Setup

- Complete Projects 03–04 and 06–07. Study trait bounds, associated types, dispatch, and typed errors first.
- Reuse text fixtures and analysis logic; the standard library is sufficient for the required library.
- Create a library plus two caller examples and integration/documentation tests. Introduce workspace organization later in Project 20.

## Focus

- Concrete types, generics, associated types, `impl Trait`, and `dyn Trait`
- Trait contracts, default methods, blanket implementations, orphan rules, and overlap
- Dyn compatibility, object lifetimes, auto traits, and public bounds
- Higher-ranked trait bounds (HRTBs), generic associated types (GATs), and borrowing abstractions
- Newtypes, validated construction, builders, and proportionate typestate
- `From`/`TryFrom`, `AsRef`/`Borrow`, library errors, and diagnostic context
- SemVer and observable downstream contracts

## Required Work

1. Expose text input, a small composable set of analysis rules, results, and typed errors. Callers must be able to handle expected failures.
2. Compare at least two suitable abstraction choices in small prototypes; choose one for the final library and explain ownership, dispatch, extensibility, and complexity.
3. Provide one simple caller and one caller composing rules. Validate configuration at construction or an equally clear boundary.
4. Use focused examples to compare generic parameters with associated types, demonstrate an HRTB callback, and explain a GAT whose output depends on a borrow. Also demonstrate a coherence restriction and a dyn-compatibility restriction.
5. Check a returned `impl Trait` example for borrowing/capture behavior on the recorded compiler. Explain the public bounds and object-lifetime assumptions rather than treating the concrete type as the entire contract.
6. Make one rule extension or representation change. Compare before/after downstream compilation and behavior; classify any breakage and propose an appropriate compatibility strategy.

## Deliverables

- Library, two caller examples, integration tests, and documentation tests
- Small language experiments with explanations of expected compile failures
- An API evolution note covering error extensibility, bounds/auto traits, and at least one rejected design

## Completion Standard

- Invalid inputs and configurations do not silently succeed; callers can distinguish expected error categories.
- Public interfaces expose intentional ownership and conversion semantics.
- Explain `AsRef` versus the stronger semantic expectations of `Borrow`, and fallible versus infallible conversion.
- Verify the API change against saved caller examples; do not infer compatibility from the library's own tests alone.
- Explain why the final implementation does or does not need GATs, typestate, or trait objects.
- Pass shared quality checks and all public-usage tests.

## Boundaries

Keep rules and business behavior small. Advanced type features belong in focused experiments unless they solve a real library requirement. Do not add macros, a plugin loader, a database, or a custom framework.
