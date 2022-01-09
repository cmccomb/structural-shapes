#[cfg(test)]
mod tests {
    use structural_shapes::{length, point, CompositeShape, StructuralShape};

    #[test]
    fn rod_symmetry() {
        let x = StructuralShape::Rod {
            radius: length(1.0),
            center_of_gravity: point(0.0, 0.0),
        };
        assert_eq!(x.moi_x(), x.moi_y());
    }

    #[test]
    fn rod_value() {
        let x = StructuralShape::Rod {
            radius: length(1.0),
            center_of_gravity: point(0.0, 0.0),
        };
        assert_eq!(x.moi_x().value, std::f64::consts::PI / 4.0);
    }

    #[test]
    fn pipe_symmetry() {
        let x = StructuralShape::Pipe {
            outer_radius: length(1.0),
            thickness: length(0.01),
            center_of_gravity: point(0.0, 0.0),
        };
        assert_eq!(x.moi_x(), x.moi_y());
    }

    #[test]
    fn pipe_value() {
        let x = StructuralShape::Pipe {
            outer_radius: length(2.0),
            thickness: length(1.0),
            center_of_gravity: point(0.0, 0.0),
        };
        assert_eq!(x.moi_x().value, std::f64::consts::PI * 15.0 / 4.0);
    }

    #[test]
    fn rectangle_symmetry() {
        let x = StructuralShape::Rectangle {
            width: length(2.0),
            height: length(2.0),
            center_of_gravity: point(0.0, 0.0),
        };
        assert_eq!(x.moi_x(), x.moi_y());
    }

    #[test]
    fn rectangle_value() {
        let x = StructuralShape::Rectangle {
            width: length(2.0),
            height: length(2.0),
            center_of_gravity: point(0.0, 0.0),
        };
        assert_eq!(x.moi_x().value, 16.0 / 12.0);
    }

    #[test]
    fn boxbeam_symmetry() {
        let x = StructuralShape::BoxBeam {
            width: length(3.0),
            height: length(3.0),
            thickness: length(1.0),
            center_of_gravity: point(0.0, 0.0),
        };
        assert_eq!(x.moi_x(), x.moi_y());
    }

    #[test]
    fn boxbeam_value() {
        let x = StructuralShape::BoxBeam {
            width: length(3.0),
            height: length(3.0),
            thickness: length(1.0),
            center_of_gravity: point(0.0, 0.0),
        };
        assert_eq!(x.moi_x().value, 80.0 / 12.0);
    }

    #[test]
    fn ibeam_symmetry() {
        let x = StructuralShape::IBeam {
            width: length(2.0),
            height: length(2.0),
            flange_thickness: length(1.0),
            web_thickness: length(1.0),
            center_of_gravity: point(0.0, 0.0),
        };
        assert_eq!(x.moi_x(), x.moi_y());
    }

    #[test]
    fn ibeam_value() {
        let x = StructuralShape::IBeam {
            width: length(2.0),
            height: length(2.0),
            flange_thickness: length(1.0),
            web_thickness: length(1.0),
            center_of_gravity: point(0.0, 0.0),
        };
        let y = StructuralShape::Rectangle {
            width: length(2.0),
            height: length(2.0),
            center_of_gravity: point(0.0, 0.0),
        };
        assert_eq!(x.moi_x(), y.moi_x());
        assert_eq!(x.moi_y(), y.moi_y());
    }

    #[test]
    fn composite_chain() {
        let x = CompositeShape::new()
            .add(StructuralShape::Rod {
                radius: length(2.0),
                center_of_gravity: point(3.0, 0.0),
            })
            .add(StructuralShape::Rod {
                radius: length(2.0),
                center_of_gravity: point(-3.0, 0.0),
            });
        println!("moi-x: {}", x.moi_x().value);
        println!("moi-y: {}", x.moi_y().value);
        println!("area: {}", x.area().value);
    }

    #[test]
    fn composite_nonchained() {
        let mut x = CompositeShape::new();
        x.add(StructuralShape::Rod {
            radius: length(2.0),
            center_of_gravity: point(3.0, 0.0),
        });
        x.add(StructuralShape::Rod {
            radius: length(2.0),
            center_of_gravity: point(-3.0, 0.0),
        });
        println!("moi-x: {}", x.moi_x().value);
        println!("moi-y: {}", x.moi_y().value);
        println!("area: {}", x.area().value);
    }

    #[test]
    fn composite_cog_calculation() {
        let mut x = CompositeShape::new()
            .add(StructuralShape::Rectangle {
                width: length(1.0),
                height: length(1.0),
                center_of_gravity: point(2.0, 1.5),
            })
            .sub(StructuralShape::Rectangle {
                width: length(0.9),
                height: length(0.9),
                center_of_gravity: point(2.0, 1.5),
            });
        assert_eq!(x.calculate_cog(), point(2.0, 1.5));
        x.update_cog();
        assert_eq!(x.calculate_cog(), point(0.0, 0.0));
    }
}
