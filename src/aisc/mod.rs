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
    /// M section (miscellaneous beam).
    MiscellaneousBeam,
    /// S section (American Standard beam).
    AmericanStandardBeam,
    /// HP bearing pile.
    BearingPile,
    /// C section (American Standard channel).
    AmericanStandardChannel,
    /// MC section (miscellaneous channel).
    MiscellaneousChannel,
    /// Single L section.
    Angle,
    /// WT tee cut from a W section.
    WideFlangeTee,
    /// MT tee cut from an M section.
    MiscellaneousTee,
    /// ST tee cut from an S section.
    AmericanStandardTee,
    /// Two back-to-back angles, including the catalog's spacing/orientation variants.
    DoubleAngle,
    /// Standard, extra-strong, or double-extra-strong pipe.
    Pipe,
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
    /// M, S, or HP beam dimensions; idealization treats flanges as uniform thickness.
    IBeam {
        /// Overall depth (`d`).
        depth: Length,
        /// Flange width (`bf`).
        flange_width: Length,
        /// Web thickness (`tw`).
        web_thickness: Length,
        /// Tabulated flange thickness (`tf`); taper is not represented.
        flange_thickness: Length,
    },
    /// C or MC channel dimensions; idealization omits flange taper.
    Channel {
        /// Overall vertical depth (`d`).
        depth: Length,
        /// Overall horizontal flange width (`bf`).
        flange_width: Length,
        /// Web thickness (`tw`).
        web_thickness: Length,
        /// Tabulated flange thickness (`tf`).
        flange_thickness: Length,
    },
    /// WT, MT, or ST tee dimensions; idealization omits flange taper.
    Tee {
        /// Overall depth, including the flange (`d`).
        depth: Length,
        /// Flange width (`bf`).
        flange_width: Length,
        /// Stem thickness (`tw`).
        web_thickness: Length,
        /// Tabulated flange thickness (`tf`).
        flange_thickness: Length,
    },
    /// Single-angle dimensions in the AISC orientation (long leg vertical).
    Angle {
        /// Overall vertical leg dimension (`b`).
        vertical_leg: Length,
        /// Overall horizontal leg dimension (`d`).
        horizontal_leg: Length,
        /// Leg thickness (`t`).
        thickness: Length,
    },
    /// Double-angle dimensions in the designation's back-to-back arrangement.
    DoubleAngle {
        /// Vertical leg dimension, matching LLBB/SLBB orientation (`d`).
        back_to_back_leg: Length,
        /// Horizontal dimension of each outward leg (`b`).
        outstanding_leg: Length,
        /// Leg thickness (`t`).
        thickness: Length,
        /// Clear spacing between the backs, decoded from the designation; zero if omitted.
        gap: Length,
    },
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
    /// Round HSS or pipe dimensions.
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
    torsional_constant: Option<SecondAreaMomentofInertia>,
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
    /// Published Saint-Venant torsional constant (`J`), or `None` for double angles.
    /// The source does not tabulate double-angle `J`; it is not inferred or set to zero.
    pub fn torsional_constant(&self) -> Option<SecondAreaMomentofInertia> {
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
    /// Unconverted geometry in inches.
    geometry: RawDimensions,
    /// W (lb/ft), A (in²), Ix and Iy (in⁴), in that order.
    properties: [f64; 4],
    /// Tabulated J in in⁴; missing for double angles.
    torsional_constant: Option<f64>,
}

/// Geometry layouts used by the generated source records; all values are inches.
enum RawDimensions {
    /// Depth, flange width, web thickness, flange thickness.
    IBeam([f64; 4]),
    /// Depth, flange width, web thickness, flange thickness.
    Channel([f64; 4]),
    /// Depth, flange width, stem thickness, flange thickness.
    Tee([f64; 4]),
    /// Vertical leg, horizontal leg, thickness.
    Angle([f64; 3]),
    /// Back-to-back leg, outstanding leg, thickness, clear gap.
    DoubleAngle([f64; 4]),
    /// Height, width, nominal thickness, design thickness.
    HollowRectangle([f64; 4]),
    /// Outside diameter, nominal thickness, design thickness.
    HollowRound([f64; 3]),
}

