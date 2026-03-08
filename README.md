<div style="display: flex; flex-direction: column; justify-content: center; align-items: center;" align="center">
    <h1><code>optimization-n-safety</code></h1>
    <h4>Built with <a href="https://rust-lang.org/">🦀</a></h4>
</div>

[![main](https://github.com/arthurhovhannisyan31/optimization_n_safety/actions/workflows/code-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/optimization_n_safety/actions/workflows/code-validation.yml)
[![main](https://github.com/arthurhovhannisyan31/optimization_n_safety/actions/workflows/packages-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/optimization_n_safety/actions/workflows/packages-validation.yml)

## Overview

This is a demo workspace with performance and safety tools application results.

The used tools cover both static and runtime code analysis and cover the following objectives:

- code runtime validation
- memory usage and boundary safety
- UB and memory leak prevention

## Description

The workspace contains two applications: [broken-app](./modules/broken-app)
and [reference-app](./modules/reference-app).

The main job was done on [broken-app](./modules/broken-app), the [reference-app](./modules/reference-app) exists as a
correct implementation example.

You can find the initial state
of [broken-app](./modules/broken-app) at `before` tag, and initial checks artifacts
can be found in [logs](./modules/broken-app/artifacts/before/logs).

The following optimization workflow was applied:

- Profiling
- Benchmark
- Code optimization
- Benchmark

The initial state of [broken-app](./modules/broken-app) contains several bugs, and fixes could be found at
`after` tag.

The following code analysis tools are used in
repo [CI](https://github.com/arthurhovhannisyan31/optimization_n_safety/actions/workflows/code-validation.yml):

- Cargo tests
- Miri static code analysis
- Valgrind code runtime analysis
- Code compiler sanitizers

The full list of commands can be found in
the [makefile](https://github.com/arthurhovhannisyan31/optimization_n_safety/blob/main/Makefile#L14).

Performance results `before` and `after` changes can be found in the [artifacts](./modules/broken-app/artifacts) folder.

The artifacts include `cargo bench` logs, the `criterion` logs, and `flamegraph` visualization.
Performance measurements were done a on single [demo](./modules/broken-app/src/bin/demo.rs) that includes invocation of
all relevant functions.

## Usage

Please build the project locally as follows:

```bash
RUSTFLAGS="-C force-frame-pointers=yes" cargo build --release
```

To see the difference in code performance, check out tag `before`:
`before`.

Run the following commands:

```bash
cargo flamegraph --flamechart --root -p broken-app --bin demo -F 60000
cargo bench --bench baseline > baseline.txt
cargo bench --bench criterion > criterion.txt
```

> Please check your platform sampling frequency limits to get a detailed report
> and provide the `-F <frequency>` argument to the `cargo flamegraph`

Once you are done with initial measurements, please back up artifacts and run the same commands on the HEAD commit and
observe
the results.

> Current tool set was tested on Linux machine, please check your platform performance tools availability

## Stack

- [Rust](https://rust-lang.org/)
- [Cargo bench](https://doc.rust-lang.org/cargo/commands/cargo-bench.html)
- [Compiler sanitizers](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html)
- [Criterion](https://github.com/bheisler/criterion.rs)
- [Miri](https://github.com/rust-lang/miri)
- [Perf](https://perfwiki.github.io/main/)
- [Valgrind](https://valgrind.org/)

## Credits

Crate implemented as part of the [Yandex practicum](https://practicum.yandex.ru/) course.

## License

Licensed under either of your options.

* Apache License, Version 2.0, [LICENSE-APACHE](./LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0
* MIT license [LICENSE-MIT](./LICENSE_MIT) or http://opensource.org/licenses/MIT
