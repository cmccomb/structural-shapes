//! Standard sections from the AISC Shapes Database v16.0 (optional `aisc` feature).
//!
//! Catalog properties are published, centroidal values. They are independent of
//! the sharp-corner geometry returned by [`AiscSection::idealized_shape`].
//! Units are represented by `uom` quantities and stored in SI.
//!
//! ```
//! use structural_shapes::{AiscSection, SectionFamily};
//! let beam = AiscSection::W12X26;
//! assert_eq!(" w12x26 ".parse::<AiscSection>()?, beam);
//! assert_eq!(beam.family(), SectionFamily::WideFlange);
//! assert!(beam.moi_x() > beam.moi_y());
//! # Ok::<(), structural_shapes::ParseAiscSectionError>(())
//! ```

use crate::{SecondAreaMomentofInertia, StructuralShape};
use std::{error::Error, fmt, str::FromStr};
use uom::si::{
    area::square_inch,
    f64::{Area, Length, LinearMassDensity},
    length::inch,
    linear_mass_density::kilogram_per_meter,
};

/// Source edition for every catalog entry.
pub const DATABASE_VERSION: &str = "AISC Shapes Database v16.0 (August 2023)";
/// Pinned upstream workbook; checksum and extraction instructions are in data/README.md.
pub const SOURCE_URL: &str = "https://cloud.aisc.org/biggie_bin/aisc-shapes-database-v160-2.xlsx";

/// Cross-section family, independent of material grade.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SectionFamily {
    /// W section with parallel flanges.
    WideFlange,
    /// Square hollow structural section.
    SquareHollow,
    /// Rectangular hollow structural section.
    RectangularHollow,
    /// Round hollow structural section.
    RoundHollow,
}

/// Published dimensions; nominal and design HSS wall thickness are distinct.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum SectionDimensions {
    /// W section dimensions, excluding fillet geometry.
    WideFlange {
        /// Overall depth (`d`).
        depth: Length,
        /// Flange width (`bf`).
        flange_width: Length,
        /// Web thickness (`tw`).
        web_thickness: Length,
        /// Flange thickness (`tf`).
        flange_thickness: Length,
    },
    /// Square or rectangular HSS dimensions, excluding corner radii.
    HollowRectangle {
        /// Overall depth (`Ht`), oriented vertically.
        height: Length,
        /// Overall width (`B`), oriented horizontally.
        width: Length,
        /// Nominal wall thickness (`tnom`).
        nominal_thickness: Length,
        /// Design wall thickness (`tdes`), used by the idealized shape.
        design_thickness: Length,
    },
    /// Round HSS dimensions.
    HollowRound {
        /// Tabulated outside diameter (`OD`).
        outer_diameter: Length,
        /// Nominal wall thickness (`tnom`).
        nominal_thickness: Length,
        /// Design wall thickness (`tdes`), used by the idealized shape.
        design_thickness: Length,
    },
}

/// Published section properties converted from US customary columns to SI.
///
/// Returned by value; editing custom geometry never changes these catalog values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SectionProperties {
    /// Cross-sectional area.
    area: Area,
    /// Centroidal x-axis second moment of area.
    moi_x: SecondAreaMomentofInertia,
    /// Centroidal y-axis second moment of area.
    moi_y: SecondAreaMomentofInertia,
    /// Saint-Venant torsional constant, distinct from the polar second moment.
    torsional_constant: SecondAreaMomentofInertia,
    /// Nominal mass per unit length.
    mass_per_length: LinearMassDensity,
}

