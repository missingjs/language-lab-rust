# Rust Learning Roadmap

A roadmap for growing from Rust beginner to advanced Rust user, with language mechanisms, API contracts, concurrency reasoning, and reproducible engineering evidence as the core goals.

Use Rust stable and the Rust 2024 edition, recording the actual compiler, target, lockfile, dependency versions, and enabled features. Async mechanism labs use Tokio 1.x. Miri and fuzzing may use separately recorded nightly toolchains. See the [shared course requirements](projects/README.md#environment-and-shared-completion-requirements).

The [twenty-course sequence](projects/README.md#the-twenty-courses) combines 11 retained assignments, 7 new courses, and 2 expanded assignments. It distinguishes core language work from optional ecosystem depth; reading a topic here does not replace its practical verification.

Rust is different from Go, Elixir, and Scala in one crucial way: its learning curve is dominated by **ownership, borrowing, lifetimes, and memory-safety-oriented design**. Rust is not merely a systems language with modern syntax; it teaches a different way to think about data, mutation, aliasing, concurrency, and API boundaries.

This roadmap is the knowledge map: what to learn and in what order. For the project sequence that turns these concepts into practical engineering skill, see [projects/README.md](projects/README.md).

---

## 1. Understand Rust's Positioning

Rust is best understood as a language for writing software that needs some combination of:

- memory safety without garbage collection
- predictable performance
- low-level control
- strong static guarantees
- safe concurrency
- reliable command-line tools
- networking services
- embedded or systems programming
- WebAssembly

Rust is not optimized for rapid scripting in the way Python is, nor is it as minimal as Go. Its power comes from making important correctness constraints explicit in the type system.

A good Rust developer is not someone who fights the borrow checker forever. A good Rust developer learns how to design data ownership so the borrow checker becomes a guide instead of an enemy.

---

## 2. Stage One: Master Rust Syntax and Tooling

The first stage is becoming productive in the Rust workflow. Before tackling ownership in depth, you should be able to create projects, run tests, and read the standard tooling output without friction.

Before Project 01, learn bindings/mutability, shadowing, basic types, expression results and semicolons, functions, conditionals/loops, integer boundaries, and explicit conversions. Use small fixed-data examples before combining file I/O and serialization. Practice reading a compiler diagnostic, reducing the example, explaining the cause, and verifying a repair.

### 2.1 Cargo and Tooling

Learn the standard Rust workflow early:

```bash
cargo new myapp
cargo build
cargo run
cargo test
cargo fmt
cargo clippy
cargo doc --open
```

Key topics:

- `Cargo.toml`
- crates and packages
- binary vs library crates
- modules
- workspaces
- dependency versions
- feature flags
- `rustfmt`
- Clippy
- documentation tests

Rust's tooling is one of its strongest advantages. Treat Cargo, rustfmt, Clippy, and tests as part of the language experience, not as optional extras.

---

## 3. Stage Two: Ownership, Borrowing, and Lifetimes

This stage is the heart of Rust's learning curve. Most of what makes Rust unique — and most of what early learners struggle with — lives here. The goal is not to memorize rules, but to develop a feel for designing data ownership so the borrow checker becomes a collaborator.

### 3.1 Ownership and Moves

Ownership is the foundation of Rust.

You must understand:

- every value has one owner
- moving transfers ownership
- values are dropped when their owner goes out of scope
- types may or may not implement `Copy`
- cloning is explicit

Example:

```rust
let name = String::from("Alice");
let other = name;
// name can no longer be used here
```

Important questions to practice asking:

```text
Who owns this value?
Is this value moved, borrowed, cloned, or copied?
When will this value be dropped?
Should this function take ownership or borrow?
```

---

### 3.2 Borrowing and References

Borrowing lets code access data without taking ownership.

You need to internalize:

- immutable references: `&T`
- mutable references: `&mut T`
- many immutable references are allowed
- only one mutable reference is allowed at a time
- references must not outlive the data they point to

Example:

```rust
fn len(s: &String) -> usize {
    s.len()
}
```

In idiomatic Rust, prefer `&str` over `&String` when you only need string data:

```rust
fn len(s: &str) -> usize {
    s.len()
}
```

This is the beginning of good Rust API design.

---

### 3.3 Lifetimes

Lifetimes describe how long references are valid.

At first, most lifetimes are inferred. Later, you need to understand explicit lifetime parameters:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Learn lifetimes as a way to express relationships between references, not as random compiler annotations.

Key topics:

- lifetime elision
- lifetime parameters
- references in structs
- `'static`
- owned data vs borrowed data
- avoiding unnecessary lifetime complexity

A practical rule:

> If lifetimes become too complicated, reconsider your ownership model.

---

### 3.4 Advanced Borrowing and Data Representation

Before [Project 07: Text Store](projects/07-text-store/README.md), study reborrowing, field/slice splitting, multiple lifetimes, outlives constraints, `T: 'a` versus `&'a T`, and lifetime subtyping/variance. Use small passing and failing cases to explain shared versus mutable access and function parameter/return substitution.

Compare owned values, borrowed views, `Cow`, reference counting, indices, and stable handles. Analyze deletion, stale identifiers, reference cycles, and why self-referential data complicates ordinary mutation. `'static` bounds describe reference constraints, not a requirement that a value live forever.

Project 06 provides parsing experience; Project 07 makes returned-reference contracts and handle invalidation explicit.

---

## 4. Stage Three: Data Modeling and Error Handling

Once ownership feels natural, the next stage is modeling your domain in Rust's type system. This is where Rust starts to feel different from "C with safety": you express invariants as types, replace runtime branches with pattern matching, and treat errors as values.

### 4.1 Structs, Enums, and Pattern Matching

Rust's data modeling power comes from structs and enums.

Use structs for product types:

```rust
struct User {
    id: UserId,
    email: Email,
}
```

Use enums for sum types:

```rust
enum PaymentStatus {
    Pending,
    Paid,
    Failed(String),
}
```

Pattern matching is central:

```rust
match status {
    PaymentStatus::Pending => "waiting",
    PaymentStatus::Paid => "done",
    PaymentStatus::Failed(reason) => reason.as_str(),
}
```

Learn to model states explicitly instead of using strings, booleans, or magic integers.

---

### 4.2 Option and Result

Rust avoids null and exceptions for normal control flow.

Use `Option<T>` when a value may be absent:

```rust
fn find_user(id: UserId) -> Option<User> {
    // ...
}
```

Use `Result<T, E>` when an operation may fail:

```rust
fn read_config(path: &str) -> Result<Config, ConfigError> {
    // ...
}
```

Learn:

- `map`
- `and_then`
- `unwrap_or`
- `ok_or`
- `?`
- custom error enums
- `thiserror`
- `anyhow`

Recommended distinction:

```text
Library code: prefer typed errors, often with thiserror.
Application boundary code: anyhow can be practical.
```

---

### 4.3 Traits and Generics

Traits define shared behavior.

```rust
trait Repository {
    fn find(&self, id: UserId) -> Result<Option<User>, RepoError>;
}
```

Learn:

- trait definitions
- trait implementations
- default methods
- generic functions
- trait bounds
- associated types
- `impl Trait`
- trait objects: `dyn Trait`

Important distinction:

```text
Static dispatch: generics and impl Trait
Dynamic dispatch: dyn Trait
```

Use generics when performance and compile-time specialization matter. Use trait objects when runtime polymorphism and simpler boundaries are useful.

---

### 4.4 Advanced Trait Contracts and API Evolution

Before [Project 08: Rule Library](projects/08-rule-library/README.md), deepen associated types versus generic parameters, GATs, higher-ranked trait bounds, blanket implementations, coherence/orphan rules, implementation overlap, dyn compatibility, object lifetimes, and auto-trait contracts. Check returned `impl Trait` capture behavior on the recorded compiler.

Compare newtypes, construction validation, proportionate typestate/builders, and conversion semantics (`From`/`TryFrom`, `AsRef`/`Borrow`). Separate typed library errors from application diagnostics. Evaluate SemVer effects on fields, variants, bounds, auto traits, and features using independent callers.

Learn advanced features through small explanatory examples; use them in the main library only when the requirement warrants their complexity.

---

## 5. Stage Four: Iterators, Smart Pointers, and API Design

This stage is about writing idiomatic Rust at a larger scale: composing transformations with iterators, reaching for smart pointers only when ownership genuinely needs them, and shaping module boundaries that communicate intent. Many ownership headaches at this point are actually API design problems.

### 5.1 Iterators and Closures

Rust iterators are powerful, zero-cost abstractions when used well.

Learn:

- `iter`
- `iter_mut`
- `into_iter`
- `map`
- `filter`
- `fold`
- `collect`
- `find`
- `any`
- `all`
- `flat_map`

Example:

```rust
let emails: Vec<String> = users
    .iter()
    .filter(|user| user.active)
    .map(|user| user.email.clone())
    .collect();
```

Pay attention to ownership in iterator chains. Many Rust learning problems appear here.

---

### 5.2 Smart Pointers and Interior Mutability

Study stack/heap relationships, indirection, RAII, `Drop`, and ownership-dependent release before [Project 05: Resource Lifecycles](projects/05-resource-lifecycle/README.md). Predict traces, compare them with observations, and distinguish leaks, panics, and undefined behavior. Include `Cell`, `Weak`, reference cycles, and runtime versus compile-time borrow checking.

Eventually you need to understand:

- `Box<T>`
- `Rc<T>`
- `Arc<T>`
- `RefCell<T>`
- `Mutex<T>`
- `RwLock<T>`
- `Cow<'a, T>`

Typical uses:

```text
Box<T>: heap allocation and recursive data structures
Rc<T>: shared ownership in single-threaded code
Arc<T>: shared ownership across threads
RefCell<T>: runtime-checked interior mutability
Mutex<T>: synchronized mutation across threads
Cow<T>: clone-on-write borrowed-or-owned data
```

Do not overuse these to avoid learning ownership. Use them when the ownership model genuinely requires them.

---

### 5.3 Modules, Crates, and API Design

Rust's module system matters for maintainability.

Learn:

- `mod`
- `pub`
- `pub(crate)`
- `use`
- library crate vs binary crate
- integration tests
- workspaces

Good Rust APIs make ownership clear:

```rust
fn parse(input: &str) -> Result<Document, ParseError>
fn save(path: impl AsRef<Path>, document: &Document) -> Result<(), SaveError>
fn into_inner(self) -> Inner
```

Useful naming patterns:

```text
as_*      borrowed view
to_*      converted or cloned value
into_*    consumes self
try_*     fallible operation
from_*    conversion constructor
new       ordinary constructor
```

---

## 6. Stage Five: Concurrency and Async

Rust's type system rules out many concurrency bugs at compile time, but you still need to design ownership and lifetimes deliberately. Start with synchronous threads and channels, then move to async only after the rest of the language feels comfortable.

### 6.1 Threads and Message Passing

Start with standard library threads:

```rust
std::thread::spawn(|| {
    println!("hello from another thread");
});
```

Learn:

- thread spawning
- join handles
- channels
- `Send`
- `Sync`
- `move` closures

Rust's type system prevents many data races at compile time, but you still need good design.

---

### 6.2 Shared State Concurrency

[Project 09: Standard-Library Work Queue](projects/09-sync-work-queue/README.md) comes before the first async application. Study scoped threads, `Condvar` predicate loops, bounded channels/queues, poisoning, lock order, shutdown wakeups, and panic propagation. Establish state invariants and controlled race tests rather than relying on sleep-based timing or assumed fairness.

Study atomicity, modification order, happens-before, `Relaxed`, Acquire/Release, and `SeqCst` using bounded examples. Distinguish atomic observations from multi-step business transactions; a stronger ordering does not repair an incorrect protocol.

Learn:

- `Arc<T>`
- `Mutex<T>`
- `RwLock<T>`
- atomics
- poisoning
- lock scope
- deadlock avoidance

Example:

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
```

Important question:

> Should this be shared mutable state, message passing, or independent ownership?

---

### 6.3 Async Rust

Async Rust is powerful but complex. Learn it after ownership, traits, and error handling.

Key topics:

- `async fn`
- `.await`
- futures are lazy until polled
- executors and runtimes
- Tokio
- async channels
- cancellation
- timeouts
- `Send` futures
- blocking vs non-blocking work

Common ecosystem tools:

- Tokio
- reqwest
- axum
- hyper
- tower
- sqlx
- tracing

Do not block inside async tasks unless you understand `spawn_blocking` or equivalent patterns.

---

### 6.4 Async Mechanisms and Cancellation Contracts

After [Project 10: URL Checker](projects/10-url-checker/README.md), use [Project 11: Async Lifecycles](projects/11-async-lifecycle/README.md) to study `poll`, `Pending`/`Ready`, wakeups, executor responsibilities, and task creation versus awaiting. Learn safe stack/heap pinning, `Pin`/`Unpin`, and why moving a pointer differs from moving its pinned value.

Explain how values held across `.await` affect `Send`; compare current-thread and multi-thread execution. Contrast joins, selection, dropping a future, dropping a task handle, abort, cooperative cancellation, and timeout. Include partial-progress cancellation, bounded queues and in-flight tasks, result collection, and resource release. Isolate blocking work and account for already-started blocking operations.

Reuse these contracts in Projects 14 and 16. An HTTP client or runtime call alone does not establish cancellation safety.

---

## 7. Stage Six: Engineering Practices

Mature Rust code is observable, tested, and measured. This stage is the same kind of work that any production language requires, but Rust's tooling makes it unusually rewarding to invest in early.

### 7.1 Testing and Quality

Learn testing early:

```rust
#[test]
fn parses_valid_input() {
    // ...
}
```

Key topics:

- unit tests
- integration tests
- doc tests
- test fixtures
- property-based tests
- snapshot tests
- benchmarking
- fuzzing basics

Useful tools:

```text
cargo test
cargo fmt
cargo clippy
cargo doc
cargo bench
proptest
insta
criterion
cargo-nextest
```

A strong Rust workflow usually includes:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

---

### 7.2 Performance and Memory

[Project 17: Performance Lab](projects/17-performance-lab/README.md) requires a correctness reference, representative input generation, repeated measurements, profiler/allocation evidence, and an explicit cost hypothesis. Study locality, padding, false sharing, code size, inlining/dispatch, LTO, and target/build settings as explanations for measured costs. Compare benefits with complexity and report inconclusive or negative findings.

Rust gives you low-level control, but you still need measurement.

Learn:

- stack vs heap
- allocation behavior
- cloning costs
- iterator performance
- `Vec` capacity
- string allocation
- zero-copy parsing
- memory layout
- `#[derive]` costs and benefits
- benchmarking with Criterion
- profiling with flamegraphs or platform profilers

Important principle:

> Optimize after measuring. Rust makes performance possible, not automatic.

---

### 7.3 Invariant Verification and Failure Reduction

In [Project 19](projects/19-invariant-verification/README.md), derive properties from public contracts, add compile-pass/fail checks, preserve seeds and minimized fuzz inputs, and model a small synchronization protocol with Loom primitives. Establish execution budgets and explain model assumptions and exploration limits. Reuse Miri evidence from Project 18; distinguish executed-path checks from input generation and bounded schedule exploration.

### 7.4 Workspaces and Maintenance

[Project 20](projects/20-workspace-maintenance/README.md) integrates an analysis library, async processor, and CLI. Study optional/default features, feature unification, duplicate dependencies, MSRV, upgrades, lockfiles, and toolchain/platform reproducibility. Check supported configurations and downstream callers rather than assuming source-level internal changes are invisible.

Practice reducing a failure, recording environment and events, proposing a falsifiable hypothesis, verifying a repair, and separating observed facts from untested explanations. Reuse performance and verification artifacts to justify architectural choices.

---

## 8. Stage Seven: Advanced and Specialized Topics

The common advanced path includes bounded unsafe review and implementation in Project 18. FFI, embedded, WebAssembly, and deeper systems specializations remain optional. Do not start with them or try to pursue every direction at once.

### 8.1 Unsafe Rust

[Project 18](projects/18-unsafe-boundary/README.md) uses a fixed-capacity `[MaybeUninit<T>; N]` buffer to study initialization prefixes, ownership transfer, shared slice validity, destruction, and surrounding-code panic unwinding. Cover alignment, provenance/aliasing, layouts, `ManuallyDrop`, `UnsafeCell`, and `PhantomData`; review rather than casually add manual `Send`/`Sync` or pinning implementations. Separate language guarantees from tool-specific models and acknowledge memory-model limits.

Run normal/edge tests and Miri on a recorded separate toolchain. Keep the interface bounded; growth, general arenas, and allocators are outside the required assignment. The optional FFI branch uses a minimal C opaque handle with explicit allocation/release, ABI, and error/unwind contracts.

Unsafe Rust is advanced. You do not need it for most application development.

Learn only after you are comfortable with safe Rust.

Topics:

- raw pointers
- unsafe functions
- unsafe traits
- FFI
- aliasing rules
- invariants
- soundness
- `MaybeUninit`
- `Pin`

Rule of thumb:

```text
Minimize unsafe blocks.
Document invariants.
Wrap unsafe internals in safe APIs.
Prefer proven crates when possible.
```

---

### 8.2 Web, CLI, Systems, and Specialized Ecosystems

After core Rust, choose one or two directions.

#### CLI and Developer Tools

Learn:

- `clap`
- `serde`
- `toml`
- `anyhow`
- `thiserror`
- `tracing`
- file I/O
- terminal output

#### Web Backend

Learn:

- Tokio
- axum
- tower
- serde
- sqlx
- reqwest
- tracing
- metrics
- graceful shutdown

#### Systems Programming

Learn:

- OS APIs
- memory layout
- FFI
- networking
- parsers
- binary protocols
- unsafe boundaries

#### WebAssembly

Learn:

- wasm-bindgen
- wasm-pack
- JS interop
- browser APIs
- size optimization

#### Embedded Rust

Learn:

- `no_std`
- HAL crates
- embedded-hal
- interrupts
- memory constraints
- hardware-specific tooling

---

## 9. Recommended Learning Order

The project index is the authoritative numbered course order:

1. **Projects 01–06:** basic syntax/tooling, ownership, data/error modeling, resources, and parsing.
2. **Projects 07–09:** borrowed/handle designs, public abstractions, and synchronous coordination.
3. **Projects 10–11:** a first async application followed by runtime and cancellation experiments.
4. **Projects 12–16:** HTTP/database applications, async workers, shared-state structures, and streaming.
5. **Projects 17–20:** performance evidence, bounded unsafe safety, invariant verification, and maintenance.

Study concepts before the corresponding assignment and follow the prerequisites in each README. Projects 19–20 reuse earlier components; they are not unrelated new products. For a core-language route, web/database/streaming depth can be deferred as described in the [time-limited route](projects/README.md#if-time-is-limited).

Compiler internals, procedural macros, embedded/`no_std`, Wasm, complex lock-free algorithms, and platform-specific optimization remain follow-up specializations. Neither the shortened route nor completing a set of framework APIs demonstrates mastery of every Rust domain.

---

## Final Advice

Rust rewards patience. The early learning curve can feel slower than Go or Python because the compiler forces you to answer design questions that other languages allow you to postpone.

That difficulty is also the value of Rust.

A mature Rust developer does not merely know syntax. A mature Rust developer can design ownership, model errors, manage concurrency, and expose APIs that make invalid states difficult to represent.

The goal is not to silence the compiler. The goal is to collaborate with it.
