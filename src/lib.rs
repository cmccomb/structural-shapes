#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![warn(clippy::missing_docs_in_private_items)]

//! This package provides utilities for designing and analyzing truss structures

/// This enum contains different structural shapes
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum StructuralShape {
    /// This is a pipe with an outer_radius and a thickness
    Pipe {
        /// Outer radius of hte pipe
        outer_radius: f64,
        /// Thickness of the pipe wall
        thickness: f64,
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
    },
    /// This is a box beam with a width, height, and thickness
    BoxBeam {
        /// Width of the box beam
        width: f64,
        /// Height of the box beam
        height: f64,
        /// Thickness of the wall
        thickness: f64,
    },
    /// This is a rod with a radius only
    Rod {
        /// Radius of the road
        radius: f64,
    },
    /// This is a solid rectangular with width and height
    Rectangle {
        /// Width of the rectangle
        width: f64,
        /// Height of the rectangle
        height: f64,
    },
}

impl StructuralShape {
    /// This function returns the moment of inertia of the structural shape around the x-axis
    /// ```
    /// use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::Rod{radius: 2.0};
    /// let area = shape.moi_x();
    /// ```
    pub fn moi_x(&self) -> f64 {
        match self {
            StructuralShape::Pipe {
                outer_radius,
                thickness,
            } => {
                StructuralShape::Rod {
                    radius: *outer_radius,
                }
                .moi_x()
                    - StructuralShape::Rod {
                        radius: (outer_radius - thickness),
                    }
                    .moi_x()
            }
            StructuralShape::IBeam {
                width,
                height,
                flange_thickness,
                web_thickness,
            } => {
                StructuralShape::Rectangle {
                    width: *width,
                    height: *height,
                }
                .moi_x()
                    - 2.0
                        * StructuralShape::Rectangle {
                            width: ((width - web_thickness) / 2.0),
                            height: (height - 2.0 * flange_thickness),
                        }
                        .moi_x()
            }
            StructuralShape::BoxBeam {
                width,
                height,
                thickness,
            } => {
                StructuralShape::Rectangle {
                    width: *width,
                    height: *height,
                }
                .moi_x()
                    - StructuralShape::Rectangle {
                        width: (width - 2.0 * thickness),
                        height: (height - 2.0 * thickness),
                    }
                    .moi_x()
            }
            StructuralShape::Rod { radius } => std::f64::consts::PI * radius.powi(4) / 4.0,
            StructuralShape::Rectangle { width, height } => width * height.powi(3) / 12.0,
        }
    }

    /// This function returns the moment of inertia of hte structural shape around the y-axis
    /// ```
    /// use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::Rod{radius: 2.0};
    /// let area = shape.moi_y();
    /// ```
    pub fn moi_y(&self) -> f64 {
        match self {
            StructuralShape::Pipe { .. } => self.moi_x(),
            StructuralShape::IBeam {
                height,
                width,
                flange_thickness,
                web_thickness,
            } => {
                2.0 * StructuralShape::Rectangle {
                    height: *width,
                    width: *flange_thickness,
                }
                .moi_x()
                    + StructuralShape::Rectangle {
                        height: *web_thickness,
                        width: (height - 2.0 * flange_thickness),
                    }
                    .moi_x()
            }
            StructuralShape::BoxBeam {
                width,
                height,
                thickness,
            } => StructuralShape::BoxBeam {
                width: *height,
                height: *width,
                thickness: *thickness,
            }
            .moi_x(),
            StructuralShape::Rod { .. } => self.moi_x(),
            StructuralShape::Rectangle { .. } => self.moi_x(),
        }
    }

    /// This function returns the cross-sectional area of the structural shape
    /// ```
    /// use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::Rod{radius: 2.0};
    /// let area = shape.area();
    /// ```
    pub fn area(&self) -> f64 {
        match *self {
            StructuralShape::Pipe {
                outer_radius,
                thickness,
            } => std::f64::consts::PI * (outer_radius.powi(2) - (outer_radius - thickness).powi(2)),
            StructuralShape::IBeam {
                width,
                height,
                web_thickness,
                flange_thickness,
            } => width * height - (height - 2.0 * flange_thickness) * (width - web_thickness),
            StructuralShape::BoxBeam {
                width,
                height,
                thickness,
            } => width * height - (width - 2.0 * thickness) * (height - 2.0 * thickness),
            StructuralShape::Rod { radius } => std::f64::consts::PI * radius.powi(2),
            StructuralShape::Rectangle { width, height } => width * height,
        }
    }

    /// This function returns the moment of intertia of the structural shape around the x-axis when
    /// displaced perpendicular to the axis by a distance d
    /// ```
    /// use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::Rod{radius: 2.0};
    /// let area = shape.moi_x_d(2.0);
    /// ```
    pub fn moi_x_d(&self, d: f64) -> f64 {
        self.moi_x() + self.area() * d.powi(2)
    }

    /// This function returns the moment of intertia of the structural shape around the y-axis when
    /// displaced perpendicular to the axis by a distance d
    /// ```
    /// use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::Rod{radius: 2.0};
    /// let area = shape.moi_y_d(2.0);
    /// ```
    pub fn moi_y_d(&self, d: f64) -> f64 {
        self.moi_y() + self.area() * d.powi(2)
    }
}
