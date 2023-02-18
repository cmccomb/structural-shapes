#[cfg(test)]
mod tests {
    use structural_shapes::{meters, CompositeShape, StructuralShape};

    #[test]
    fn rod_symmetry() {
        let x = StructuralShape::Rod {
            radius: meters(1.0),
            center_of_gravity: (meters(0.0), meters(0.0)),
        };
        assert_eq!(x.moi_x(), x.moi_y());
    }

    #[test]
    fn rod_value() {
        let x = StructuralShape::Rod {
            radius: meters(1.0),
            center_of_gravity: (meters(0.0), meters(0.0)),
        };
        assert_eq!(x.moi_x().value, std::f64::consts::PI / 4.0);
    }

    #[test]
    fn pipe_symmetry() {
        let x = StructuralShape::Pipe {
            outer_radius: meters(1.0),
            thickness: meters(0.01),
            center_of_gravity: (meters(0.0), meters(0.0)),
        };
        assert_eq!(x.moi_x(), x.moi_y());
    }

    #[test]
    fn pipe_value() {
        let x = StructuralShape::Pipe {
            outer_radius: meters(2.0),
            thickness: meters(1.0),
            center_of_gravity: (meters(0.0), meters(0.0)),
        };
        assert_eq!(x.moi_x().value, std::f64::consts::PI * 15.0 / 4.0);
    }

    #[test]
    fn rectangle_symmetry() {
        let x = StructuralShape::Rectangle {
            width: meters(2.0),
            height: meters(2.0),
            center_of_gravity: (meters(0.0), meters(0.0)),
        };
        assert_eq!(x.moi_x(), x.moi_y());
    }

    #[test]
    fn rectangle_value() {
        let x = StructuralShape::Rectangle {
            width: meters(2.0),
            height: meters(2.0),
            center_of_gravity: (meters(0.0), meters(0.0)),
        };
        assert_eq!(x.moi_x().value, 16.0 / 12.0);
    }

    #[test]
    fn boxbeam_symmetry() {
        let x = StructuralShape::BoxBeam {
            width: meters(3.0),
            height: meters(3.0),
            thickness: meters(1.0),
            center_of_gravity: (meters(0.0), meters(0.0)),
        };
        assert_eq!(x.moi_x(), x.moi_y());
    }

    #[test]
    fn boxbeam_value() {
        let x = StructuralShape::BoxBeam {
            width: meters(3.0),
            height: meters(3.0),
            thickness: meters(1.0),
            center_of_gravity: (meters(0.0), meters(0.0)),
        };
        assert_eq!(x.moi_x().value, 80.0 / 12.0);
    }

    #[test]
    fn ibeam_symmetry() {
        let x = StructuralShape::IBeam {
            width: meters(2.0),
            height: meters(2.0),
            flange_thickness: meters(1.0),
            web_thickness: meters(1.0),
            center_of_gravity: (meters(0.0), meters(0.0)),
        };
        assert_eq!(x.moi_x(), x.moi_y());
    }

    #[test]
    fn ibeam_value() {
        let x = StructuralShape::IBeam {
            width: meters(2.0),
            height: meters(2.0),
            flange_thickness: meters(1.0),
            web_thickness: meters(1.0),
            center_of_gravity: (meters(0.0), meters(0.0)),
        };
        let y = StructuralShape::Rectangle {
            width: meters(2.0),
            height: meters(2.0),
            center_of_gravity:  (meters(0.0), meters(0.0)),
        };
        assert_eq!(x.moi_x(), y.moi_x());
        assert_eq!(x.moi_y(), y.moi_y());
    }

    #[test]
    fn composite_chain() {
        let x = CompositeShape::new()
            .add(StructuralShape::Rod {
                radius: meters(2.0),
                center_of_gravity: (meters(3.0), meters(0.0)),
            })
            .add(StructuralShape::Rod {
                radius: meters(2.0),
                center_of_gravity:  (meters(-3.0), meters(0.0)),
            });
        println!("moi-x: {}", x.moi_x().value);
        println!("moi-y: {}", x.moi_y().value);
        println!("moi-z: {}", x.polar_moi().value);
        println!("area: {}", x.area().value);
    }

    #[test]
    fn composite_nonchained() {
        let mut x = CompositeShape::new();
        x.add(StructuralShape::Rod {
            radius: meters(2.0),
            center_of_gravity:  (meters(3.0), meters(0.0)),
        });
        x.add(StructuralShape::Rod {
            radius: meters(2.0),
            center_of_gravity:  (meters(-3.0), meters(0.0)),
        });
        println!("moi-x: {}", x.moi_x().value);
        println!("moi-y: {}", x.moi_y().value);
        println!("moi-z: {}", x.polar_moi().value);
        println!("area: {}", x.area().value);
    }

    #[test]
    fn composite_cog_calculation() {
        let mut x = CompositeShape::new()
            .add(StructuralShape::Rectangle {
                width: meters(1.0),
                height: meters(1.0),
                center_of_gravity:  (meters(2.0), meters(1.5)),
            })
            .sub(StructuralShape::Rectangle {
                width: meters(0.9),
                height: meters(0.9),
                center_of_gravity: (meters(2.0), meters(1.5)),
            });
        assert_eq!(x.calculate_cog(), (meters(2.0), meters(1.5)));
        x.update_cog();
        assert_eq!(x.calculate_cog(), (meters(0.0), meters(0.0)),);
    }
}
