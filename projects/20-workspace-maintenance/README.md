[← Back to projects](../README.md)

# Project 20: Workspaces, Compatibility, and Engineering Maintenance

**Status:** New.

## Goal

Integrate an existing analysis library, bounded async processor, and command-line adapter into a maintainable workspace. Validate build boundaries, compatibility claims, and one controlled upgrade or API evolution.

## Prerequisites and Setup

- Reuse Project 08's library, Project 11's bounded runner (or the relevant part of Project 14), Project 17's measurements, and Project 19's verification artifacts.
- Study workspace/package responsibilities, Cargo features, dependency resolution, SemVer, and minimum supported Rust version (MSRV) before implementation.
- Use stable Rust 2024 for normal development and retain the lockfile. A declared MSRV must support the chosen edition, syntax, dependencies, and advertised configurations.
- Prepare finite local input files and controlled failure/timeout operations; no database, container, or deployed service is required.

## Focus

- Module versus crate boundaries, runtime-independent libraries, and workspace integration
- Optional/default features, additive feature design, feature unification, and duplicate dependency versions
- Toolchain, lockfile, target platform, MSRV, and upgrade policy
- Public errors, bounds, auto traits, observable behavior, and downstream compatibility
- Failure diagnosis, structured events, regression checks, and design evidence

## Required Work

1. Organize the reused analysis library, async adapter, and CLI as workspace members with clear responsibilities. Keep runtime details out of the analysis library unless a documented contract requires them.
2. Accept finite local inputs, summarize per-item results, and expose failure and exit behavior. Isolate blocking file work and preserve the runner's defined drain/cancel boundaries.
3. Define a small meaningful feature surface, including an optional capability. Document default and supported combinations, inspect feature unification, and justify rather than automatically remove duplicate dependency versions.
4. Declare an MSRV and upgrade strategy. Check the supported build/test surface with the development toolchain and the declared minimum, documenting any separately scoped developer-tool requirements.
5. Perform one controlled dependency upgrade or public API change. Keep a before/after record with downstream callers, changed behavior, compatibility classification, and regression verification. Isolate candidate state so results can be reproduced.
6. Reuse the performance and verification evidence to compare at least one architectural alternative. Do not add an unrelated benchmark or new business subsystem just for this course.

## Deliverables

- Runnable workspace, CLI usage, library documentation, and reproducible quality checks
- Supported feature/toolchain matrix, lockfile, and upgrade/compatibility policy
- A design/diagnostic note with a change comparison, failure evidence, and relevant performance results

## Completion Standard

- Normal input, one-item failure, saturated capacity, timeout, drain, and cancellation have reproducible checks. Tasks/resources finish under the documented policy.
- Public usage passes unit, integration, and documentation tests; intentional compile-fail cases remain isolated.
- Formatting, Clippy, documentation generation, tests, and release builds pass for the supported configurations.
- Default, no-default (if supported), and explicitly supported feature combinations are checked; do not claim all subsets without testing them.
- The declared MSRV has an actual check using compatible dependency resolution; retaining a lockfile alone is not sufficient evidence.
- A fresh reader can run the workspace and understand its ownership, error paths, runtime boundaries, and maintenance tradeoffs.

## Boundaries

Do not require every component to be generic or async, incorporate the unsafe buffer into production paths, publish crates, or deploy services. Projects 12–13 and 16 are useful application branches, not prerequisites for this local capstone.