/// Keep enum identity, enumeration, and record lookup in one generated list.
macro_rules! catalog {
    ($($id:ident => ($label:literal, $edi:literal, $family:ident, $geometry:expr, $properties:expr, $torsion:expr);)*) => {
        /// A standard section from the complete AISC v16.0 catalog.
        ///
        /// Variant names follow the US Manual label with `.`, `/`, and `-`
        /// replaced by `_`, e.g. `HSS6X6X1_4` means `HSS6X6X1/4`.
        /// The leading `2L` is spelled `DoubleAngle`, e.g. `DoubleAngle8X4X1_2LLBB`.
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
                        geometry: $geometry, properties: $properties, torsional_constant: $torsion,
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
        match entry.geometry {
            RawDimensions::IBeam(values) => {
                let [depth, flange_width, web_thickness, flange_thickness] =
                    values.map(Length::new::<inch>);
                if entry.family == SectionFamily::WideFlange {
                    SectionDimensions::WideFlange {
                        depth,
                        flange_width,
                        web_thickness,
                        flange_thickness,
                    }
                } else {
                    SectionDimensions::IBeam {
                        depth,
                        flange_width,
                        web_thickness,
                        flange_thickness,
                    }
                }
            }
            RawDimensions::Channel(values) => {
                let [depth, flange_width, web_thickness, flange_thickness] =
                    values.map(Length::new::<inch>);
                SectionDimensions::Channel {
                    depth,
                    flange_width,
                    web_thickness,
                    flange_thickness,
                }
            }
            RawDimensions::Tee(values) => {
                let [depth, flange_width, web_thickness, flange_thickness] =
                    values.map(Length::new::<inch>);
                SectionDimensions::Tee {
                    depth,
                    flange_width,
                    web_thickness,
                    flange_thickness,
                }
            }
            RawDimensions::Angle(values) => {
                let [vertical_leg, horizontal_leg, thickness] = values.map(Length::new::<inch>);
                SectionDimensions::Angle {
                    vertical_leg,
                    horizontal_leg,
                    thickness,
                }
            }
            RawDimensions::DoubleAngle(values) => {
                let [back_to_back_leg, outstanding_leg, thickness, gap] =
                    values.map(Length::new::<inch>);
                SectionDimensions::DoubleAngle {
                    back_to_back_leg,
                    outstanding_leg,
                    thickness,
                    gap,
                }
            }
            RawDimensions::HollowRectangle(values) => {
                let [height, width, nominal_thickness, design_thickness] =
                    values.map(Length::new::<inch>);
                SectionDimensions::HollowRectangle {
                    height,
                    width,
                    nominal_thickness,
                    design_thickness,
                }
            }
            RawDimensions::HollowRound(values) => {
                let [outer_diameter, nominal_thickness, design_thickness] =
                    values.map(Length::new::<inch>);
                SectionDimensions::HollowRound {
                    outer_diameter,
                    nominal_thickness,
                    design_thickness,
                }
            }
        }
    }

    /// Published centroidal properties, independent of idealized geometry.
    pub fn properties(self) -> SectionProperties {
        let entry = self.entry();
        let [weight, area, ix, iy] = entry.properties;
        let one_inch = Length::new::<inch>(1.0);
        let fourth_power: SecondAreaMomentofInertia = one_inch * one_inch * one_inch * one_inch;
        SectionProperties {
            area: Area::new::<square_inch>(area),
            moi_x: fourth_power * ix,
            moi_y: fourth_power * iy,
            torsional_constant: entry.torsional_constant.map(|j| fourth_power * j),
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
    /// Published Saint-Venant torsional constant (`J`), or `None` for double angles.
    pub fn torsional_constant(self) -> Option<SecondAreaMomentofInertia> {
        self.properties().torsional_constant()
    }
    /// Nominal mass per unit length.
    pub fn mass_per_length(self) -> LinearMassDensity {
        self.properties().mass_per_length()
    }

    /// Approximate geometry centered at its geometric centroid, using HSS/pipe design thickness.
    ///
    /// Fillets, toe radii, rectangular HSS corner radii, and flange taper are
    /// omitted. Source rounding also affects the geometry. Calculated properties
    /// and centroid locations need not equal the catalog. Single angles have
    /// their longer leg vertical; double angles honor their designation's gap
    /// and back-to-back orientation. Angles use geometric, not principal, axes.
    /// This explicit conversion discards the catalog identity and tabulated values.
    pub fn idealized_shape(self) -> StructuralShape {
        match self.dimensions() {
            SectionDimensions::WideFlange {
                depth,
                flange_width,
                web_thickness,
                flange_thickness,
            }
            | SectionDimensions::IBeam {
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
            SectionDimensions::Channel {
                depth,
                flange_width,
                web_thickness,
                flange_thickness,
            } => StructuralShape::new_channel(
                depth.value,
                flange_width.value,
                web_thickness.value,
                flange_thickness.value,
            ),
            SectionDimensions::Tee {
                depth,
                flange_width,
                web_thickness,
                flange_thickness,
            } => StructuralShape::new_tee(
                depth.value,
                flange_width.value,
                web_thickness.value,
                flange_thickness.value,
            ),
            SectionDimensions::Angle {
                vertical_leg,
                horizontal_leg,
                thickness,
            } => StructuralShape::new_angle(
                vertical_leg.value,
                horizontal_leg.value,
                thickness.value,
            ),
            SectionDimensions::DoubleAngle {
                back_to_back_leg,
                outstanding_leg,
                thickness,
                gap,
            } => StructuralShape::new_double_angle(
                back_to_back_leg.value,
                outstanding_leg.value,
                thickness.value,
                gap.value,
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

/// A name that is not in the AISC v16.0 catalog.
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
        write!(f, "unknown AISC section: {:?}", self.input)
    }
}

impl Error for ParseAiscSectionError {}

impl FromStr for AiscSection {
    type Err = ParseAiscSectionError;

    /// Accept US Manual or EDI labels, ignoring ASCII case and outer whitespace.
    /// Other aliases, metric designations, and unknown sections are rejected.
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
