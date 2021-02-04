//! This package provides utilities for designing and analyzing truss structures

/// This enum contains different structural shapes
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum StructuralShape {
    Pipe {
        outer_radius: f64,
        thickness: f64,
    },
    IBeam {
        width: f64,
        height: f64,
        web_thickness: f64,
        flange_thickness: f64,
    },
    BoxBeam {
        width: f64,
        height: f64,
        thickness: f64,
    },
    Rod {
        outer_radius: f64,
    },
}

impl StructuralShape {
    pub fn moment_of_inertia(&self) -> f64 {
        match self {
            StructuralShape::Pipe { .. } => 0.0,
            StructuralShape::IBeam { .. } => 0.0,
            StructuralShape::BoxBeam { .. } => 0.0,
            _ => 0.0,
        }
    }

    pub fn area(&self) -> f64 {
        match *self {
            StructuralShape::Pipe {
                outer_radius,
                thickness,
            } => {
                std::f64::consts::PI
                    * (outer_radius.powf(2.0) - (outer_radius - thickness).powf(2.0))
            }
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
            _ => 0.0,
        }
    }
}
