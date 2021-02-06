[![Build Status](https://travis-ci.com/cmccomb/structural-shapes.svg?branch=main)](https://travis-ci.com/cmccomb/structural-shapes)
[![Crates.io](https://img.shields.io/crates/v/structural-shapes.svg)](https://crates.io/crates/structural-shapes)
[![docs.rs](https://docs.rs/structural-shapes/badge.svg)](https://docs.rs/structural-shapes)
# About
This package provides utilities for a variety of different structural shapes. Currently, the following are included:
- Pipes
- Box Beams
- I-Beams
- Rods

# Usage
Here are some basic examples of usage

```rust
 fn main() {
    let x = structural_shapes::Rod{radius: 1.0};
    println!("cross sectional area: {:?}", x.area());
    println!("moment of inertia: {:?}", x.moi_x());
    println!("moment of inertia with displacement 2.0: {:?}", x.moi_x_d());
 }
```