impl SectionProperties {
    /// Cross-sectional area (`A`).
    pub fn area(&self) -> Area {
        self.area
    }
    /// Centroidal second moment about the horizontal x-axis (`Ix`).
    pub fn moi_x(&self) -> SecondAreaMomentofInertia {
        self.moi_x
    }
    /// Centroidal second moment about the vertical y-axis (`Iy`).
    pub fn moi_y(&self) -> SecondAreaMomentofInertia {
        self.moi_y
    }
    /// Polar second moment (`Ix + Iy`); generally not the torsional constant `J`.
    pub fn polar_moi(&self) -> SecondAreaMomentofInertia {
        self.moi_x + self.moi_y
    }
    /// Published Saint-Venant torsional constant (`J`).
    pub fn torsional_constant(&self) -> SecondAreaMomentofInertia {
        self.torsional_constant
    }
    /// Nominal weight-per-length column (`W`), expressed as mass per length.
    pub fn mass_per_length(&self) -> LinearMassDensity {
        self.mass_per_length
    }
}

/// Unconverted source record, instantiated by the generated catalog.
struct CatalogEntry {
    /// AISC Manual designation.
    designation: &'static str,
    /// EDI designation, accepted as an alternate spelling.
    edi_designation: &'static str,
    /// Family determining the interpretation of geometry.
    family: SectionFamily,
    /// Four dimensions in inches; ordering is documented in generated.rs.
    geometry: [f64; 4],
    /// W (lb/ft), A (in²), Ix, Iy, J (in⁴), in that order.
    properties: [f64; 5],
}

