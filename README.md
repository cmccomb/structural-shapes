[![build](https://github.com/cmccomb/structural-shapes/actions/workflows/tests.yml/badge.svg)](https://github.com/cmccomb/structural-shapes/actions/workflows/tests.yml)
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
use structural_shapes::StructuralShape;
let x = StructuralShape::Rod{radius: 1.0, center_of_gravity: (0.0, 1.0)};
println!("cross sectional area: {:?}", x.area());
println!("moment of inertia: {:?}", x.moi_x());
```

You can also create composite shapes that are composed of more than one primitive:
```rust
use structural_shapes::{CompositeShape, StructuralShape};
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
```
