[← Back to projects](../README.md)

# Project 3: Config Loader

## Goal

Build a small configuration loader.

Features:

```text
Load TOML or JSON config.
Apply defaults.
Override with environment variables.
Validate values.
```

## Focus

- `serde`
- custom error types
- `Path` and `PathBuf`
- API design
- `Option`
- `Result`

## Completion Standard

```text
Invalid config returns clear typed errors.
The public API is small and easy to use.
Tests cover valid and invalid configs.
```
