[← Back to projects](../README.md)

# Project 9: Standard-Library Bounded Work Queue

**Status:** New.

## Goal

Implement a capacity-limited queue with a fixed number of standard-library worker threads and a defined drain-and-join shutdown contract.

## Prerequisites and Setup

- Complete Project 08 and study threads, `Send`/`Sync`, locks, channels, and condition variables first.
- Use the standard library; no Tokio or external services are needed.
- Prepare finite jobs with IDs, event recording, barriers/channels for controlled progress, and a deliberately panicking job.

## Focus

- Thread ownership, `move` captures, join handles, scoped threads, and auto traits
- `Mutex`, `RwLock`, `Condvar`, shared predicates, lock scope, and poisoning
- Bounded capacity, backpressure, close states, safety, and progress
- Atomicity, modification order, happens-before, and memory ordering

## Required Work

1. Define submission, receipt, execution, completion, and closure states before implementation. Specify queue capacity and worker-count validation.
2. Implement bounded submission, worker waiting, stop-accepting, draining, and joining. Define what happens to a submission racing with close; wake blocked submitters and workers when closure changes their predicate.
3. Execute user jobs outside the queue lock. Select and document a task-panic policy that still accounts for accepted jobs and permits worker reclamation.
4. Check empty, full, close-with-pending-work, repeated close, and task-panic paths using controlled events. A shutdown call must be made from a context that cannot join itself.
5. Add short experiments comparing scoped and unscoped thread borrowing, channel-based versus shared-state aggregation, and accepted/rejected `Send`/`Sync` boundaries.
6. Explain small atomic examples: a `Relaxed` statistics counter, Acquire/Release publication through appropriately protected storage, and why `SeqCst` cannot make a multi-step business operation atomic. Keep this separate from the queue's locking design.

## Deliverables

- Queue, state/invariant note, deterministic coordination tests, and panic-policy explanation
- Thread-boundary and memory-ordering examples with expected outcomes and reasoning
- A comparison of locking and message-passing alternatives

## Completion Standard

- In non-panicking runs, each accepted job executes exactly once and pending capacity never exceeds the configured limit.
- A job failure has a recorded outcome under the selected policy. Accepted work cannot disappear without explanation.
- Shutdown rejects new work, wakes relevant waiters, accounts for pending work, and joins all workers, including failure paths.
- Predicates are checked in a loop; no unpromised fairness assumption is required for correctness.
- Explain lock ordering, poisoning policy, and the difference between data-race prevention and application correctness.
- Tests establish race conditions with events/barriers rather than fixed sleeps; use bounded watchdogs only to detect hangs.

## Boundaries

Do not implement work stealing, lock-free storage, or a production thread-pool framework. Atomic counters are not a replacement for a clear queue invariant. Reuse the closure logic in Project 19's small model.
