#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

mod open_sections;
mod section_mechanics;

pub use section_mechanics::AreaMoments;

use num::{Float, NumCast};
use typenum::{P4, P6, Z0};
use uom::si::{
    f64::{Area, Length, Volume},
    length::meter,
    {Quantity, ISQ, SI},
};
/// Second moment of area, stored in meters to the fourth power.
pub type SecondAreaMomentofInertia = Quantity<ISQ<P4, Z0, Z0, Z0, Z0, Z0, Z0>, SI<f64>, f64>;
/// Elastic/plastic section modulus (length cubed), stored in cubic meters.
pub type SectionModulus = Volume;
/// Warping constant (length to the sixth power), stored in meters to the sixth power.
pub type WarpingConstant = Quantity<ISQ<P6, Z0, Z0, Z0, Z0, Z0, Z0>, SI<f64>, f64>;

#[cfg(feature = "aisc")]
pub mod aisc;
#[cfg(feature = "aisc")]
pub use aisc::{
    AiscSection, ParseAiscSectionError, SectionDimensions, SectionFamily, SectionProperties,
};

/// A helper function supporting conversion of floating point numbers to meters
pub fn meters<T: Float>(l: T) -> Length {
    Length::new::<meter>(NumCast::from(l).expect("The input must be castable to a float."))
}

/// This enum contains different structural shapes
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum StructuralShape {
    /// Sharp-corner channel with a vertical web on the left and flanges pointing right.
    Channel {
        /// Overall horizontal width.
        width: Length,
        /// Overall vertical height.
        height: Length,
        /// Uniform web thickness.
        web_thickness: Length,
        /// Uniform flange thickness.
        flange_thickness: Length,
        /// Position of the geometric centroid.
        center_of_gravity: (Length, Length),
    },
    /// Sharp-corner tee with a horizontal top flange and a vertical stem below it.
    Tee {
        /// Overall flange width.
        width: Length,
        /// Overall vertical height, including the flange.
        height: Length,
        /// Uniform stem thickness.
        web_thickness: Length,
        /// Uniform flange thickness.
        flange_thickness: Length,
        /// Position of the geometric centroid.
        center_of_gravity: (Length, Length),
    },
    /// Sharp-corner L with a vertical leg on the left and a bottom leg pointing right.
    Angle {
        /// Overall horizontal leg width.
        width: Length,
        /// Overall vertical leg height.
        height: Length,
        /// Uniform thickness of both legs.
        thickness: Length,
        /// Position of the geometric centroid.
        center_of_gravity: (Length, Length),
    },
    /// Mirrored sharp-corner angles with vertical legs back-to-back and bottom legs outward.
    DoubleAngle {
        /// Overall horizontal width of each individual angle.
        width: Length,
        /// Vertical height of the back-to-back legs.
        height: Length,
        /// Uniform thickness of the angle legs.
        thickness: Length,
        /// Clear horizontal spacing between the backs of the vertical legs.
        gap: Length,
        /// Position of the combined geometric centroid.
        center_of_gravity: (Length, Length),
    },
    /// This is a pipe with an outer_radius and a thickness
    Pipe {
        /// Outer radius of hte pipe
        outer_radius: Length,
        /// Thickness of the pipe wall
        thickness: Length,
        /// Coordinates of center of gravity
        center_of_gravity: (Length, Length),
    },
    /// This is an I-Beam, with a width, height, web thickness, and flange thickness
    IBeam {
        /// Width of the beam
        width: Length,
        /// Height of the beam
        height: Length,
        /// Thickness of the web
        web_thickness: Length,
        /// Thickness of the flange
        flange_thickness: Length,
        /// Coordinates of center of gravity
        center_of_gravity: (Length, Length),
    },
    /// This is a box beam with a width, height, and thickness
    BoxBeam {
        /// Width of the box beam
        width: Length,
        /// Height of the box beam
        height: Length,
        /// Thickness of the wall
        thickness: Length,
        /// Coordinates of center of gravity
        center_of_gravity: (Length, Length),
    },
    /// This is a rod with a radius only
    Rod {
        /// Radius of the road
        radius: Length,
        /// Coordinates of center of gravity
        center_of_gravity: (Length, Length),
    },
    /// This is a solid rectangular with width and height
    Rectangle {
        /// Width of the rectangle
        width: Length,
        /// Height of the rectangle
        height: Length,
        /// Coordinates of center of gravity
        center_of_gravity: (Length, Length),
    },
}

