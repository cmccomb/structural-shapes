[![Build Status](https://travis-ci.com/cmccomb/structural-shapes.svg?branch=main)](https://travis-ci.com/cmccomb/structural-shapes)
[![Crates.io](https://img.shields.io/crates/v/structural-shapes.svg)](https://crates.io/crates/structural-shapes)
[![docs.rs](https://docs.rs/structural-shapes/badge.svg)](https://docs.rs/structural-shapes)
# About
This package provides utilities for a variety of different structural shapes. Currently, the following are included:
- Rods
- Rectangular bars
- Pipes
- Box Beams
- I-Beams
- Composite Shapes

# Usage
Here are some basic examples of usage

```rust
 fn main() {
    let x = structural_shapes::Rod{radius: 1.0, center_of_gravity: (0.0, 1.0)};
    println!("cross sectional area: {:?}", x.area());
    println!("moment of inertia: {:?}", x.moi_x());
 }
```
You can also create composite shapes that are composed of more than one primitive:
```rust
fn main() {
    let mut x = CompositeShape::new()
        .add(StructuralShape::Rod {
          radius: 2.0,
            center_of_gravity: (2.0, 0.0),
        })
        .add(StructuralShape::Rod {
            radius: 2.0,
            center_of_gravity: (-2.0, 0.0),
        });
    println!("cross sectional area: {:?}", x.area());
    println!("moment of inertia: {:?}", x.moi_x());
}
```
