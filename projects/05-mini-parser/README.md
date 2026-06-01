[← Back to projects](../README.md)

# Project 5: Mini Parser

## Goal

Build a parser for a small format, such as:

```text
CSV subset
INI file
simple arithmetic expressions
Markdown headings
custom log format
```

## Focus

- borrowing with `&str`
- lifetimes
- enums
- recursive data structures
- error positions
- zero-copy parsing where possible

## Stretch Goals

```text
Avoid unnecessary String allocation.
Return spans or borrowed slices.
Add property-based tests.
```

This is one of the best projects for learning lifetimes in a practical way.
