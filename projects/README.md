# Project-Based Learning Path

Learn Rust through a sequence of application projects and focused mechanism labs. Reuse components across courses so that language reasoning, verification, and maintenance receive as much attention as adding features.

For the knowledge map and topics to study before each assignment, see [../roadmap.md](../roadmap.md). These are assignment specifications, not completed implementations or reference solutions.

## The Twenty Courses

The sequence contains **11 existing courses, 7 new courses, and 2 expanded existing courses**. Numbers in directory names, course headings, and this table use the new sequence. The original-number column preserves the mapping from the earlier thirteen-project path.

- **Existing:** original assignment scope retained; numbering and origin metadata updated.
- **New:** added to cover a language mechanism, design skill, or verification/maintenance gap.
- **Expanded existing:** original purpose retained with stronger required work and completion criteria.

| No. | Course | Status | Original no. | Main focus |
| --- | --- | --- | --- | --- |
| 01 | [CLI TODO Tool](01-cli-todo/README.md) | Existing | 01 | Cargo, data modeling, ownership basics, persistence |
| 02 | [Log Analyzer](02-log-analyzer/README.md) | Existing | 02 | Streaming file I/O, strings, maps, iterators |
| 03 | [Config Loader](03-config-loader/README.md) | Existing | 03 | Configuration, typed errors, a small public API |
| 04 | [Domain Validator](04-domain-validator/README.md) | Existing | 04 | Newtypes, enums, validation, pure domain logic |
| 05 | [Resource Lifecycles and Interior Mutability](05-resource-lifecycle/README.md) | New | — | RAII, Drop, Rc/Weak, RefCell, observable behavior |
| 06 | [Mini Parser](06-mini-parser/README.md) | Existing | 05 | Borrowing, lifetimes, recursive data, parsing |
| 07 | [Text Store: Borrowed Views and Stable Handles](07-text-store/README.md) | New | — | Borrowing contracts, lifetime relationships, handle invalidation |
| 08 | [Extensible Rule Library and Public API Evolution](08-rule-library/README.md) | New | — | Trait contracts, abstraction choices, downstream compatibility |
| 09 | [Standard-Library Bounded Work Queue](09-sync-work-queue/README.md) | New | — | Threads, condition variables, shutdown, panic policy |
| 10 | [Concurrent URL Checker](10-url-checker/README.md) | Existing | 06 | First async application, Tokio, limits, timeouts |
| 11 | [Futures, Cancellation, and Async Task Lifecycles](11-async-lifecycle/README.md) | New | — | Polling, pinning, cancellation safety, blocking isolation |
| 12 | [In-Memory HTTP API](12-http-api/README.md) | Existing | 07 | axum, shared state, handlers, integration tests |
| 13 | [Database-Backed API](13-db-api/README.md) | Existing | 08 | sqlx, migrations, transactions, repository boundaries |
| 14 | [Background Job Queue](14-job-queue/README.md) | Existing | 09 | Async workers, retries, cancellation, tracing |
| 15 | [Cache or Rate Limiter](15-cache-or-rate-limiter/README.md) | Existing | 10 | Generics, shared state, time-dependent logic |
| 16 | [Streaming Log Tail Service](16-log-tail-service/README.md) | Existing | 11 | Streaming, backpressure, disconnects, task cleanup |
| 17 | [Performance Models, Measurement, and Optimization](17-performance-lab/README.md) | Expanded existing | 12 | Correctness references, profiling, repeatable benchmarks, tradeoffs |
| 18 | [Unsafe Abstractions and an Initialization Buffer](18-unsafe-boundary/README.md) | Expanded existing | 13 | MaybeUninit, ownership transfer, panic safety, Miri |
| 19 | [Invariant Verification and Failure Reduction](19-invariant-verification/README.md) | New | — | Property testing, bounded fuzzing, Loom, minimal failures |
| 20 | [Workspaces, Compatibility, and Engineering Maintenance](20-workspace-maintenance/README.md) | New | — | Component integration, features, MSRV, upgrades, compatibility |

## Recommended Order and Prerequisites

Use the numbered order as the default path. Study each course's concepts before its required work; the labs assess application and explanation rather than introducing advanced terms without preparation.

1. **01–06: foundations.** Build small tools, model data/errors, observe resource behavior, and work with borrowed text.
2. **07–09: advanced data and synchronization.** Design borrowing/handle interfaces, evolve a library API, and establish thread-based coordination before async.
3. **10–16: async and application engineering.** Start with a URL checker, examine the runtime/cancellation model, then apply it in services and workers.
4. **17–20: evidence and maintenance.** Measure costs, audit a bounded unsafe abstraction, verify invariants, and integrate reusable components.

Project 11 deepens the async model after Project 10's first application. Projects 19 and 20 reuse earlier components and evidence instead of requiring new business systems. New and expanded course READMEs list their prerequisites explicitly.

## Environment and Shared Completion Requirements

- Use Rust stable with the Rust 2024 edition. Record `rustc -Vv`, Cargo, target/OS, resolved dependency versions, and enabled features; keep the lockfile and avoid unrelated upgrades during a course.
- Use Tokio 1.x for the async labs and record its resolved version/features. Record separate dated nightly toolchains for Miri and cargo-fuzz when required; keep the main development toolchain stable.
- Prefer standard-library/local fixtures for mechanism labs. Web/database courses document their own service and dependency setup. No cloud deployment is required.
- For each implemented course, include runnable usage, normal and relevant failure cases, and a brief ownership/error/design explanation. Match test depth to risk and the course's completion criteria.
- Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo doc --no-deps`, and `cargo build --release` for the implemented package or workspace and selected supported configuration. Documentation tests are part of verification for public libraries. Project 20 adds the feature/MSRV matrix.
- Keep intentionally failing code outside successful build targets, or use explicit compile-fail tests. Keep expected panics isolated and explain them.
- For experiments, record the expected result before execution, compare actual evidence, and explain discrepancies. A successful tool run is not proof beyond its checked paths, inputs, or states.

These common workflow requirements support the retained assignments without changing their domain feature sets. Specialized tools, benchmarks, concurrency, or production concerns are required only where the course calls for them.

## If Time Is Limited

For a first **foundation pass**, prioritize 01–06, including the resource lab. For a **core language continuation**, prioritize 07–11 and 17–20 while reusing the text library and runner. Project 15 is a useful additional shared-state design exercise.

Projects 12–13 and 16 emphasize web/database/streaming ecosystems; they can be deferred when language depth is the immediate goal. No abbreviated route should be treated as complete coverage of advanced Rust. The synchronous queue and async lifecycle lab train different models and do not replace one another.

## Three-Version Method

For application projects:

1. **Make it work:** implement the bounded required behavior and visible failures.
2. **Make it idiomatic:** clarify ownership, errors, interfaces, modules, and unnecessary clones.
3. **Make it robust:** add the relevant lifecycle, observability, verification, or performance evidence required by the course.

For mechanism labs, use **predict → observe → explain/refine** instead. Do not force networking, logging, timeouts, or benchmarks into every experiment. Keep advanced features in focused examples unless they improve the actual design.

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
Explain resource release, runtime borrow checking, and handle invalidation.
Use threads, condition variables, atomics, channels, and async tasks with explicit contracts.
Handle cancellation, timeouts, and graceful shutdown.
Profile before optimizing.
Reduce unnecessary allocation and cloning.
Understand when unsafe is necessary and how to wrap it safely.
Validate public API evolution against downstream callers.
Use bounded fuzzing, Miri, and Loom with explicit coverage limits.
Maintain a workspace with checked feature and MSRV claims.
Read and understand production Rust crate APIs.
```
