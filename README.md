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
use structural_shapes::{StructuralShape, length, point};
let x = StructuralShape::Rod{
    radius: length(1.0), 
    center_of_gravity: point(0.0, 1.0)
};
println!("cross sectional area: {:?}", x.area().value);
println!("moment of inertia: {:?}", x.moi_x().value);
```

You can also create composite shapes that are composed of more than one primitive:
```rust
use structural_shapes::{CompositeShape, StructuralShape, point, length};
let mut x = CompositeShape::new()
    .add(StructuralShape::Rod {
        radius: length(2.0),
        center_of_gravity: point(2.0, 0.0),
    })
    .add(StructuralShape::Rod {
        radius: length(2.0),
        center_of_gravity: point(-2.0, 0.0),
    });
println!("cross sectional area: {:?}", x.area().value);
println!("moment of inertia: {:?}", x.moi_x().value);
```
