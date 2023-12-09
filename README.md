# Advent of Code 2023

Nothing special, just my solutions for the 2023 edition of [Advent of Code](adventofcode.com) written in Rust.

Each day, I usually go for speed, and after getting both stars will refactor the solution to be (even more) idiomatic
Rust code.

## Required Rust Version

The project currently requires Rust's nightly compiler since it makes use of the
[return-position-impl-trait-in-traits](https://rust-lang.github.io/rfcs/3425-return-position-impl-trait-in-traits.html)
feature. Note that this feature [will soon be in stable Rust](https://github.com/rust-lang/rust/pull/115822).

## Project Layout

The project uses cargo's workspace feature, each day is a separate workspace member. `utils` contains various helpers
for things that come up a lot in AoC, including some basic math stuff (2D vector, box and grid), algorithms (bfs,
dijkstra, ..) and helpers for parsing strings into data via regular expressions.

`aoc_derive` implements the `#[aoc_main]` proc_macro that implements the `main()` function for each day.

There's also an `init_day.sh` script that will download my input into a file, create the project for the day and will
open the `main.rs` in neovim with some AoC-specific key-bindings and window layout (see `aoc.lua`).
