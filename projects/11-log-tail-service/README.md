[← Back to projects](../README.md)

# Project 11: Streaming Log Tail Service

## Goal

Build a service that streams log updates over HTTP.

Features:

```text
GET /logs
GET /logs/stream
```

## Focus

- streaming responses
- Server-Sent Events or WebSocket
- file watching
- client disconnect handling
- backpressure
- async task lifecycle

## Completion Standard

```text
Disconnected clients are cleaned up.
Slow clients do not break the entire service.
The service shuts down gracefully.
```
