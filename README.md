[![build](https://github.com/cmccomb/structural-shapes/actions/workflows/tests.yml/badge.svg)](https://github.com/cmccomb/structural-shapes/actions/workflows/tests.yml)
[![Crates.io](https://img.shields.io/crates/v/structural-shapes.svg)](https://crates.io/crates/structural-shapes)
[![docs.rs](https://docs.rs/structural-shapes/badge.svg)](https://docs.rs/structural-shapes)

# structural-shapes

Cross-section geometry and properties in Rust, with units provided by `uom`.
Supports rods, rectangular bars, pipes, box and I-beams, channels, tees, angles,
double angles, and composite shapes. An optional AISC catalog provides standard
steel sections.

## Usage

Constructors take dimensions in meters and assume valid geometry.

```rust
use structural_shapes::StructuralShape;

let beam = StructuralShape::new_ibeam(0.30, 0.15, 0.008, 0.012);
println!("Area: {} m²", beam.area().value);
println!("Ix: {} m⁴", beam.moi_x().value);
```

Use `CompositeShape` to add or subtract shapes. Custom `moi_x()` and `moi_y()`
measure about the origin; `centroidal_area_moments()` gives moments about the
centroid, including `Ixy`, with rotation and principal-axis methods.
**`polar_moi()` is `Ix + Iy`, generally not the torsional constant `J`.**
Use `torsional_constant()` for supported torsion calculations.

## Standard steel sections

Enable `aisc` for 2,299 sections from the AISC Shapes Database v16.0, including
beams, channels, angles, tees, double angles, HSS, and pipe:

```toml
[dependencies]
structural-shapes = { version = "0.3", features = ["aisc"] }
```

```rust
use structural_shapes::AiscSection;

let beam = AiscSection::W12X26;
let sx = beam.elastic_section_modulus_x();
let j = beam.torsional_constant();
```

Catalog properties include centroidal moments, section moduli, radii of gyration,
and torsion/warping properties. Missing values return `None`.
`idealized_shape()` supplies approximate geometry; its calculated properties can
differ from the published catalog.

See the [API documentation](https://docs.rs/structural-shapes),
[examples](https://github.com/cmccomb/structural-shapes/tree/main/examples), and
[catalog source notes](https://github.com/cmccomb/structural-shapes/blob/main/data/README.md) for details.
For API documentation matching this checkout, run `cargo doc --open --all-features`.
