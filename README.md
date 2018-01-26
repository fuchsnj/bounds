# Bounds
[![Build Status](https://travis-ci.org/fuchsnj/bounds.svg?branch=master)](https://travis-ci.org/fuchsnj/bounds)
[![crates.io](https://img.shields.io/crates/v/bounds.svg)](https://crates.io/crates/bounds)
[documentation](https://docs.rs/bounds)

A library to interact with bounded and unbounded ranges

## Example

```rust
use bounds::*;

assert!(Bounds::from(2..4).intersects(&Bounds::from(1..3)));
```