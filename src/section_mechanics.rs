//! Area moments, axis transformations, and supported torsional constants.

use crate::{meters, CompositeShape, SecondAreaMomentofInertia, StructuralShape};
use uom::si::{angle::radian, f64::Angle, f64::Length};

/// Second moments of area about a pair of perpendicular axes through one point.
///
/// The sign convention is `Ixy = integral(x * y dA)`. These are area moments,
/// not a torsional constant. For centroidal moments use
/// [`StructuralShape::centroidal_area_moments`] or
/// [`CompositeShape::centroidal_area_moments`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AreaMoments {
    /// Integral of y squared.
    ix: SecondAreaMomentofInertia,
    /// Integral of x squared.
    iy: SecondAreaMomentofInertia,
    /// Integral of x times y, including its sign.
    ixy: SecondAreaMomentofInertia,
}

impl AreaMoments {
    /// Construct moments about the same origin and axes; no physical validation is performed.
    pub fn new(
        ix: SecondAreaMomentofInertia,
        iy: SecondAreaMomentofInertia,
        ixy: SecondAreaMomentofInertia,
    ) -> Self {
        Self { ix, iy, ixy }
    }

    /// Second moment about the x-axis.
    pub fn moi_x(self) -> SecondAreaMomentofInertia {
        self.ix
    }

    /// Second moment about the y-axis.
    pub fn moi_y(self) -> SecondAreaMomentofInertia {
        self.iy
    }

    /// Signed product of inertia: `integral(x * y dA)`.
    pub fn product_moi(self) -> SecondAreaMomentofInertia {
        self.ixy
    }

    /// Polar second moment of area, `Ix + Iy`; generally not the torsional constant `J`.
    pub fn polar_moi(self) -> SecondAreaMomentofInertia {
        self.ix + self.iy
    }

    /// Moments when the coordinate axes rotate counterclockwise by `angle`.
    ///
    /// The shape and origin remain fixed: `x' = x cos(angle) + y sin(angle)`.
    pub fn rotated(self, angle: Angle) -> Self {
        let (sin, cos) = (2.0 * angle.get::<radian>()).sin_cos();
        let average = (self.ix + self.iy) / 2.0;
        let difference = (self.ix - self.iy) / 2.0;
        Self::new(
            average + difference * cos - self.ixy * sin,
            average - difference * cos + self.ixy * sin,
            difference * sin + self.ixy * cos,
        )
    }

    /// Principal second moments `(major, minor)` about this origin, in descending order.
    pub fn principal_moments(self) -> (SecondAreaMomentofInertia, SecondAreaMomentofInertia) {
        let average = (self.ix + self.iy) / 2.0;
        let mut radius = self.ix;
        radius.value = ((self.ix.value - self.iy.value) / 2.0).hypot(self.ixy.value);
        (average + radius, average - radius)
    }

    /// Counterclockwise angle from x to the axis with the major second moment.
    ///
    /// Returns `None` if the moments are isotropic (every axis is principal).
    /// Otherwise the angle is in `[-pi/2, pi/2]`, equivalent modulo pi.
    pub fn principal_axis_angle(self) -> Option<Angle> {
        if self.ix == self.iy && self.ixy.value == 0.0 {
            None
        } else {
            Some(Angle::new::<radian>(
                (-self.ixy.value).atan2((self.ix.value - self.iy.value) / 2.0) / 2.0,
            ))
        }
    }
}

impl StructuralShape {
    /// Exact Saint-Venant torsional constant for a circular rod or concentric pipe.
    ///
    /// Independent of the shape's position. Returns `None` for other geometries:
    /// their `J` cannot generally be obtained from `Ix + Iy`. Dimensions must be valid.
    /// Published AISC values are available separately through the `aisc` feature.
    pub fn torsional_constant(&self) -> Option<SecondAreaMomentofInertia> {
        let fourth_power = |r: Length| r * r * r * r;
        match *self {
            Self::Rod { radius, .. } => Some(std::f64::consts::PI * fourth_power(radius) / 2.0),
            Self::Pipe {
                outer_radius: r,
                thickness: t,
                ..
            } => {
                let inner = r - t;
                // Factored r^4 - inner^4 avoids subtracting nearly equal fourth powers.
                Some(std::f64::consts::PI / 2.0 * t * (r + inner) * (r * r + inner * inner))
            }
            _ => None,
        }
    }