impl StructuralShape {
    /// Make a new rod without COG
    /// ```
    /// # use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::new_rod(2.0);
    /// ```
    pub fn new_rod(radius: f64) -> StructuralShape {
        StructuralShape::Rod {
            radius: meters(radius),
            center_of_gravity: (meters(0.0), meters(0.0)),
        }
    }

    /// Make a new pipe without COG
    /// ```
    /// # use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::new_pipe(2.0, 0.15);
    /// ```
    pub fn new_pipe(radius: f64, thickness: f64) -> StructuralShape {
        StructuralShape::Pipe {
            outer_radius: meters(radius),
            thickness: meters(thickness),
            center_of_gravity: (meters(0.0), meters(0.0)),
        }
    }

    /// Make a new rectangle without COG
    /// ```
    /// # use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::new_rectangle(2.0, 2.0);
    /// ```
    pub fn new_rectangle(height: f64, width: f64) -> StructuralShape {
        StructuralShape::Rectangle {
            width: meters(width),
            height: meters(height),
            center_of_gravity: (meters(0.0), meters(0.0)),
        }
    }

    /// Make a new boxbeam without COG
    /// ```
    /// # use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::new_boxbeam(2.0, 2.0, 0.15);
    /// ```
    pub fn new_boxbeam(height: f64, width: f64, thickness: f64) -> StructuralShape {
        StructuralShape::BoxBeam {
            width: meters(width),
            height: meters(height),
            thickness: meters(thickness),
            center_of_gravity: (meters(0.0), meters(0.0)),
        }
    }

    /// Make a new Ibeam without COG
    /// ```
    /// # use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::new_ibeam(2.0, 2.0, 0.15, 0.15);
    /// ```
    pub fn new_ibeam(
        height: f64,
        width: f64,
        web_thickness: f64,
        flange_thickness: f64,
    ) -> StructuralShape {
        StructuralShape::IBeam {
            width: meters(width),
            height: meters(height),
            web_thickness: meters(web_thickness),
            center_of_gravity: (meters(0.0), meters(0.0)),
            flange_thickness: meters(flange_thickness),
        }
    }

    /// Second moment about the horizontal x-axis through the origin, including `A * y²`.
    /// ```
    /// # use structural_shapes::{StructuralShape};
    /// let shape = StructuralShape::new_rod(2.0);
    /// let moi = shape.moi_x();
    /// ```
    pub fn moi_x(&self) -> SecondAreaMomentofInertia {
        match *self {
            Self::Channel { .. }
            | Self::Tee { .. }
            | Self::Angle { .. }
            | Self::DoubleAngle { .. } => self.open_section_composite().moi_x(),
            StructuralShape::Pipe {
                outer_radius,
                thickness,
                center_of_gravity,
            } => CompositeShape::new()
                .add(StructuralShape::Rod {
                    radius: outer_radius,
                    center_of_gravity,
                })
                .sub(StructuralShape::Rod {
                    radius: (outer_radius - thickness),
                    center_of_gravity,
                })
                .moi_x(),
            StructuralShape::IBeam {
                width,
                height,
                flange_thickness,
                web_thickness,
                center_of_gravity,
            } => composite_ibeam(
                width,
                height,
                web_thickness,
                flange_thickness,
                center_of_gravity,
            )
            .moi_x(),
            StructuralShape::BoxBeam {
                width,
                height,
                thickness,
                center_of_gravity,
            } => CompositeShape::new()
                .add(StructuralShape::Rectangle {
                    width,
                    height,
                    center_of_gravity,
                })
                .sub(StructuralShape::Rectangle {
                    width: (width - 2.0 * thickness),
                    height: (height - 2.0 * thickness),
                    center_of_gravity,
                })
                .moi_x(),
            StructuralShape::Rod {
                radius,
                center_of_gravity,
            } => {
                std::f64::consts::PI * radius * radius * radius * radius / 4.0
                    + self.area() * center_of_gravity.1 * center_of_gravity.1
            }
            StructuralShape::Rectangle {
                width,
                height,
                center_of_gravity,
            } => {
                width * height * height * height / 12.0
                    + self.area() * center_of_gravity.1 * center_of_gravity.1
            }
        }
    }

