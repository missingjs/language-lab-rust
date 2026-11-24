[← Back to projects](../README.md)

# Project 5: Resource Lifecycles and Interior Mutability

**Status:** New.

## Goal

Build a small experiment suite that predicts and observes resource release, shared ownership, and runtime borrow checking. Explain each result before changing the implementation.

## Prerequisites and Setup

- Complete Projects 01–04 and study moves, borrowing, scopes, `Box`, `Rc`, and interior mutability first.
- Use the standard library and the shared [toolchain requirements](../README.md#environment-and-shared-completion-requirements).
- Use a named probe type with a `Drop` implementation and an event recorder. Keep the recorder alive longer than the probes it observes.

## Focus

- Stack values, heap allocations, indirection, and RAII for memory and other resources
- Moves, explicit `drop`, scope exit, and release order
- `Box`, shared ownership through `Rc`, and non-owning links through `Weak`
- `Cell` versus runtime-checked borrowing through `RefCell`
- Memory leaks versus panics, logical errors, and memory safety

## Required Work

1. Record a prediction and event trace for scope exit, ownership transfer, and explicit release. Show that a move does not itself destroy the resource.
2. Clone `Rc` handles and release them in different scopes. Observe when the inner value is dropped and what a `Weak` handle can still access.
3. Demonstrate a small reference cycle in an isolated example, then replace an appropriate edge with `Weak` and compare release events.
4. Trigger an overlapping `RefCell` borrow in an isolated test, explain the runtime failure, and repair the borrow scope. Compare a fallible borrow with a panicking borrow.
5. Contrast the experiment with a minimal example rejected by the compile-time borrow checker. Keep intentionally failing code separate from normal builds.

## Deliverables

- Runnable experiments and tests, including an isolated expected-panic case
- A prediction/result table and explanations of ownership and release behavior
- A short comparison of `Cell`, `RefCell`, owned values, and ordinary references

## Completion Standard

- Event traces show each normally owned probe is released exactly once; the cycle experiment explicitly identifies the leak.
- Explain the difference between dropping an `Rc` handle and dropping its inner value. Explain that a `Weak` handle does not keep that value alive.
- Explain why a `RefCell` conflict is detected at runtime and why interior mutability is not a universal fix for ownership problems.
- Distinguish observed implementation details from guaranteed behavior; do not infer a stable type layout from addresses.
- Apply the shared quality checks to the successful examples.

## Boundaries

Use small single-threaded examples. Do not use raw pointers, `unsafe`, custom allocators, or a general-purpose graph library. Provide experiment scaffolding when teaching this course, while leaving predictions and explanations to the learner.
