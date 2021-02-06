#[cfg(test)]
mod tests {
    use structural_shapes::StructuralShape;

    #[test]
    fn rod() {
        let x = StructuralShape::Rod { radius: 1.0 };
        x.moi_x();
        x.moi_y();
        x.moi_x_d(2.0);
        x.moi_y_d(2.0);
    }

    #[test]
    fn pipe() {
        let x = StructuralShape::Pipe {
            outer_radius: 1.0,
            thickness: 0.01,
        };
        x.moi_x();
        x.moi_y();
        x.moi_x_d(2.0);
        x.moi_y_d(2.0);
    }

    #[test]
    fn rectangle() {
        let x = StructuralShape::Rectangle {
            width: 2.0,
            height: 2.0,
        };
        x.moi_x();
        x.moi_y();
        x.moi_x_d(2.0);
        x.moi_y_d(2.0);
    }

    #[test]
    fn boxbeam() {
        let x = StructuralShape::BoxBeam {
            width: 2.0,
            height: 2.0,
            thickness: 0.05,
        };
        x.moi_x();
        x.moi_y();
        x.moi_x_d(2.0);
        x.moi_y_d(2.0);
    }

    #[test]
    fn ibeam() {
        let x = StructuralShape::IBeam {
            width: 2.0,
            height: 2.0,
            flange_thickness: 0.05,
            web_thickness: 0.05,
        };
        x.moi_x();
        x.moi_y();
        x.moi_x_d(2.0);
        x.moi_y_d(2.0);
    }
}