    /// Second moment about the vertical y-axis through the origin, including `A * x²`.
    /// ```
    /// # use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::new_rod(2.0);
    /// let area = shape.moi_y();
    /// ```
    pub fn moi_y(&self) -> SecondAreaMomentofInertia {
        match *self {
            Self::Channel { .. }
            | Self::Tee { .. }
            | Self::Angle { .. }
            | Self::DoubleAngle { .. } => self.open_section_composite().moi_y(),
            StructuralShape::Pipe {
                outer_radius,
                thickness,
                center_of_gravity,
            } => StructuralShape::Pipe {
                outer_radius,
                thickness,
                center_of_gravity: swap(center_of_gravity),
            }
            .moi_x(),

            StructuralShape::IBeam {
                height,
                width,
                flange_thickness,
                web_thickness,
                center_of_gravity,
            } => composite_ibeam(
                width,
                height,
                web_thickness,
                flange_thickness,
                center_of_gravity,
            )
            .moi_y(),
            StructuralShape::BoxBeam {
                width,
                height,
                thickness,
                center_of_gravity,
            } => StructuralShape::BoxBeam {
                width: height,
                height: width,
                thickness,
                center_of_gravity: swap(center_of_gravity),
            }
            .moi_x(),
            StructuralShape::Rod {
                radius,
                center_of_gravity,
            } => {
                std::f64::consts::PI * radius * radius * radius * radius / 4.0
                    + self.area() * center_of_gravity.0 * center_of_gravity.0
            }
            StructuralShape::Rectangle {
                width,
                height,
                center_of_gravity,
            } => {
                height * width * width * width / 12.0
                    + self.area() * center_of_gravity.0 * center_of_gravity.0
            }
        }
    }

    /// Polar second moment of area about the origin: `Ix + Iy`.
    ///
    /// This is generally **not** the Saint-Venant torsional constant `J`.
    /// For a circular rod or concentric pipe they coincide only about the centroid.
    /// Use [`Self::torsional_constant`] for supported torsion calculations.
    /// ```
    /// # use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::new_rod(2.0);
    /// let area = shape.polar_moi();
    /// ```
    pub fn polar_moi(&self) -> SecondAreaMomentofInertia {
        self.moi_x() + self.moi_y()
    }

    /// This function returns the cross-sectional area of the structural shape
    /// ```
    /// # use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::new_rod(2.0);
    /// let area = shape.area();
    /// ```
    pub fn area(&self) -> Area {
        match *self {
            Self::Channel {
                width,
                height,
                web_thickness,
                flange_thickness,
                ..
            } => 2.0 * width * flange_thickness + (height - 2.0 * flange_thickness) * web_thickness,
            Self::Tee {
                width,
                height,
                web_thickness,
                flange_thickness,
                ..
            } => width * flange_thickness + (height - flange_thickness) * web_thickness,
            Self::Angle {
                width,
                height,
                thickness,
                ..
            } => (width + height - thickness) * thickness,
            Self::DoubleAngle {
                width,
                height,
                thickness,
                ..
            } => 2.0 * (width + height - thickness) * thickness,
            StructuralShape::Pipe {
                outer_radius,
                thickness,
                ..
            } => {
                std::f64::consts::PI
                    * (outer_radius * outer_radius
                        - (outer_radius - thickness) * (outer_radius - thickness))
            }
            StructuralShape::IBeam {
                width,
                height,
                web_thickness,
                flange_thickness,
                ..
            } => width * height - (height - 2.0 * flange_thickness) * (width - web_thickness),
            StructuralShape::BoxBeam {
                width,
                height,
                thickness,
                ..
            } => width * height - (width - 2.0 * thickness) * (height - 2.0 * thickness),
            StructuralShape::Rod { radius, .. } => std::f64::consts::PI * radius * radius,
            StructuralShape::Rectangle { width, height, .. } => width * height,
        }
    }

