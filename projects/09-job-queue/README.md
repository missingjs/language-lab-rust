[← Back to projects](../README.md)

# Project 9: Background Job Queue

## Goal

Build a small in-process job queue.

Features:

```text
Enqueue jobs.
Run multiple workers.
Retry failed jobs.
Apply timeout.
Shutdown gracefully.
```

## Focus

- Tokio tasks
- channels
- cancellation
- retry policy
- backoff
- state management
- tracing

## Completion Standard

```text
Workers do not leak.
Shutdown waits for active jobs or cancels them deliberately.
Failures and retries are observable.
```
