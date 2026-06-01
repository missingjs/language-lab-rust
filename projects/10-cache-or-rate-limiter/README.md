[← Back to projects](../README.md)

# Project 10: Cache or Rate Limiter

## Goal

Build one of:

```text
TTL cache
LRU cache
token bucket rate limiter
sliding window rate limiter
```

## Focus

- ownership and shared state
- `Arc`
- `Mutex` / `RwLock`
- time handling
- generics
- trait bounds
- testing time-dependent logic

## Completion Standard

```text
The data structure is concurrency-safe.
Tests cover expiration or rate limiting behavior.
The public API is ergonomic.
```
