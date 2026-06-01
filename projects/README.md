# Project-Based Learning Path

The best way to learn Rust is through small projects that each target a specific set of concepts. Many ownership, lifetime, and API design questions only become real when you put your hands on code:

- Who should own this data?
- Should this function take `&T`, `&mut T`, or `T`?
- Is this clone deliberate, or am I avoiding a design decision?
- Should this be `Result` or a panic?
- Is `Arc<Mutex<T>>` the right answer, or message passing?
- Should this parse borrow `&str` or own `String`?
- Where does unsafe begin and end?

The project sequence increases difficulty gradually. Each project focuses on one to three major ideas.

For the conceptual map behind these projects, see [../roadmap.md](../roadmap.md).

---

## The Thirteen Projects

1. [CLI TODO Tool](01-cli-todo/README.md) — Cargo, structs, enums, ownership basics, file I/O.
2. [Log Analyzer](02-log-analyzer/README.md) — `BufReader`, iterators, streaming, string parsing.
3. [Config Loader](03-config-loader/README.md) — `serde`, custom error types, API design.
4. [Domain Validator](04-domain-validator/README.md) — newtype pattern, typed errors, primitive obsession.
5. [Mini Parser](05-mini-parser/README.md) — `&str`, lifetimes, recursive data, zero-copy parsing.
6. [Concurrent URL Checker](06-url-checker/README.md) — async Rust, Tokio, reqwest, bounded concurrency.
7. [In-Memory HTTP API](07-http-api/README.md) — axum, shared state, `Arc`, error responses.
8. [Database-Backed API](08-db-api/README.md) — sqlx, migrations, transactions, repositories.
9. [Background Job Queue](09-job-queue/README.md) — Tokio tasks, channels, retry, cancellation.
10. [Cache or Rate Limiter](10-cache-or-rate-limiter/README.md) — shared state, generics, time-dependent logic.
11. [Streaming Log Tail Service](11-log-tail-service/README.md) — streaming responses, SSE/WebSocket, backpressure.
12. [Performance Lab](12-performance-lab/README.md) — Criterion, profiling, allocation reduction, `Cow`.
13. [Unsafe Boundary Exercise](13-unsafe-boundary/README.md) — raw pointers, FFI, safe abstractions, invariants.

---

## Recommended Project Order

```text
1. CLI TODO Tool
2. Log Analyzer
3. Config Loader
4. Domain Validator
5. Mini Parser
6. Concurrent URL Checker
7. In-Memory HTTP API
8. Database-Backed API
9. Background Job Queue
10. Cache or Rate Limiter
11. Streaming Log Tail Service
12. Performance Lab
13. Unsafe Boundary Exercise
```

This order gradually builds:

```text
syntax and tooling
→ ownership and borrowing
→ error modeling
→ lifetimes
→ async and concurrency
→ web services
→ databases
→ production behavior
→ performance
→ unsafe boundaries
```

---

## If Time Is Limited

If you only have time for five projects, do these:

```text
1. Log Analyzer
2. Domain Validator
3. Mini Parser
4. Concurrent URL Checker
5. Database-Backed API
```

These cover the most important Rust skills:

```text
ownership
borrowing
lifetimes
error handling
iterators
async
concurrency
API design
serialization
database access
```

---

## Three-Version Method for Every Project

For each project, build it in three passes.

### Version 1: Make It Work

Focus on completing the feature set. Do not over-engineer.

### Version 2: Make It Idiomatic

Refactor for:

```text
clear ownership
fewer clones
better error types
smaller modules
cleaner API boundaries
more tests
```

### Version 3: Make It Robust

Add production-like concerns:

```text
logging / tracing
timeouts
configuration
benchmarks
graceful shutdown
property tests
Clippy cleanups
```

---

## Common Rust Mistakes

### Mistake 1: Fighting the Borrow Checker Instead of Redesigning Ownership

If the borrow checker keeps rejecting your code, step back and ask:

```text
Who should own this data?
Can I split the data structure?
Should this function borrow instead of own?
Should this value be cloned deliberately?
Is shared mutability actually required?
```

### Mistake 2: Using `clone()` Everywhere

Cloning is sometimes correct, but excessive cloning often hides poor ownership design.

Ask:

```text
Can this function take &T instead of T?
Can this type use borrowed data?
Is the clone cheap and intentional?
```

### Mistake 3: Calling `unwrap()` in Real Code

`unwrap()` is acceptable in quick experiments, tests, and impossible states. For application code, prefer explicit error handling.

### Mistake 4: Starting With Async Too Early

Async Rust combines lifetimes, traits, pinning, runtimes, cancellation, and Send bounds. Learn synchronous Rust well first.

### Mistake 5: Overusing `Arc<Mutex<T>>`

`Arc<Mutex<T>>` is useful, but it is not the answer to every ownership problem. Prefer simpler ownership, message passing, or immutable data when possible.

### Mistake 6: Ignoring Error Design

String errors are easy, but typed errors make libraries and services easier to maintain.

### Mistake 7: Reaching for Unsafe Too Soon

Most Rust applications need little or no unsafe code. Learn safe Rust deeply before writing unsafe Rust.

---

## Advanced Rust User Checklist

You are becoming an advanced Rust user when you can:

```text
Design APIs with clear ownership semantics.
Choose between owned values, borrowed values, and Cow.
Use lifetimes to express relationships rather than silence errors.
Model domains with structs, enums, and typed errors.
Write ergonomic Result-based APIs.
Use traits, generics, impl Trait, and dyn Trait appropriately.
Use iterators without losing track of ownership.
Design modules and crates with clean boundaries.
Write unit, integration, doc, and property-based tests.
Use rustfmt, Clippy, and Cargo workflows naturally.
Build reliable CLI tools.
Build async HTTP services with bounded concurrency.
Use Arc, Mutex, channels, and tasks safely.
Handle cancellation, timeouts, and graceful shutdown.
Profile before optimizing.
Reduce unnecessary allocation and cloning.
Understand when unsafe is necessary and how to wrap it safely.
Read and understand production Rust crate APIs.
```
