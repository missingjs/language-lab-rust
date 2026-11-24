[← Back to projects](../README.md)

# Project 7: Text Store: Borrowed Views and Stable Handles

**Status:** New.

## Goal

Design a text store with two access models: borrowed query results for immediate use, and stable record handles that callers can retain across mutations.

## Prerequisites and Setup

- Complete Projects 05–06; study reborrowing, lifetime annotations, and owned versus borrowed data first.
- Use only the standard library. Reuse parser inputs where useful.
- Prepare a fixed fixture containing Chinese text, empty strings, duplicate contents, missing matches, and multiple records.

## Focus

- Receiver borrowing, returned references, non-overlapping field and slice access
- Multiple lifetime parameters, outlives constraints, `T: 'a`, and `&'a T`
- Lifetime subtyping, covariance, invariance, and function parameter/return variance
- Owned storage, borrowed views, `Cow`, indices, and stable handles
- Why persistent self-references complicate mutation and ownership

## Required Work

1. Build an owning store with add and query operations. Query results borrow matching text instead of copying complete records.
2. Keep a minimal compile-fail example that uses a borrowed result across an incompatible mutation. Explain the signature-level constraint and a valid caller redesign.
3. Add deletion and handle lookup. A handle may be retained across changes, but deleted or invalid handles must never silently resolve to a different record. Never-reused IDs are sufficient; generational handles are optional.
4. Compare borrowed access with handle access, including lookup cost, mutation flexibility, and explicit invalidation behavior. Include an owned or `Cow` alternative in the comparison without requiring a third store implementation.
5. Add small passing/failing examples for reborrowing, field splitting, safe slice splitting, outlives constraints, and lifetime variance. Explain a failed substitution involving mutable access or interior mutability, and the direction of a function-type lifetime substitution.

## Deliverables

- Store implementation, borrowed and handle-based caller examples, and tests
- Isolated compile-fail examples with explanations, including why annotations cannot extend storage lifetime
- A design note explaining data ownership, handle allocation/exhaustion behavior, and rejected alternatives

## Completion Standard

- Normal, empty, Unicode, duplicate-content, missing-result, and deletion cases have deterministic checks.
- Borrowed references are tied to the store's valid borrow, and lookups cannot return references to destroyed storage.
- Delete a record, add another, and verify the old handle remains invalid. Invalid IDs and allocation exhaustion have defined behavior.
- Explain why `T: 'static` does not require the value to remain alive forever.
- Run the shared quality checks; exclude intentionally failing examples from successful build targets.

## Boundaries

Do not build a general arena or use `unsafe` or `Rc<RefCell<_>>` to bypass the design problem. Stable handles identify records; they do not promise stable memory addresses. Keep the lifetime experiments small and separate from the store's public API.
