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
- Channels, tees, angles, and back-to-back double angles
- Composite Shapes

# Usage
Here are some basic examples of usage

```rust
use structural_shapes::StructuralShape;
let x = StructuralShape::new_rod(1.0).with_cog(0.0, 1.0);
println!("cross sectional area: {:?}", x.area().value);
println!("area moment of inertia: {:?}", x.moi_x().value);
println!("polar moment of inertia: {:?}", x.polar_moi().value);
```

You can also create composite shapes that are composed of more than one primitive:
```rust
use structural_shapes::{CompositeShape, StructuralShape};
let x = CompositeShape::new()
    .add(StructuralShape::new_rod(2.0).with_cog(2.0, 0.0))
    .add(StructuralShape::new_rod(2.0).with_cog(-2.0, 0.0));
println!("cross sectional area: {:?}", x.area().value);
println!("area moment of inertia: {:?}", x.moi_x().value);
println!("polar moment of inertia: {:?}", x.polar_moi().value);
```

# Standard steel sections

Enable the optional `aisc` feature for all 2,299 sections
from the AISC Shapes Database v16.0: W, M, S, HP, C, MC, L, WT, MT, ST, 2L,
HSS (square, rectangular, and round), and PIPE. No runtime downloads or additional Rust
dependencies are required. The default feature set remains empty.

```toml
[dependencies]
structural-shapes = { git = "https://github.com/cmccomb/structural-shapes", features = ["aisc"] }
```

The catalog is not included in the published 0.2.3 release.

```rust
# #[cfg(feature = "aisc")]
# fn main() -> Result<(), structural_shapes::ParseAiscSectionError> {
use structural_shapes::{AiscSection, SectionFamily};

let beam = AiscSection::W12X26;
let area = beam.area();
let ix = beam.moi_x();
let dimensions = beam.dimensions();
assert_eq!(beam.family(), SectionFamily::WideFlange);
assert_eq!("w12x26".parse::<AiscSection>()?, beam);

let tube = AiscSection::HSS6X6X1_4;
assert_eq!(tube.to_string(), "HSS6X6X1/4");
assert_eq!("HSS6X6X.250".parse::<AiscSection>()?, tube);

let channel = AiscSection::C12X20_7;
let angle = AiscSection::L8X4X1_2;
let tee = AiscSection::WT6X13;
let pipe = AiscSection::Pipe2STD;
let pair = AiscSection::DoubleAngle8X4X1_2X3_8LLBB;
assert_eq!(pair.to_string(), "2L8X4X1/2X3/8LLBB");
assert_eq!(pair.torsional_constant(), None); // Not tabulated by AISC.

for section in AiscSection::iter().filter(|s| s.moi_x() >= ix) {
    println!("{}: {} m²", section, section.area().value);
}
# Ok(())
# }
# #[cfg(not(feature = "aisc"))]
# fn main() {}
```

Enum names replace `.`, `/`, and `-` in the US Manual label with `_`.
The leading `2L` becomes `DoubleAngle` to form a valid Rust identifier. The
double-angle designation retains its gap and LLBB/SLBB orientation; omitting the
gap means the backs touch. `Pipe` names retain the Manual's capitalization.
`FromStr` accepts the US Manual and EDI labels, ignoring ASCII case and outer
whitespace. This includes pipe schedule aliases such as `Pipe24SCH20` for
`Pipe24STD`. Unknown names and metric designations return
`ParseAiscSectionError`. Use `Display` names for persistence; enum discriminants
and iteration order are not stable identifiers. The public enums are
`#[non_exhaustive]` so later catalogs can add sections and families.

`properties()` returns published area, centroidal inertias, optional torsional constant,
and mass per length as unit-safe quantities. `dimensions()` distinguishes nominal
and design HSS/pipe wall thickness. `torsional_constant()` returns `Some(J)` where
tabulated, and `None` for all double angles; missing values are never replaced
with zero or an inferred value. `polar_moi()` is `Ix + Iy`, not `J`.

Published properties remain separate from `idealized_shape()`, which creates a
`StructuralShape` centered at its own geometric centroid. It uses HSS/pipe design
thickness and omits fillets, toe/corner radii, and flange taper. Its calculated
properties and centroid locations can differ from tabulated values. Single
angles have the long leg vertical; double angles honor their gap and LLBB/SLBB
orientation. Angle moments use geometric x/y axes, not principal axes.

Run `cargo run --example select_section --features aisc` for a lightest-section
search constrained by inertia and depth. This is geometric selection, not a
strength or code-compliance check. See [catalog provenance and regeneration](data/README.md).

# Custom open sections

The new parameterized shapes are also available without the catalog feature.
As with the existing constructors, dimensions are in meters. The constructors
assume valid dimensions; they do not perform input validation.

```rust
use structural_shapes::StructuralShape;

let channel = StructuralShape::new_channel(0.30, 0.10, 0.008, 0.012);
let tee = StructuralShape::new_tee(0.15, 0.10, 0.008, 0.012);
let angle = StructuralShape::new_angle(0.08, 0.04, 0.005);
let pair = StructuralShape::new_double_angle(0.08, 0.04, 0.005, 0.01);
assert!(angle.moi_x() > angle.moi_y());
```

These are sharp-corner sections with uniform thickness. The channel web is on
the left, tee flange at the top, and angle heel at the bottom left. Double angles
have vertical legs back-to-back and bottom legs pointing outward; `width` is the
width of each angle and `gap` is the clear distance between their backs. All are
centered at their geometric centroid and support `with_cog()` and composition.

# Axis convention

For custom geometry, x is horizontal and y is vertical. `moi_x()` and `moi_y()`
are about the axes through the origin, including the parallel-axis terms
`A * y²` and `A * x²`, respectively. Call `CompositeShape::update_cog()` to
center a composite before requesting centroidal values. Catalog properties
are always about their centroidal axes.

This change corrects the prior I-beam x-axis dispatch, rectangle y-axis formula,
and swapped translation terms. Results for asymmetric or translated custom
shapes therefore change.
