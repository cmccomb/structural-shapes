#[cfg(test)]
mod tests {
    use structural_shapes::{CompositeShape, StructuralShape};

    #[test]
    fn rod_symmetry() {
        let x = StructuralShape::Rod {
            radius: 1.0,
            center_of_gravity: (0.0, 0.0),
        };
        assert_eq!(x.moi_x(), x.moi_y());
    }

    #[test]
    fn rod_value() {
        let x = StructuralShape::Rod {
            radius: 1.0,
            center_of_gravity: (0.0, 0.0),
        };
        assert_eq!(x.moi_x(), std::f64::consts::PI / 4.0);
    }

    #[test]
    fn pipe_symmetry() {
        let x = StructuralShape::Pipe {
            outer_radius: 1.0,
            thickness: 0.01,
            center_of_gravity: (0.0, 0.0),
        };
        assert_eq!(x.moi_x(), x.moi_y());
    }

    #[test]
    fn pipe_value() {
        let x = StructuralShape::Pipe {
            outer_radius: 2.0,
            thickness: 1.0,
            center_of_gravity: (0.0, 0.0),
        };
        assert_eq!(x.moi_x(), std::f64::consts::PI * 15.0 / 4.0);
    }

    #[test]
    fn rectangle_symmetry() {
        let x = StructuralShape::Rectangle {
            width: 2.0,
            height: 2.0,
            center_of_gravity: (0.0, 0.0),
        };
        assert_eq!(x.moi_x(), x.moi_y());
    }

    #[test]
    fn rectangle_value() {
        let x = StructuralShape::Rectangle {
            width: 2.0,
            height: 2.0,
            center_of_gravity: (0.0, 0.0),
        };
        assert_eq!(x.moi_x(), 16.0 / 12.0);
    }

    #[test]
    fn boxbeam_symmetry() {
        let x = StructuralShape::BoxBeam {
            width: 3.0,
            height: 3.0,
            thickness: 1.0,
            center_of_gravity: (0.0, 0.0),
        };
        assert_eq!(x.moi_x(), x.moi_y());
    }

    #[test]
    fn boxbeam_value() {
        let x = StructuralShape::BoxBeam {
            width: 3.0,
            height: 3.0,
            thickness: 1.0,
            center_of_gravity: (0.0, 0.0),
        };
        assert_eq!(x.moi_x(), 80.0 / 12.0);
    }

    #[test]
    fn ibeam_symmetry() {
        let x = StructuralShape::IBeam {
            width: 2.0,
            height: 2.0,
            flange_thickness: 1.0,
            web_thickness: 1.0,
            center_of_gravity: (0.0, 0.0),
        };
        assert_eq!(x.moi_x(), x.moi_y());
    }

    #[test]
    fn ibeam_value() {
        let x = StructuralShape::IBeam {
            width: 2.0,
            height: 2.0,
            flange_thickness: 1.0,
            web_thickness: 1.0,
            center_of_gravity: (0.0, 0.0),
        };
        let y = StructuralShape::Rectangle {
            width: 2.0,
            height: 2.0,
            center_of_gravity: (0.0, 0.0),
        };
        assert_eq!(x.moi_x(), y.moi_x());
        assert_eq!(x.moi_y(), y.moi_y());
    }

    #[test]
    fn composite_chain() {
        let x = CompositeShape::new()
            .add(StructuralShape::Rod {
                radius: 2.0,
                center_of_gravity: (3.0, 0.0),
            })
            .add(StructuralShape::Rod {
                radius: 2.0,
                center_of_gravity: (-3.0, 0.0),
            });
        println!("moi-x: {}", x.moi_x());
        println!("moi-y: {}", x.moi_y());
        println!("area: {}", x.area());
    }

    #[test]
    fn composite_nonchained() {
        let mut x = CompositeShape::new();
        x.add(StructuralShape::Rod {
            radius: 2.0,
            center_of_gravity: (3.0, 0.0),
        });
        x.add(StructuralShape::Rod {
            radius: 2.0,
            center_of_gravity: (-3.0, 0.0),
        });
        println!("moi-x: {}", x.moi_x());
        println!("moi-y: {}", x.moi_y());
        println!("area: {}", x.area());
    }

    #[test]
    fn composite_cog_calculation() {
        let mut x = CompositeShape::new()
            .add(StructuralShape::Rectangle {
                width: 1.0,
                height: 1.0,
                center_of_gravity: (2.0, 1.5),
            })
            .sub(StructuralShape::Rectangle {
                width: 0.9,
                height: 0.9,
                center_of_gravity: (2.0, 1.5),
            });
        assert_eq!(x.calculate_cog(), (2.0, 1.5));
        x.update_cog();
        assert_eq!(x.calculate_cog(), (0.0, 0.0));
    }
}
