#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![warn(clippy::missing_docs_in_private_items)]

//! This package provides utilities for designing and analyzing truss structures

/// This enum contains different structural shapes
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum StructuralShape {
    /// This is a pipe with an outer_radius and a thickness
    Pipe {
        /// Outer radius of hte pipe
        outer_radius: f64,
        /// Thickness of the pipe wall
        thickness: f64,
        /// Coordinates of center of gravity
        center_of_gravity: (f64, f64),
    },
    /// This is an I-Beam, with a width, height, web thickness, and flange thickness
    IBeam {
        /// Width of the beam
        width: f64,
        /// Height of the beam
        height: f64,
        /// Thickness of the web
        web_thickness: f64,
        /// Thickness of the flange
        flange_thickness: f64,
        /// Coordinates of center of gravity
        center_of_gravity: (f64, f64),
    },
    /// This is a box beam with a width, height, and thickness
    BoxBeam {
        /// Width of the box beam
        width: f64,
        /// Height of the box beam
        height: f64,
        /// Thickness of the wall
        thickness: f64,
        /// Coordinates of center of gravity
        center_of_gravity: (f64, f64),
    },
    /// This is a rod with a radius only
    Rod {
        /// Radius of the road
        radius: f64,
        /// Coordinates of center of gravity
        center_of_gravity: (f64, f64),
    },
    /// This is a solid rectangular with width and height
    Rectangle {
        /// Width of the rectangle
        width: f64,
        /// Height of the rectangle
        height: f64,
        /// Coordinates of center of gravity
        center_of_gravity: (f64, f64),
    },
}

impl StructuralShape {
    /// This function returns the moment of inertia of the structural shape around the x-axis
    /// ```
    /// # use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::Rod{radius: 2.0, center_of_gravity: (0.0, 0.0)};
    /// let area = shape.moi_x();
    /// ```
    pub fn moi_x(&self) -> f64 {
        match self {
            StructuralShape::Pipe {
                outer_radius,
                thickness,
                center_of_gravity,
            } => CompositeShape::new()
                .add(StructuralShape::Rod {
                    radius: *outer_radius,
                    center_of_gravity: *center_of_gravity,
                })
                .sub(StructuralShape::Rod {
                    radius: (outer_radius - thickness),
                    center_of_gravity: *center_of_gravity,
                })
                .moi_x(),
            StructuralShape::IBeam {
                width,
                height,
                flange_thickness,
                web_thickness,
                center_of_gravity,
            } => composite_ibeam(
                *width,
                *height,
                *web_thickness,
                *flange_thickness,
                *center_of_gravity,
            )
            .moi_y(),
            StructuralShape::BoxBeam {
                width,
                height,
                thickness,
                center_of_gravity,
            } => CompositeShape::new()
                .add(StructuralShape::Rectangle {
                    width: *width,
                    height: *height,
                    center_of_gravity: *center_of_gravity,
                })
                .sub(StructuralShape::Rectangle {
                    width: (width - 2.0 * thickness),
                    height: (height - 2.0 * thickness),
                    center_of_gravity: *center_of_gravity,
                })
                .moi_x(),
            StructuralShape::Rod {
                radius,
                center_of_gravity,
            } => {
                std::f64::consts::PI * radius.powi(4) / 4.0
                    + self.area() * center_of_gravity.0.powi(2)
            }
            StructuralShape::Rectangle {
                width,
                height,
                center_of_gravity,
            } => width * height.powi(3) / 12.0 + self.area() * center_of_gravity.0.powi(2),
        }
    }

    /// This function returns the moment of inertia of hte structural shape around the y-axis
    /// ```
    /// # use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::Rod{radius: 2.0, center_of_gravity: (0.0, 0.0)};
    /// let area = shape.moi_y();
    /// ```
    pub fn moi_y(&self) -> f64 {
        match self {
            StructuralShape::Pipe {
                outer_radius,
                thickness,
                center_of_gravity,
            } => StructuralShape::Pipe {
                outer_radius: *outer_radius,
                thickness: *thickness,
                center_of_gravity: swap(*center_of_gravity),
            }
            .moi_x(),

            StructuralShape::IBeam {
                height,
                width,
                flange_thickness,
                web_thickness,
                center_of_gravity,
            } => composite_ibeam(
                *width,
                *height,
                *web_thickness,
                *flange_thickness,
                *center_of_gravity,
            )
            .moi_y(),
            StructuralShape::BoxBeam {
                width,
                height,
                thickness,
                center_of_gravity,
            } => StructuralShape::BoxBeam {
                width: *height,
                height: *width,
                thickness: *thickness,
                center_of_gravity: swap(*center_of_gravity),
            }
            .moi_x(),
            StructuralShape::Rod {
                radius,
                center_of_gravity,
            } => {
                std::f64::consts::PI * radius.powi(4) / 4.0
                    + self.area() * center_of_gravity.1.powf(2.0)
            }
            StructuralShape::Rectangle {
                width,
                height,
                center_of_gravity,
            } => width * height.powi(3) / 12.0 + self.area() * center_of_gravity.0.powf(2.0),
        }
    }

    /// This function returns the cross-sectional area of the structural shape
    /// ```
    /// # use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::Rod{radius: 2.0, center_of_gravity: (0.0, 0.0)};
    /// let area = shape.area();
    /// ```
    pub fn area(&self) -> f64 {
        match *self {
            StructuralShape::Pipe {
                outer_radius,
                thickness,
                ..
            } => std::f64::consts::PI * (outer_radius.powi(2) - (outer_radius - thickness).powi(2)),
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
            StructuralShape::Rod { radius, .. } => std::f64::consts::PI * radius.powi(2),
            StructuralShape::Rectangle { width, height, .. } => width * height,
        }
    }

    /// A function to return the current center of gravity for a shape
    pub(crate) fn get_cog(&self) -> (f64, f64) {
        match *self {
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
    pub(crate) fn set_cog(&mut self, cog: (f64, f64)) {
        match *self {
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
///     .add(StructuralShape::Rod {
///         radius: 2.0,
///         center_of_gravity: (2.0, 0.0)
///     })
///     .add(StructuralShape::Rod {
///         radius: 2.0,
///         center_of_gravity: (-2.0, 0.0)
///     });
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
    /// Calculate center of gravity and update COG of members
    pub fn calculate_cog(&self) -> (f64, f64) {
        let area = self.area();
        let area_times_cx: f64 = self
            .shapes
            .iter()
            .map(|x| {
                let center_of_gravity = x.1.get_cog();
                (x.0 as f64) * x.1.area() * center_of_gravity.0
            })
            .sum();
        let area_times_cy: f64 = self
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
    pub fn moi_x(&self) -> f64 {
        self.shapes.iter().map(|x| (x.0 as f64) * x.1.moi_x()).sum()
    }
    /// This function returns the moment of inertia of the composite shape around the y-axis
    pub fn moi_y(&self) -> f64 {
        self.shapes.iter().map(|x| (x.0 as f64) * x.1.moi_y()).sum()
    }
    /// This function returns the area of the composite shape
    pub fn area(&self) -> f64 {
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
fn swap(pair: (f64, f64)) -> (f64, f64) {
    (pair.1, pair.0)
}

/// Create a composite I-beam from some initial parameters
fn composite_ibeam(
    width: f64,
    height: f64,
    web_thickness: f64,
    flange_thickness: f64,
    center_of_gravity: (f64, f64),
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