    /// A function to set the center of gravity of a shape
    /// ```
    /// # use structural_shapes::{StructuralShape};
    /// let shape = StructuralShape::new_rod(2.0).with_cog(0.0, 0.0);
    /// let moi = shape.moi_x();
    /// ```
    pub fn with_cog(&mut self, x: f64, y: f64) -> StructuralShape {
        self.set_cog((meters(x), meters(y)));
        *self
    }

    /// A function to return the current center of gravity for a shape
    pub(crate) fn get_cog(&self) -> (Length, Length) {
        match *self {
            Self::Channel {
                center_of_gravity, ..
            }
            | Self::Tee {
                center_of_gravity, ..
            }
            | Self::Angle {
                center_of_gravity, ..
            }
            | Self::DoubleAngle {
                center_of_gravity, ..
            } => center_of_gravity,
            StructuralShape::Pipe {
                center_of_gravity, ..
            } => center_of_gravity,
            StructuralShape::IBeam {
                center_of_gravity, ..
            } => center_of_gravity,
            StructuralShape::BoxBeam {
                center_of_gravity, ..
            } => center_of_gravity,
            StructuralShape::Rod {
                center_of_gravity, ..
            } => center_of_gravity,
            StructuralShape::Rectangle {
                center_of_gravity, ..
            } => center_of_gravity,
        }
    }

    /// A function to set the current center of gravity for a shape
    pub(crate) fn set_cog(&mut self, cog: (Length, Length)) {
        match *self {
            Self::Channel {
                ref mut center_of_gravity,
                ..
            }
            | Self::Tee {
                ref mut center_of_gravity,
                ..
            }
            | Self::Angle {
                ref mut center_of_gravity,
                ..
            }
            | Self::DoubleAngle {
                ref mut center_of_gravity,
                ..
            } => {
                *center_of_gravity = cog;
            }
            StructuralShape::Pipe {
                ref mut center_of_gravity,
                ..
            } => {
                *center_of_gravity = cog;
            }
            StructuralShape::IBeam {
                ref mut center_of_gravity,
                ..
            } => {
                *center_of_gravity = cog;
            }
            StructuralShape::BoxBeam {
                ref mut center_of_gravity,
                ..
            } => {
                *center_of_gravity = cog;
            }
            StructuralShape::Rod {
                ref mut center_of_gravity,
                ..
            } => {
                *center_of_gravity = cog;
            }
            StructuralShape::Rectangle {
                ref mut center_of_gravity,
                ..
            } => {
                *center_of_gravity = cog;
            }
        };
    }
}

/// A composite composed of multiple individual shapes
/// ```
/// # use structural_shapes::*;
/// let x = CompositeShape::new()
///     .add(StructuralShape::new_rod(2.0).with_cog(2.0, 0.0))
///     .add(StructuralShape::new_rod(2.0).with_cog(-2.0, 0.0));
/// ```
#[derive(Clone, Debug)]
pub struct CompositeShape {
    /// Constituent shapes
    pub shapes: Vec<(i8, StructuralShape)>,
}