    /// Product of inertia about the origin, with sign `Ixy = integral(x * y dA)`.
    /// Includes the parallel-axis term `A * cx * cy`.
    pub fn product_moi(&self) -> SecondAreaMomentofInertia {
        match self {
            Self::Angle { .. } => self.open_section_composite().product_moi(),
            Self::Rod { .. }
            | Self::Pipe { .. }
            | Self::Rectangle { .. }
            | Self::IBeam { .. }
            | Self::BoxBeam { .. }
            | Self::Channel { .. }
            | Self::Tee { .. }
            | Self::DoubleAngle { .. } => {
                // Every other supported primitive has a centroidal symmetry axis.
                let (x, y) = self.get_cog();
                self.area() * x * y
            }
        }
    }

    /// All second moments about the current global origin.
    pub fn area_moments(&self) -> AreaMoments {
        AreaMoments::new(self.moi_x(), self.moi_y(), self.product_moi())
    }

    /// All second moments about the centroid, independent of position.
    pub fn centroidal_area_moments(&self) -> AreaMoments {
        let mut centered = *self;
        centered.set_cog((meters(0.0), meters(0.0)));
        centered.area_moments()
    }

    /// Centroidal radii of gyration `(rx, ry) = (sqrt(Ix/A), sqrt(Iy/A))`.
    /// Dimensions must be valid, as for the existing geometric calculations.
    pub fn radii_of_gyration(&self) -> (Length, Length) {
        let moments = self.centroidal_area_moments();
        (
            (moments.ix / self.area()).sqrt(),
            (moments.iy / self.area()).sqrt(),
        )
    }
}

impl CompositeShape {
    /// Signed sum of member products of inertia about the global origin.
    /// Uses `Ixy = integral(x * y dA)` and signed parallel-axis contributions.
    pub fn product_moi(&self) -> SecondAreaMomentofInertia {
        self.shapes
            .iter()
            .map(|(sign, shape)| f64::from(*sign) * shape.product_moi())
            .sum()
    }

    /// All second moments about the current global origin.
    pub fn area_moments(&self) -> AreaMoments {
        AreaMoments::new(self.moi_x(), self.moi_y(), self.product_moi())
    }

    /// Centroid if the net area is finite and positive and its coordinates are finite.
    ///
    /// This checks the resulting sums, not the validity or overlap of individual shapes.
    pub fn try_calculate_cog(&self) -> Option<(Length, Length)> {
        let area = self.area().value;
        if !area.is_finite() || area <= 0.0 {
            return None;
        }
        let (x, y) = self.calculate_cog();
        if x.value.is_finite() && y.value.is_finite() {
            Some((x, y))
        } else {
            None
        }
    }

    /// Moments about the composite centroid, or `None` for an undefined centroid.
    ///
    /// Members are summed as signed areas, not geometric Boolean unions/differences.
    pub fn centroidal_area_moments(&self) -> Option<AreaMoments> {
        let (cx, cy) = self.try_calculate_cog()?;
        let zero = meters(0.0) * meters(0.0) * meters(0.0) * meters(0.0);
        let mut total = AreaMoments::new(zero, zero, zero);
        for (sign, shape) in &self.shapes {
            let mut shifted = *shape;
            let (x, y) = shifted.get_cog();
            shifted.set_cog((x - cx, y - cy));
            let moments = shifted.area_moments();
            total.ix += f64::from(*sign) * moments.ix;
            total.iy += f64::from(*sign) * moments.iy;
            total.ixy += f64::from(*sign) * moments.ixy;
        }
        Some(total)
    }
}
