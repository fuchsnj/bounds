# Bounds

A library to interact with bounded and unbounded ranges

## Example

```rust
use bounds::*;

assert!(Bounds::from(2..4).intersects(&Bounds::from(1..3)));
```