impl CompositeShape {
    /// This creates a new composite shape, identical to default
    pub fn new() -> Self {
        Self::default()
    }
    /// This function adds a new shape to the composite
    pub fn add(&mut self, new_shape: StructuralShape) -> Self {
        self.shapes.push((1, new_shape));
        self.clone()
    }
    /// This function subtracts a new shape to the composite
    pub fn sub(&mut self, new_shape: StructuralShape) -> Self {
        self.shapes.push((-1, new_shape));
        self.clone()
    }
    /// Calculate the centroid without moving members.
    ///
    /// Empty or zero-area composites have no defined centroid and yield nonfinite coordinates.
    /// Use [`Self::try_calculate_cog`] to check for a finite positive net area.
    pub fn calculate_cog(&self) -> (Length, Length) {
        let area = self.area();
        let area_times_cx: Volume = self
            .shapes
            .iter()
            .map(|x| {
                let center_of_gravity = x.1.get_cog();
                (x.0 as f64) * x.1.area() * center_of_gravity.0
            })
            .sum();
        let area_times_cy: Volume = self
            .shapes
            .iter()
            .map(|x| {
                let center_of_gravity = x.1.get_cog();
                (x.0 as f64) * x.1.area() * center_of_gravity.1
            })
            .sum();
        let cog_x = area_times_cx / area;
        let cog_y = area_times_cy / area;
        (cog_x, cog_y)
    }
    /// Shift structure to have cog at (0.0,0.0)
    pub fn update_cog(&mut self) {
        let (cog_x, cog_y) = self.calculate_cog();
        self.shapes.iter_mut().for_each(|x| {
            let (_, ref mut shape) = x;
            let (old_x, old_y) = shape.get_cog();
            shape.set_cog((old_x - cog_x, old_y - cog_y));
        });
    }

    /// This function returns the moment of inertia of the composite shape around the x-axis
    pub fn moi_x(&self) -> SecondAreaMomentofInertia {
        self.shapes.iter().map(|x| (x.0 as f64) * x.1.moi_x()).sum()
    }
    /// This function returns the moment of inertia of the composite shape around the y-axis
    pub fn moi_y(&self) -> SecondAreaMomentofInertia {
        self.shapes.iter().map(|x| (x.0 as f64) * x.1.moi_y()).sum()
    }
    /// Polar second moment of area about the origin: `Ix + Iy`, generally not `J`.
    /// Torsional constants cannot in general be added/subtracted like area moments.
    pub fn polar_moi(&self) -> SecondAreaMomentofInertia {
        self.moi_x() + self.moi_y()
    }
    /// This function returns the area of the composite shape
    pub fn area(&self) -> Area {
        self.shapes.iter().map(|x| (x.0 as f64) * x.1.area()).sum()
    }
}

/// Implement default
impl Default for CompositeShape {
    fn default() -> Self {
        CompositeShape { shapes: vec![] }
    }
}

/// Function for swapping values
fn swap(pair: (Length, Length)) -> (Length, Length) {
    (pair.1, pair.0)
}

/// Create a composite I-beam from some initial parameters
fn composite_ibeam(
    width: Length,
    height: Length,
    web_thickness: Length,
    flange_thickness: Length,
    center_of_gravity: (Length, Length),
) -> CompositeShape {
    CompositeShape::new()
        .add(StructuralShape::Rectangle {
            width,
            height,
            center_of_gravity,
        })
        .sub(StructuralShape::Rectangle {
            width: ((width - web_thickness) / 2.0),
            height: (height - 2.0 * flange_thickness),
            center_of_gravity: (
                center_of_gravity.0 - ((width - web_thickness) / 4.0) - web_thickness / 2.0,
                center_of_gravity.1,
            ),
        })
        .sub(StructuralShape::Rectangle {
            width: ((width - web_thickness) / 2.0),
            height: (height - 2.0 * flange_thickness),
            center_of_gravity: (
                center_of_gravity.0 + ((width - web_thickness) / 4.0) + web_thickness / 2.0,
                center_of_gravity.1,
            ),
        })
}
