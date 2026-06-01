[← Back to projects](../README.md)

# Project 7: In-Memory HTTP API

## Goal

Build a small REST API, such as a bookmark service.

Endpoints:

```text
POST   /bookmarks
GET    /bookmarks
GET    /bookmarks/{id}
DELETE /bookmarks/{id}
```

## Focus

- axum
- serde
- shared state
- `Arc`
- `Mutex` or `RwLock`
- handler design
- error responses
- integration tests

## Completion Standard

```text
HTTP layer and domain logic are separated.
Errors return consistent JSON responses.
Shared state is concurrency-safe.
```
