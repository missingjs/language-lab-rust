# Rust Coding Conventions

## Tooling

- Run `cargo fmt` on every commit. Formatting is not a matter of taste in Rust — `rustfmt` is the standard.
- Run `cargo clippy` regularly and address its warnings. Only silence a Clippy lint with `#[allow(...)]` when you have a specific reason and document it.

## Naming

- `snake_case` for functions, variables, modules, and crates.
- `CamelCase` for types (structs, enums, traits) and enum variants.
- `SCREAMING_SNAKE_CASE` for `const` and `static` items.
- Method names that convert or borrow follow the standard conventions:

  | Prefix | Meaning |
  |--------|---------|
  | `as_`   | Borrowed view, cheap |
  | `to_`   | Allocating conversion |
  | `into_` | Consuming conversion |

## Imports

- Import types and functions explicitly. Avoid glob imports (`use foo::*`), except for prelude modules or well-known patterns like `use super::*` in test modules.
- Group imports by source: `std` / external crates / `crate` / `super` / `self`. Separate groups with a blank line.

  ```rust
  use std::path::PathBuf;

  use serde::{Deserialize, Serialize};

  use crate::config::Config;

  use super::parser::parse;
  ```

## Error Handling

- Prefer a dedicated error enum over `String` errors. Callers should be able to inspect and react to specific failures.
- Use `Result<T, E>` when failure needs explanation; use `Option<T>` when absence is normal.
- Avoid bare `unwrap()` in application and library code. It is acceptable in tests, examples, and states the programmer knows are unreachable. Prefer `?` for error propagation.

  ```rust
  // Avoid
  let config = read_config().unwrap();

  // Prefer
  let config = read_config().context("failed to read config")?;
  ```

## Ownership and Borrowing

- Prefer borrowed parameters (`&T`, `&str`) over owned parameters (`T`, `String`) when the function does not need to store the value.
- Avoid unnecessary `.clone()`. Cloning is sometimes the right call, but repeated clones often signal an ownership design that should be reconsidered.
- Do not reach for `Arc<Mutex<T>>` as a first reflex. Prefer simpler patterns: pass ownership down, use message passing, or refactor data layout before adding shared mutable state.

## Pattern Matching

- Prefer exhaustive `match` over chains of `if let` / `else if let` when dealing with an enum that has three or more variants.
- Avoid wildcard arms (`_ => {}`) on enums you control — a new variant should trigger a compile error so you don't silently miss it.

  ```rust
  // Avoid
  match status {
      Status::Active => handle_active(),
      _ => {} // new variants silently ignored
  }

  // Prefer
  match status {
      Status::Active => handle_active(),
      Status::Pending => handle_pending(),
      Status::Archived => handle_archived(),
  }
  ```

## Documentation and Tests

- Document public items with `///` comments. A one-liner is enough for simple items; add an example for non-obvious behavior.
- Write unit tests in the same file inside a `#[cfg(test)] mod tests { ... }` block. Integration tests go under `tests/`.
- Use `cargo test` frequently. Tests are a first-class part of the Rust ecosystem, not an afterthought.
