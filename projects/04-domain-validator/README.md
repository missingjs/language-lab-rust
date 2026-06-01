[← Back to projects](../README.md)

# Project 4: Domain Validator

## Goal

Build a pure domain validation library.

Example domain:

```text
User registration
Order checkout
Payment request
Inventory reservation
```

## Focus

- newtype pattern
- enums
- typed errors
- pure functions
- `Result`
- avoiding primitive obsession

## Example

```rust
struct Email(String);
struct Quantity(u32);

enum ValidationError {
    EmptyEmail,
    InvalidEmail,
    InvalidQuantity,
}
```

## Completion Standard

```text
Invalid states are harder to represent.
Business logic is independent of I/O.
Errors are modeled as enums.
```
