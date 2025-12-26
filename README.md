# Advent of Code Solutions

This repository contains my solutions for [Advent of Code](https://adventofcode.com),
implemented primarily in Rust. Each day's solution is organized as a separate crate,
following the naming convention `aoc-yyyy-dd` (e.g., `aoc-2025-01` for Day 1 of 2025).

## Philosophy

My approach emphasizes:

- **Efficiency:** Solutions are designed to run quickly and handle the provided inputs well.
- **Robustness:** Code avoids panics and handles edge cases gracefully.
- **Maintainability:** Readable, well-structured code that is easy to revisit and extend.
- **Pragmatism:** I focus on solving the problem effectively, without excessive micro-optimizations for specific inputs.

## Repository Structure

- Each day's solution is a separate crate in its own directory.
- The root directory contains a workspace-only `Cargo.toml` that aggregates all daily crates.

## Running Solutions

### Rust

To run a specific day's solution, use the following command from the repository root:

```sh
cargo run --release -p aoc-yyyy-dd
```

Replace `yyyy` and `dd` with the desired year and day, respectively.

Example for Day 5 of 2025:

```sh
cargo run --release -p aoc-2025-05
```

For simplicity, the input is simply baked in as a `static &str` in each day's `input.rs` as `INPUT`.

For testing, you can run:

```sh
cargo test --release -p aoc-yyyy-dd
```

### Python

Selected problems, particularly ones utilizing SIMD, uses Python/numPy for performance.
To run these solutions, navigate to the respective day's directory and execute:

```sh
python name_of_file.py
```

Some solutions may have their main file named as `test_xxx.py` or similar so to save
time on separating PyTest code from the main solution code; while not ideal, this is
done for expediency.

### C

Some problems have C implementations for learning purposes. These folders are suffixed with `-c`.

To compile and run a C solution, navigate to the respective day's directory and use:

```sh
cat input.txt | make run
```

Typically, `FLAGS` will be available to set optimization levels or other compiler flags,
with `VERBOSE` being the most common to enable detailed output.

You can enable this by:

```sh
cat input.txt | FLAGS=VERBOSE make run
```

In contrast to their Rust counterparts, these C solutions are designed to read input
from `stdin` for no particular reason other than expediency.

Most C modules contains their own `int main()` function that acts as a unit test
when run. Some requires `stdin` input to function properly. You can run tests for the
`func.c` module by:

```sh
ACTION=func FLAGS="UNIT_TEST VERBOSE" make run
```

`VERBOSE` here is optional, but useful for debugging.

There is also a unified `make test` target that will run all tests for the module:

```sh
make test
```

## Documentation

To build and view documentation for all crates in the workspace:

```sh
cargo doc --workspace --open
```

This will generate and open the documentation in your browser.
