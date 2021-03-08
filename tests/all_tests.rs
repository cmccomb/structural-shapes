#[cfg(test)]
mod tests {
    use structural_shapes::{CompositeShape, StructuralShape};

    #[test]
    fn rod() {
        let x = StructuralShape::Rod {
            radius: 1.0,
            center_of_gravity: (0.0, 0.0),
        };
        x.moi_x();
        x.moi_y();
    }

    #[test]
    fn pipe() {
        let x = StructuralShape::Pipe {
            outer_radius: 1.0,
            thickness: 0.01,
            center_of_gravity: (0.0, 0.0),
        };
        x.moi_x();
        x.moi_y();
    }

    #[test]
    fn rectangle() {
        let x = StructuralShape::Rectangle {
            width: 2.0,
            height: 2.0,
            center_of_gravity: (0.0, 0.0),
        };
        x.moi_x();
        x.moi_y();
    }

    #[test]
    fn boxbeam() {
        let x = StructuralShape::BoxBeam {
            width: 2.0,
            height: 2.0,
            thickness: 0.05,
            center_of_gravity: (0.0, 0.0),
        };
        x.moi_x();
        x.moi_y();
    }

    #[test]
    fn ibeam() {
        let x = StructuralShape::IBeam {
            width: 2.0,
            height: 2.0,
            flange_thickness: 0.05,
            web_thickness: 0.05,
            center_of_gravity: (0.0, 0.0),
        };
        x.moi_x();
        x.moi_y();
    }

    #[test]
    fn composite() {
        let mut x = CompositeShape::default();
        x.add(StructuralShape::Rod {
            radius: 2.0,
            center_of_gravity: (2.0, 0.0),
        });
        x.add(StructuralShape::Rod {
            radius: 2.0,
            center_of_gravity: (-2.0, 0.0),
        });
        x.moi_y();
        x.moi_x();
        x.area();
    }
}
