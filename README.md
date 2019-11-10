# Bounds
[![Build Status](https://travis-ci.org/fuchsnj/bounds.svg?branch=master)](https://travis-ci.org/fuchsnj/bounds)
[![crates.io](https://img.shields.io/crates/v/bounds.svg)](https://crates.io/crates/bounds)

[documentation](https://docs.rs/bounds)

A library to interact with bounded and unbounded ranges
Bounds contains a lower/upper bound which are one of [unbounded, inclusive, exclusive].
It also supports zero-sized bounds (an exact value).

You can use the four basic arithmetic operations on these ranges. (add, subtract, multiply, divide)

## Implementation Details
This library adheres to "real" math, and does not respect integer math, or floating point precision issues.
It was designed to be used with `BigRational`, but is generic so it can be used with others.
If this is used with types that can overflow, round, accumulate errors, etc. no guarantees are
made.

The lower bound MUST always be lower than the upper bound. There are occasional internal checks when
running in debug mode, but it's up to the caller to ensure all bounds are valid when they are created.

This library will never cause a divide by 0 (division returns a None in this case), and will not panic
if the input is valid and the generic type used also doesn't panic.

## Macro

Rust's built in bounds don't allow the lower bound to be exclusive, so a macro is provided for easier use.
The prefix `~` is used to make a bound exclusive. It is inclusive by default

```rust
bound!(,); // unbounded
bound!(~0, 3); // > 0 and <= 3
bound!(3,); // >= 3
bound!(-3); // exactly -3

```

## Example

```rust
use bounds::*;

assert!(bounds!(2, 4).intersects(&bounds!(1, 3)));
assert_eq!(
    bounds!(6) / bounds!(2,), 
    Some(bounds!(~0,3))
);
```