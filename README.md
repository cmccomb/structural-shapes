[![Build Status](https://travis-ci.com/cmccomb/trussx.svg?branch=master)](https://travis-ci.com/cmccomb/structural-shapes)
[![Crates.io](https://img.shields.io/crates/v/trussx.svg)](https://crates.io/crates/structural-shapes)
[![docs.rs](https://docs.rs/trussx/badge.svg)](https://docs.rs/structural-shapes)
# About
This package provides utilities for a variety of different structural shapes. Currently, the following are included:
- Pipes
- Box Beams
- I-Beams
- Rods

# Installation
Install through ``crates.io`` with:
```shell script
cargo install structural_shapes
```

Then add it to your `Cargo.toml` with:
```toml
[dependencies]
strutural_shapes = "0.1.0"
```
and add this to your root:
```rust
use structural_shapes;
```
# Usage
Here are some basic examples of usage

```rust
use structural_shapes;
fn main() {
    let x = structural_shapes::Rod{outer_radius: 1.0};
    println!("cross sectional area: {:?}", x.area());
    println!("moment of inertia: {:?}", x.moment_of_inertia());
}
```
