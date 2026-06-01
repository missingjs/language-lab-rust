[← Back to projects](../README.md)

# Project 6: Concurrent URL Checker

## Goal

Build a tool that checks a list of URLs concurrently.

Features:

```text
urlcheck urls.txt --concurrency 20 --timeout 2s
```

## Focus

- async Rust
- Tokio
- reqwest
- timeouts
- concurrency limits
- error handling
- structured output

## Completion Standard

```text
Concurrency is bounded.
Timeouts are enforced.
Failures are reported clearly.
The program exits cleanly on Ctrl+C.
```