/// Keep enum identity, enumeration, and record lookup in one generated list.
macro_rules! catalog {
    ($($id:ident => ($label:literal, $edi:literal, $family:ident, $geometry:expr, $properties:expr);)*) => {
        /// A standard W or HSS section from AISC v16.0.
        ///
        /// Variant names follow the US Manual label with `.`, `/`, and `-`
        /// replaced by `_`, e.g. `HSS6X6X1_4` means `HSS6X6X1/4`.
        /// Use `Display`/`FromStr` for persistent names, not integer discriminants.
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
        #[non_exhaustive]
        pub enum AiscSection {
            $(#[doc = $label] $id,)*
        }

        impl AiscSection {
            /// All supported sections in source order; ordering is not an API guarantee.
            pub fn all() -> &'static [Self] { &[$(Self::$id,)*] }

            /// Fetch the source record without parsing or allocation.
            // Values such as 3.14 are published measurements, not approximations of pi.
            #[allow(clippy::approx_constant)]
            fn entry(self) -> CatalogEntry {
                match self {
                    $(Self::$id => CatalogEntry {
                        designation: $label, edi_designation: $edi,
                        family: SectionFamily::$family,
                        geometry: $geometry, properties: $properties,
                    },)*
                }
            }
        }
    };
}

include!("generated.rs");

impl AiscSection {
    /// Iterate over supported sections, suitable for filtering and selection.
    pub fn iter() -> impl ExactSizeIterator<Item = Self> + DoubleEndedIterator {
        Self::all().iter().copied()
    }

    /// US customary AISC Manual label, also used by `Display`.
    pub fn designation(self) -> &'static str {
        self.entry().designation
    }
    /// US customary EDI label, also accepted by `FromStr`.
    pub fn edi_designation(self) -> &'static str {
        self.entry().edi_designation
    }
    /// Section family, independent of material grade.
    pub fn family(self) -> SectionFamily {
        self.entry().family
    }

    /// Published dimensions converted to unit-safe lengths.
    pub fn dimensions(self) -> SectionDimensions {
        let entry = self.entry();
        let [a, b, c, d] = entry.geometry.map(Length::new::<inch>);
        match entry.family {
            SectionFamily::WideFlange => SectionDimensions::WideFlange {
                depth: a,
                flange_width: b,
                web_thickness: c,
                flange_thickness: d,
            },
            SectionFamily::SquareHollow | SectionFamily::RectangularHollow => {
                SectionDimensions::HollowRectangle {
                    height: a,
                    width: b,
                    nominal_thickness: c,
                    design_thickness: d,
                }
            }
            SectionFamily::RoundHollow => SectionDimensions::HollowRound {
                outer_diameter: a,
                nominal_thickness: c,
                design_thickness: d,
            },
        }
    }

    /// Published centroidal properties, independent of idealized geometry.
    pub fn properties(self) -> SectionProperties {
        let [weight, area, ix, iy, j] = self.entry().properties;
        let one_inch = Length::new::<inch>(1.0);
        let fourth_power: SecondAreaMomentofInertia = one_inch * one_inch * one_inch * one_inch;
        SectionProperties {
            area: Area::new::<square_inch>(area),
            moi_x: fourth_power * ix,
            moi_y: fourth_power * iy,
            torsional_constant: fourth_power * j,
            // Exact international pound and foot conversions; retain source precision.
            mass_per_length: LinearMassDensity::new::<kilogram_per_meter>(
                weight * 0.453_592_37 / 0.3048,
            ),
        }
    }

    /// Published cross-sectional area (`A`).
    pub fn area(self) -> Area {
        self.properties().area()
    }
    /// Published centroidal horizontal-axis second moment (`Ix`).
    pub fn moi_x(self) -> SecondAreaMomentofInertia {
        self.properties().moi_x()
    }
    /// Published centroidal vertical-axis second moment (`Iy`).
    pub fn moi_y(self) -> SecondAreaMomentofInertia {
        self.properties().moi_y()
    }
    /// Polar second moment (`Ix + Iy`), not the torsional constant `J`.
    pub fn polar_moi(self) -> SecondAreaMomentofInertia {
        self.properties().polar_moi()
    }
    /// Published Saint-Venant torsional constant (`J`).
    pub fn torsional_constant(self) -> SecondAreaMomentofInertia {
        self.properties().torsional_constant()
    }
    /// Nominal mass per unit length.
    pub fn mass_per_length(self) -> LinearMassDensity {
        self.properties().mass_per_length()
    }

    /// Approximate geometry centered at the origin, using HSS design thickness.
    ///
    /// W fillets and rectangular HSS corner radii are omitted. Source rounding
    /// also affects round HSS. Calculated properties need not equal the catalog.
    /// This explicit conversion discards the catalog identity and tabulated values.
    pub fn idealized_shape(self) -> StructuralShape {
        match self.dimensions() {
            SectionDimensions::WideFlange {
                depth,
                flange_width,
                web_thickness,
                flange_thickness,
            } => StructuralShape::new_ibeam(
                depth.value,
                flange_width.value,
                web_thickness.value,
                flange_thickness.value,
            ),
            SectionDimensions::HollowRectangle {
                height,
                width,
                design_thickness,
                ..
            } => StructuralShape::new_boxbeam(height.value, width.value, design_thickness.value),
            SectionDimensions::HollowRound {
                outer_diameter,
                design_thickness,
                ..
            } => StructuralShape::new_pipe(outer_diameter.value / 2.0, design_thickness.value),
        }
    }
}

impl fmt::Display for AiscSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.designation())
    }
}

/// A name that is not in the supported W/HSS catalog.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseAiscSectionError {
    /// Original, unmodified input to aid diagnostics.
    input: String,
}

impl ParseAiscSectionError {
    /// The original input that could not be resolved.
    pub fn input(&self) -> &str {
        &self.input
    }
}

impl fmt::Display for ParseAiscSectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown AISC W/HSS section: {:?}", self.input)
    }
}

impl Error for ParseAiscSectionError {}

impl FromStr for AiscSection {
    type Err = ParseAiscSectionError;

    /// Accept US Manual or EDI labels, ignoring ASCII case and outer whitespace.
    /// Other aliases, metric designations, and unsupported families are rejected.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let name = input.trim();
        Self::iter()
            .find(|section| {
                let entry = section.entry();
                name.eq_ignore_ascii_case(entry.designation)
                    || name.eq_ignore_ascii_case(entry.edi_designation)
            })
            .ok_or_else(|| ParseAiscSectionError {
                input: input.to_owned(),
            })
    }
}
