# learning-rust

A project-driven learning lab for Rust, combining a knowledge map with progressively harder applications and focused mechanism experiments. The goal is to explain behavior, design clear ownership and API contracts, diagnose failures, and support engineering choices with evidence.

## Navigation

- [roadmap.md](roadmap.md) — the knowledge map, from language foundations to concurrency, safety, verification, and maintenance.
- [projects/README.md](projects/README.md) — the twenty-course action plan, prerequisites, environment, and completion workflow.

The path contains **11 existing courses, 7 new courses, and 2 expanded existing courses**. The project index maps original numbers to the new sequence. Each course has a self-contained README under `projects/`; implementations are left to the learner. Later courses reuse earlier components rather than requiring twenty unrelated codebases.

Use Rust stable with the Rust 2024 edition, retain lockfiles, and record the actual toolchain and dependency versions. Specialized verification tools use separately recorded toolchains where needed. See the [shared environment and quality requirements](projects/README.md#environment-and-shared-completion-requirements).
