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

Enable the optional `aisc` feature for a typed catalog of 1,003 W and HSS sections
from the AISC Shapes Database v16.0. No runtime downloads or additional Rust
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

for section in AiscSection::iter().filter(|s| s.moi_x() >= ix) {
    println!("{}: {} m²", section, section.area().value);
}
# Ok(())
# }
# #[cfg(not(feature = "aisc"))]
# fn main() {}
```

Enum names replace `.`, `/`, and `-` in the US Manual label with `_`.
`FromStr` accepts the US Manual and EDI labels, ignoring ASCII case and outer
whitespace. Unknown names, metric designations, and unsupported families return
`ParseAiscSectionError`. Use `Display` names for persistence; enum discriminants
and iteration order are not stable identifiers. The public enums are
`#[non_exhaustive]` so later catalogs can add sections and families.

`properties()` returns published area, centroidal inertias, torsional constant,
and mass per length as unit-safe quantities. `dimensions()` distinguishes nominal
and design HSS wall thickness. These properties remain separate from
`idealized_shape()`, which creates a centered `StructuralShape` using design
thickness and omitting fillets and rectangular HSS corner radii. Its calculated
properties can differ from tabulated values. `polar_moi()` is `Ix + Iy`, not the
Saint-Venant torsional constant returned by `torsional_constant()`.

Run `cargo run --example select_section --features aisc` for a lightest-section
search constrained by inertia and depth. This is geometric selection, not a
strength or code-compliance check. See [catalog provenance and regeneration](data/README.md).

# Axis convention

For custom geometry, x is horizontal and y is vertical. `moi_x()` and `moi_y()`
are about the axes through the origin, including the parallel-axis terms
`A * y²` and `A * x²`, respectively. Call `CompositeShape::update_cog()` to
center a composite before requesting centroidal values. Catalog properties
are always about their centroidal axes.

This change corrects the prior I-beam x-axis dispatch, rectangle y-axis formula,
and swapped translation terms. Results for asymmetric or translated custom
shapes therefore change.
