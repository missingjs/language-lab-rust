[← Back to projects](../README.md)

# Project 11: Futures, Cancellation, and Async Task Lifecycles

**Status:** New.

## Goal

Explain how futures make progress and verify task/resource behavior when operations complete, fail, time out, or are cancelled at different stages.

## Prerequisites and Setup

- Complete Projects 09–10. Study the `Future` polling model and safe `Pin` usage before the experiments.
- Use Tokio 1.x and record the resolved version and enabled features. Enable test-time clock controls if used.
- Prepare simulated operations controlled by channels/notifications, task IDs, and release probes; do not depend on public network services.

## Focus

- Future creation, `poll`, `Pending`/`Ready`, wakeups, and executors
- Stack pinning, `Box::pin`, `Unpin`, address-sensitive state, and pointer versus pointee movement
- Spawn boundaries, values held across `.await`, `Send`, join errors, and runtime flavors
- `join`, `select`, dropping a future, detached tasks, abort, timeouts, and cancellation safety
- Bounded channels, permits, task collections, blocking isolation, and shutdown

## Required Work

1. Create a small safe future or instrumented wrapper that becomes ready through controlled state and wakeups. Record creation, polling, wakeup, and completion; do not assert an exact executor poll count.
2. Compare awaiting an operation with spawning it, and compare current-thread with multi-thread execution. Demonstrate accepted/rejected `Send` futures and explain the value kept across an await.
3. Pin through safe stack and heap interfaces. Explain why moving a pointer is different from moving the pinned value, and why pinning is not equivalent to heap allocation.
4. Compare dropping an unspawned future, dropping a task handle, timing out, aborting, and cooperative cancellation. Collect task outcomes and drop events.
5. Build a bounded simulated task runner with limits on queued and in-flight work. Support normal draining and explicit cancellation, distinguishing failure, timeout, cancellation, and completion.
6. Cancel both while waiting and after observable partial progress. Document any partial side effect and a protocol that preserves or explicitly reports it; cancellation must not be described as rollback.
7. Isolate a finite blocking operation with `spawn_blocking`. Show why a started blocking operation needs its own cooperation/finish policy even after the async waiter is cancelled.

## Deliverables

- Runnable experiments, a small bounded runner, and event-based tests
- A lifecycle comparison table and cancellation contract
- Task/resource accounting and an explanation of sync versus async lock selection

## Completion Standard

- Observed queued and in-flight work respect the respective bounds; permits are acquired before unbounded task creation can occur.
- Drain collects accepted results; cancel explicitly accounts for incomplete work. All spawned work is joined or otherwise confirmed finished under the stated policy.
- Tests cover waiting and partial-progress cancellation and resource release, using controlled operations or paused time instead of exact wall-clock timings.
- Explain that `.await` need not yield, wakeups request progress rather than completing work, and task safety depends on state saved across awaits.
- Complete the shared checks and explain what the experiments do not prove about arbitrary I/O APIs.

## Boundaries

Do not implement an executor, unsafe pin projection, or production networking. Keep blocking work finite or cooperatively stoppable so the test process can exit. Reuse the lifecycle lessons in Projects 14 and 16.
