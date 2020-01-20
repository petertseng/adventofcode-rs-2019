# Advent of Code

These are my solutions to http://adventofcode.com

All solutions are written in Rust.

[![Build Status](https://travis-ci.org/petertseng/adventofcode-rs-2019.svg?branch=master)](https://travis-ci.org/petertseng/adventofcode-rs-2019)

## Input

In general, all solutions can be invoked in both of the following ways:

* Without command-line arguments, takes input on standard input.
* With 1+ command-line arguments, reads input from the first, which must be the path to an input file.
  Arguments beyond the first are ignored.

Some may additionally support other ways:

* All intcode days: May pass the intcode in ARGV as a single argument separated by commas.
* Day 04 (Password): May pass min and max in ARGV (as two args, or as one arg joined by a hyphen).

## Closing Thoughts

Sometimes `cargo fmt` does something I don't like, such as:

```rust
let foo = bar
    .iter()
    .map(|v| {
        // ...
        // ...
    })
    .collect();
```

So I restructure my code to avoid that, using one of two ways:

First possibility is to move the closure to its own line,
which works if the `collect` line ends up being short enough.

```rust
let f = |v: T| {
    // ...
    // ...
};
let foo = bar.iter().map(f).collect();
```

Second possibility is to make the `map` the last thing by delaying the `collect`.

```rust
let foo = bar.iter().map(|v| {
    // ...
    // ...
});
let foo = foo.collect();
```
