#![warn(clippy::all)]
#![warn(missing_docs)]
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
    /// let area = shape.moment_of_inertia_x();
    /// ```
    pub fn moment_of_inertia_x(&self) -> f64 {
        match self {
            StructuralShape::Pipe {
                outer_radius,
                thickness,
            } => {
                std::f64::consts::PI * (outer_radius.powi(4) - (outer_radius - thickness).powi(4))
                    / 4.0
            }
            StructuralShape::IBeam {
                width,
                height,
                flange_thickness,
                web_thickness,
            } => {
                width * height.powi(3) / 12.0
                    - 2.0
                        * ((width - web_thickness) / 2.0)
                        * (height - 2.0 * flange_thickness).powi(3)
                        / 12.0
            }
            StructuralShape::BoxBeam {
                width,
                height,
                thickness,
            } => {
                width * height.powi(3) / 12.0
                    - (width - thickness) * (height - thickness).powi(3) / 12.0
            }
            StructuralShape::Rod { radius } => std::f64::consts::PI * radius.powi(4) / 4.0,
            StructuralShape::Rectangle { width, height } => width * height.powi(3) / 12.0,
        }
    }

    /// This function returns the moment of inertia of hte structural shape around the y-axis
    /// ```
    /// use structural_shapes::StructuralShape;
    /// let shape = StructuralShape::Rod{radius: 2.0};
    /// let area = shape.moment_of_inertia_y();
    /// ```
    pub fn moment_of_inertia_y(&self) -> f64 {
        match self {
            StructuralShape::Pipe { .. } => self.moment_of_inertia_x(),
            StructuralShape::IBeam { .. } => 0.0,
            StructuralShape::BoxBeam {
                width,
                height,
                thickness,
            } => {
                height * width.powi(3) / 12.0
                    - (height - thickness) * (width - thickness).powi(3) / 12.0
            }
            StructuralShape::Rod { .. } => self.moment_of_inertia_x(),
            StructuralShape::Rectangle { .. } => self.moment_of_inertia_x(),
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
            } => width * height - (width - thickness) * (height - thickness),
            StructuralShape::Rod { radius } => std::f64::consts::PI * radius.powi(2),
            StructuralShape::Rectangle { width, height } => width * height,
        }
    }
}
