//! Parametric open sections evaluated through nonoverlapping rectangles.

use crate::{meters, CompositeShape, StructuralShape};
use uom::si::f64::Length;

impl StructuralShape {
    /// Create a centered sharp-corner channel. Dimensions are in meters.
    ///
    /// The web is vertical on the left; flanges point right. Dimensions must be
    /// positive, `web_thickness < width`, and `2 * flange_thickness < height`.
    /// Fillets and flange taper are not modeled.
    pub fn new_channel(height: f64, width: f64, web_thickness: f64, flange_thickness: f64) -> Self {
        Self::Channel {
            height: meters(height),
            width: meters(width),
            web_thickness: meters(web_thickness),
            flange_thickness: meters(flange_thickness),
            center_of_gravity: (meters(0.0), meters(0.0)),
        }
    }

    /// Create a centered sharp-corner tee. Dimensions are in meters.
    ///
    /// The flange is at the top, with the stem below it. Dimensions must be
    /// positive, `web_thickness < width`, and `flange_thickness < height`.
    /// Fillets and flange taper are not modeled.
    pub fn new_tee(height: f64, width: f64, web_thickness: f64, flange_thickness: f64) -> Self {
        Self::Tee {
            height: meters(height),
            width: meters(width),
            web_thickness: meters(web_thickness),
            flange_thickness: meters(flange_thickness),
            center_of_gravity: (meters(0.0), meters(0.0)),
        }
    }

    /// Create a centered sharp-corner angle. Dimensions are in meters.
    ///
    /// The vertical leg is on the left and the bottom leg points right. Thickness
    /// must be positive and smaller than both overall leg dimensions. The x/y
    /// axes are horizontal/vertical, not the angle's principal axes.
    ///
    /// ```
    /// use structural_shapes::StructuralShape;
    /// let angle = StructuralShape::new_angle(0.08, 0.04, 0.005);
    /// assert!(angle.moi_x() > angle.moi_y());
    /// ```
    pub fn new_angle(height: f64, width: f64, thickness: f64) -> Self {
        Self::Angle {
            height: meters(height),
            width: meters(width),
            thickness: meters(thickness),
            center_of_gravity: (meters(0.0), meters(0.0)),
        }
    }

    /// Create two mirrored, centered angles, with vertical legs back-to-back.
    ///
    /// All dimensions are in meters. `width` is the overall horizontal leg width
    /// of each angle. `gap` is the nonnegative clear spacing between the backs;
    /// total width is `2 * width + gap`. Bottom legs point outward. Thickness
    /// must be positive and smaller than both leg dimensions.
    pub fn new_double_angle(height: f64, width: f64, thickness: f64, gap: f64) -> Self {
        Self::DoubleAngle {
            height: meters(height),
            width: meters(width),
            thickness: meters(thickness),
            gap: meters(gap),
            center_of_gravity: (meters(0.0), meters(0.0)),
        }
    }

    /// Decompose an open section, recenter its geometric centroid, then translate it.
    /// Called only for the four variants handled below; leaves are rectangles.
    pub(super) fn open_section_composite(&self) -> CompositeShape {
        let mut composite = CompositeShape::new();
        let mut rectangle = |height: Length, width: Length, x: Length, y: Length| {
            composite.shapes.push((
                1,
                Self::Rectangle {
                    height,
                    width,
                    center_of_gravity: (x, y),
                },
            ));
        };
        match *self {
            Self::Channel {
                height,
                width,
                web_thickness: tw,
                flange_thickness: tf,
                ..
            } => {
                rectangle(height, tw, tw / 2.0, height / 2.0);
                rectangle(tf, width - tw, (width + tw) / 2.0, tf / 2.0);
                rectangle(tf, width - tw, (width + tw) / 2.0, height - tf / 2.0);
            }
            Self::Tee {
                height,
                width,
                web_thickness: tw,
                flange_thickness: tf,
                ..
            } => {
                rectangle(tf, width, meters(0.0), height - tf / 2.0);
                rectangle(height - tf, tw, meters(0.0), (height - tf) / 2.0);
            }
            Self::Angle {
                height,
                width,
                thickness: t,
                ..
            } => {
                rectangle(height, t, t / 2.0, height / 2.0);
                rectangle(t, width - t, (width + t) / 2.0, t / 2.0);
            }
            Self::DoubleAngle {
                height,
                width,
                thickness: t,
                gap,
                ..
            } => {
                for side in [-1.0, 1.0] {
                    rectangle(height, t, side * (gap + t) / 2.0, height / 2.0);
                    rectangle(t, width - t, side * (gap + width + t) / 2.0, t / 2.0);
                }
            }
            _ => unreachable!("open_section_composite requires an open-section variant"),
        }
        composite.update_cog();
        let (x, y) = self.get_cog();
        for (_, shape) in &mut composite.shapes {
            let (cx, cy) = shape.get_cog();
            shape.set_cog((cx + x, cy + y));
        }
        composite
    }
}
