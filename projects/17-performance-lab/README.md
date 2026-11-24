[← Back to projects](../README.md)

# Project 17: Performance Models, Measurement, and Optimization

**Status:** Expanded existing. **Original project:** 12.

## Goal

Investigate an earlier text-processing component, locate a measured cost, and compare alternatives with reproducible correctness and performance evidence. A well-supported decision to keep the simpler implementation is a valid result.

## Prerequisites and Setup

- Reuse Project 02, 06, or 08; study ownership, allocation, dispatch, and release builds first.
- Use Criterion and a platform-available CPU profiler or allocation observation tool. Record tool versions and any installation/permission requirements before choosing the tool.
- Record `rustc -Vv`, Cargo version, lockfile, hardware/OS, build profile, relevant CPU settings, and input generation rules.
- Keep a correct reference implementation and deterministic small, large, and differently distributed inputs, including Unicode and boundary cases.

## Focus

- Owned versus borrowed data, `Cow`, clones, reference counting, capacity, and reuse
- Data layout, padding, indirection, locality, and false sharing
- Monomorphization, inlining, dynamic dispatch, iterator optimization, and code size
- Optimization levels, LTO, target CPU settings, and compile-time tradeoffs
- End-to-end versus microbenchmarks, noise, warmup, and dead-code elimination

## Required Work

1. State a falsifiable cost hypothesis and retain a baseline. Measure representative end-to-end behavior before choosing a narrow benchmark.
2. Use profiling or allocation evidence to identify a relevant hotspot. Explain profiler limitations and platform-specific conditions.
3. Compare at least two implementations or configurations. Options include capacity reuse, fewer allocations, borrowed output, `Cow`, or dispatch changes; do not assume any option must win.
4. Verify equivalent outputs against the reference and rerun failure-path tests before interpreting timings.
5. Repeat measurements across small/large inputs and different distributions. Record variance and distinguish meaningful effects from noise. Explain the role and limitations of `black_box` where used.
6. Discuss code complexity and at least one relevant non-timing cost, such as memory use, code size, compile time, or retained input lifetime. Explain how locality and representation might affect the result.

## Deliverables

- Reproducible input generation, baseline and candidate implementations, benchmarks, and correctness checks
- Raw or machine-readable benchmark results and hotspot/allocation evidence
- An investigation note linking hypothesis, evidence, conclusion, and maintenance tradeoffs

## Completion Standard

- Results can be reproduced from documented commands and recorded conditions.
- All candidates agree with the reference; a performance improvement cannot excuse changed behavior.
- At least one allocation, access-pattern, or dispatch hypothesis is tested rather than asserted.
- State where the conclusion applies and where it does not. Report inconclusive or negative results honestly.
- Inspect release builds and do not base conclusions on one debug run or a single timing.
- Pass shared quality checks and regression tests.

## Stretch Goals and Boundaries

Optionally compare concurrency levels, throughput/latency, overload behavior, or selected generated assembly for a concrete question. Do not require unsafe optimizations, SIMD, custom allocators, or reading compiler source.
