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

`polar_moi()` means the polar **second moment of area**, `Ix + Iy`, about the
origin. It is generally not the Saint-Venant torsional constant `J`. For circular
rods and concentric circular pipes, `J` equals the **centroidal** polar moment;
moving a shape changes its moments about the origin but does not change `J`.
`StructuralShape::torsional_constant()` returns the exact circular result for
rods/pipes and `None` for other custom shapes. No generic torsion formula is inferred.
The circular-shaft relation `tau = T*r/J` must not be applied to arbitrary sections.

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
let sx = beam.elastic_section_modulus_x();
let zx = beam.plastic_section_modulus_x();
let rx = beam.radius_of_gyration_x();
let cw = beam.warping_constant().unwrap();
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

`properties()` returns area, centroidal inertias, section moduli, radii of gyration,
torsion/warping properties, and mass per length as unit-safe quantities.
The getters are available on both `AiscSection` and `SectionProperties`:

| Source property | Getter | SI dimension |
| --- | --- | --- |
| Sx / Sy | `elastic_section_modulus_x()` / `elastic_section_modulus_y()` | m³ |
| Zx / Zy | `plastic_section_modulus_x()` / `plastic_section_modulus_y()` | m³ |
| rx / ry | `radius_of_gyration_x()` / `radius_of_gyration_y()` | m |
| J | `torsional_constant()` | m⁴, optional |
| Cw | `warping_constant()` | m⁶, optional |
| HSS C | `torsional_section_modulus()` | m³, optional |
| Channel eo | `shear_center_distance()` | m, optional |
| rts | `effective_radius_of_gyration()` | m, optional |
| ho | `flange_centroid_distance()` | m, optional |

Tabulated S, Z, and r are preserved rather than recomputed from rounded dimensions
or inertias. S is not generally I divided by half the overall depth for asymmetric
sections. Channel `eo` is measured from the AISC-designated edge, **not** from the
centroid. The angle principal-axis property `Iw` (m⁴) is distinct from `Cw` (m⁶).

`dimensions()` distinguishes nominal
and design HSS/pipe wall thickness. `torsional_constant()` returns `Some(J)` where
tabulated, and `None` for all double angles; missing values are never replaced
with zero or an inferred value. Other optional properties similarly retain missing
data as `None`. `polar_moi()` is `Ix + Iy`, generally not `J`.

`product_moi()` uses `Ixy = integral(x*y dA)` about the centroid. It is zero by
symmetry except for single angles. For those, it is **derived** from the rounded
published `Iw`, `Iz`, and `tan(alpha)` values, with the vertical leg on the left
and bottom leg pointing right (negative Ixy). Reflecting that orientation reverses
the sign. `area_moments()` combines it with tabulated Ix/Iy; principal moments
computed from this tensor can differ slightly from published Iw/Iz through rounding.

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
`A * y²` and `A * x²`, respectively. `product_moi()` includes `A * x * y`.
`area_moments()` groups these three values. `centroidal_area_moments()` provides
centroidal values without moving the shape or composite. The composite version
returns `None` when its centroid is undefined; `try_calculate_cog()` also exposes
that check directly. Signed composite sums do not perform Boolean geometry:
overlapping additions double-count area, and subtraction does not clip shapes.
Catalog properties are always about their centroidal axes.

```rust
use structural_shapes::StructuralShape;

let angle = StructuralShape::new_angle(0.08, 0.04, 0.005).with_cog(2.0, -1.0);
let moments = angle.centroidal_area_moments();
assert!(moments.product_moi().value < 0.0);
let (major, minor) = moments.principal_moments();
let rotation = moments.principal_axis_angle().unwrap();
let principal = moments.rotated(rotation);
assert!(principal.product_moi().value.abs() < 1e-18);
let (rx, ry) = angle.radii_of_gyration(); // Centroidal and independent of position.
```

Axis rotations are counterclockwise with the shape fixed. Principal moments are
returned as `(major, minor)`; the principal-axis angle points to the major-moment
axis and is `None` when all axes have the same moment (e.g. a centered circle).
Custom-geometry methods continue to assume valid dimensions. The checked
composite centroid validates its resulting area/coordinates, not shape topology.

This change corrects the prior I-beam x-axis dispatch, rectangle y-axis formula,
and swapped translation terms. Results for asymmetric or translated custom
shapes therefore change.
