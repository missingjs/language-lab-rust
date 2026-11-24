[← Back to projects](../README.md)

# Project 18: Unsafe Abstractions and an Initialization Buffer

**Status:** Expanded existing. **Original project:** 13.

## Goal

Build a small safe fixed-capacity buffer backed by `[MaybeUninit<T>; N]`. Use initialization tracking, ownership transfer, and destruction paths to justify every unsafe operation. FFI remains an optional extension.

## Prerequisites and Setup

- Complete Projects 05, 07, and 08; study raw-pointer validity, initialization, layout, and panic safety first. Use Project 11 when reviewing pinning examples and Project 09 for thread-safety examples.
- Keep the implementation on stable Rust 2024. Use a separately dated nightly with the Miri component for verification; record both toolchains without changing the main toolchain.
- Prepare drop-counted non-`Copy` elements, zero-sized types, and tests that panic in surrounding code. When teaching the course, supply only a minimal storage scaffold and probe types, not the completed implementation.

## Focus

- `unsafe` blocks/functions/traits/implementations and their distinct proof responsibilities
- Alignment, bounds, initialization, provenance, aliasing, and reference validity
- `repr(Rust)` versus `repr(C)`, `MaybeUninit`, `ManuallyDrop`, and ownership
- Partial initialization, panic unwinding, and exactly-once resource release
- `UnsafeCell`, `PhantomData`, auto traits, and safe abstraction review

## Required Interface

Expose only construction, fallible push, pop, a shared slice view, and destruction. The buffer never grows. A full-buffer push must preserve or explicitly return ownership of the rejected value.

Define the invariant before implementation: the length is at most capacity; exactly the active prefix holds initialized values owned by the buffer; remaining slots are not read as `T`.

## Required Work

1. Implement the limited interface while maintaining the initialized-prefix invariant. A shared slice may cover only initialized elements.
2. For each unsafe operation, document its preconditions, where they were established, and which safe operations preserve them.
3. Verify ownership transfer on push/pop and drop only elements still owned by the buffer. Audit state changes and destruction when surrounding code unwinds.
4. Compare the design with a safe Rust alternative such as `Vec<T>` with an enforced limit; explain this exercise's educational value rather than claiming unsafe is necessary for ordinary application code.
5. Review small supplied examples involving `ManuallyDrop`, `UnsafeCell`, `PhantomData`, `unsafe impl Send/Sync`, and pinning. Explain the obligations or defects; do not add manual auto-trait implementations to the buffer.
6. Run the behavior tests under the recorded Miri toolchain. Explain executed-path coverage, model assumptions, and why a passing run is not a universal soundness proof.

## Deliverables

- Buffer, invariant/safety documentation, edge-case and ownership tests
- A Miri run record with exact toolchain and command
- A review note covering error returns, panic, leaks, destruction, and validity assumptions

## Completion Standard

- Cover empty/full buffers, zero capacity, zero-sized types, non-`Copy` values, repeated push/pop, and slice contents.
- Rejected pushes and popped values have explicit owners. No element is read before initialization or released twice.
- Each element still owned by the buffer is released exactly once during normal destruction and surrounding-code panic unwinding.
- Every unsafe operation has a local safety argument consistent with the public safe API.
- Distinguish leaks, panics, logical errors, and undefined behavior; do not treat an experimental aliasing model as a complete language specification.
- Pass shared quality checks and Miri for the exercised paths.

## Optional FFI Extension

Wrap a tiny locally built C library's opaque handle using RAII and typed errors. Supply the C source/build instructions and define allocation/free ownership, null/length validity, ABI/layout, and error/unwind boundaries. Check creation failure, normal release, and use restrictions. Do not expect Miri to execute arbitrary real C code.

## Boundaries

Do not add growth, arbitrary insertion, mutable iterators, an allocator, an arena, or manual `Send`/`Sync` implementations. Recovery from an element destructor that itself panics is outside scope. Do not use FFI alone as a substitute for the required buffer exercise.
