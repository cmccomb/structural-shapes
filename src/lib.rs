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
            } => {
                StructuralShape::Rod {
                    radius: *outer_radius,
                    center_of_gravity: *center_of_gravity,
                }
                .moi_x()
                    - StructuralShape::Rod {
                        radius: (outer_radius - thickness),
                        center_of_gravity: *center_of_gravity,
                    }
                    .moi_x()
            }
            StructuralShape::IBeam {
                width,
                height,
                flange_thickness,
                web_thickness,
                center_of_gravity,
            } => {
                StructuralShape::Rectangle {
                    width: *width,
                    height: *height,
                    center_of_gravity: *center_of_gravity,
                }
                .moi_x()
                    - 2.0
                        * StructuralShape::Rectangle {
                            width: ((width - web_thickness) / 2.0),
                            height: (height - 2.0 * flange_thickness),
                            center_of_gravity: *center_of_gravity,
                        }
                        .moi_x()
            }
            StructuralShape::BoxBeam {
                width,
                height,
                thickness,
                center_of_gravity,
            } => {
                StructuralShape::Rectangle {
                    width: *width,
                    height: *height,
                    center_of_gravity: *center_of_gravity,
                }
                .moi_x()
                    - StructuralShape::Rectangle {
                        width: (width - 2.0 * thickness),
                        height: (height - 2.0 * thickness),
                        center_of_gravity: *center_of_gravity,
                    }
                    .moi_x()
            }
            StructuralShape::Rod {
                radius,
                center_of_gravity,
            } => {
                std::f64::consts::PI * radius.powi(4) / 4.0
                    + self.area() * center_of_gravity.0.powf(2.0)
            }
            StructuralShape::Rectangle {
                width,
                height,
                center_of_gravity,
            } => width * height.powi(3) / 12.0 + self.area() * center_of_gravity.0.powf(2.0),
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
            } => {
                StructuralShape::Rod {
                    radius: *outer_radius,
                    center_of_gravity: *center_of_gravity,
                }
                .moi_y()
                    - StructuralShape::Rod {
                        radius: (outer_radius - thickness),
                        center_of_gravity: *center_of_gravity,
                    }
                    .moi_y()
            }

            StructuralShape::IBeam {
                height,
                width,
                flange_thickness,
                web_thickness,
                center_of_gravity,
            } => {
                2.0 * StructuralShape::Rectangle {
                    height: *width,
                    width: *flange_thickness,
                    center_of_gravity: (center_of_gravity.1, center_of_gravity.0),
                }
                .moi_x()
                    + StructuralShape::Rectangle {
                        height: *web_thickness,
                        width: (height - 2.0 * flange_thickness),
                        center_of_gravity: (center_of_gravity.1, center_of_gravity.0),
                    }
                    .moi_x()
            }
            StructuralShape::BoxBeam {
                width,
                height,
                thickness,
                center_of_gravity,
            } => StructuralShape::BoxBeam {
                width: *height,
                height: *width,
                thickness: *thickness,
                center_of_gravity: (center_of_gravity.1, center_of_gravity.0),
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
}

/// A composite composed of multiple individual shapes
/// ```
/// # use structural_shapes::*;
/// let mut x = CompositeShape::default();
/// x.add(StructuralShape::Rod {radius: 2.0, center_of_gravity: (2.0, 0.0)});
/// x.add(StructuralShape::Rod {radius: 2.0, center_of_gravity: (-2.0, 0.0)});
/// ```
pub struct CompositeShape {
    /// Constituent shapes
    pub shapes: Vec<StructuralShape>,
}

impl CompositeShape {
    /// This function adds a new shape to the composite
    pub fn add(&mut self, new_shape: StructuralShape) {
        self.shapes.push(new_shape);
    }
    /// This function returns the moment of inertia of the composite shape around the x-axis
    pub fn moi_x(&mut self) -> f64 {
        self.shapes.iter().map(|x| x.moi_x()).sum()
    }
    /// This function returns the moment of inertia of the composite shape around the y-axis
    pub fn moi_y(&mut self) -> f64 {
        self.shapes.iter().map(|x| x.moi_y()).sum()
    }
    /// This function returns the area of the composite shape
    pub fn area(&mut self) -> f64 {
        self.shapes.iter().map(|x| x.area()).sum()
    }
}

impl Default for CompositeShape {
    fn default() -> Self {
        CompositeShape { shapes: vec![] }
    }
}
