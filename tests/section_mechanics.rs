use structural_shapes::{CompositeShape, StructuralShape};
use uom::si::{angle::degree, f64::Angle};

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1e-12 * expected.abs().max(1.0),
        "{} != {}",
        actual,
        expected
    );
}

#[test]
fn circular_torsion_is_centroidal_and_translation_invariant() {
    for (mut shape, j) in [
        (StructuralShape::new_rod(2.0), std::f64::consts::PI * 8.0),
        (
            StructuralShape::new_pipe(2.0, 0.5),
            std::f64::consts::PI / 2.0 * (16.0 - 1.5_f64.powi(4)),
        ),
    ] {
        close(shape.torsional_constant().unwrap().value, j);
        close(shape.polar_moi().value, j);
        shape.with_cog(3.0, -4.0);
        close(shape.torsional_constant().unwrap().value, j);
        close(shape.centroidal_area_moments().polar_moi().value, j);
        close(shape.polar_moi().value, j + shape.area().value * 25.0);
        close(shape.product_moi().value, -12.0 * shape.area().value);
    }
}

#[test]
fn noncircular_shapes_do_not_substitute_polar_moment_for_j() {
    for shape in [
        StructuralShape::new_rectangle(2.0, 3.0),
        StructuralShape::new_boxbeam(4.0, 3.0, 0.25),
        StructuralShape::new_ibeam(4.0, 3.0, 0.25, 0.5),
        StructuralShape::new_channel(4.0, 3.0, 0.25, 0.5),
        StructuralShape::new_tee(4.0, 3.0, 0.25, 0.5),
        StructuralShape::new_angle(4.0, 3.0, 0.25),
        StructuralShape::new_double_angle(4.0, 3.0, 0.25, 0.5),
    ] {
        assert_eq!(shape.torsional_constant(), None);
        assert!(shape.polar_moi().value > 0.0);
    }
}

#[test]
fn rotated_axes_follow_the_signed_product_convention() {
    let moments = StructuralShape::new_rectangle(6.0, 4.0).area_moments();
    let rotated = moments.rotated(Angle::new::<degree>(45.0));
    close(rotated.moi_x().value, 52.0);
    close(rotated.moi_y().value, 52.0);
    close(rotated.product_moi().value, 20.0);
    for degrees in [-135.0, -30.0, 0.0, 27.0, 90.0, 180.0] {
        let angle = Angle::new::<degree>(degrees);
        let r = moments.rotated(angle);
        close(r.polar_moi().value, moments.polar_moi().value);
        close(
            r.moi_x().value * r.moi_y().value - r.product_moi().value.powi(2),
            72.0 * 32.0,
        );
        let inverse = r.rotated(-angle);
        close(inverse.moi_x().value, 72.0);
        close(inverse.moi_y().value, 32.0);
        close(inverse.product_moi().value, 0.0);
    }
}

#[test]
fn principal_axes_diagonalize_angles_and_handle_symmetry() {
    for shape in [
        StructuralShape::new_angle(8.0, 4.0, 0.5),
        StructuralShape::new_angle(4.0, 4.0, 0.5),
        StructuralShape::new_rectangle(2.0, 4.0),
    ] {
        let m = shape.centroidal_area_moments();
        let (major, minor) = m.principal_moments();
        let diagonal = m.rotated(m.principal_axis_angle().unwrap());
        assert!(major >= minor && minor.value > 0.0);
        close(diagonal.product_moi().value, 0.0);
        close(diagonal.moi_x().value, major.value);
        close(diagonal.moi_y().value, minor.value);
    }
    let equal_angle = StructuralShape::new_angle(4.0, 4.0, 0.5).area_moments();
    close(
        equal_angle.principal_axis_angle().unwrap().get::<degree>(),
        45.0,
    );
    let circle = StructuralShape::new_rod(1.0).area_moments();
    assert_eq!(circle.principal_axis_angle(), None);
    assert_eq!(circle.principal_moments().0, circle.principal_moments().1);
}

#[test]
fn signed_composite_moments_obey_parallel_axis_theorem() {
    let make = |x, y| {
        CompositeShape::new()
            .add(StructuralShape::new_rectangle(6.0, 4.0).with_cog(x, y))
            .sub(StructuralShape::new_rectangle(2.0, 1.0).with_cog(x + 1.0, y + 2.0))
    };
    let composite = make(0.0, 0.0);
    close(composite.product_moi().value, -4.0);
    let (cx, cy) = composite.try_calculate_cog().unwrap();
    close(cx.value, -1.0 / 11.0);
    close(cy.value, -2.0 / 11.0);
    let central = composite.centroidal_area_moments().unwrap();
    close(central.product_moi().value, -48.0 / 11.0);
    close(
        central.moi_x().value,
        composite.moi_x().value - 22.0 * cy.value.powi(2),
    );
    close(
        central.moi_y().value,
        composite.moi_y().value - 22.0 * cx.value.powi(2),
    );
    let translated = make(20.0, -10.0).centroidal_area_moments().unwrap();
    close(translated.moi_x().value, central.moi_x().value);
    close(translated.moi_y().value, central.moi_y().value);
    close(translated.product_moi().value, central.product_moi().value);
}

#[test]
fn checked_centroid_rejects_empty_cancelled_negative_and_nonfinite_areas() {
    let mut shape = StructuralShape::new_rectangle(1.0, 1.0);
    for composite in [
        CompositeShape::new(),
        CompositeShape::new().add(shape).sub(shape),
        CompositeShape::new().sub(shape),
        CompositeShape::new().add(StructuralShape::new_rectangle(f64::NAN, 1.0)),
        CompositeShape::new().add(StructuralShape::new_rectangle(f64::INFINITY, 1.0)),
        CompositeShape::new().add(shape.with_cog(f64::INFINITY, 0.0)),
    ] {
        assert_eq!(composite.try_calculate_cog(), None);
        assert_eq!(composite.centroidal_area_moments(), None);
    }
}

#[test]
fn geometric_radii_are_centroidal() {
    let mut shape = StructuralShape::new_rectangle(6.0, 4.0);
    shape.with_cog(100.0, -200.0);
    let (rx, ry) = shape.radii_of_gyration();
    close(rx.value, 6.0 / 12.0_f64.sqrt());
    close(ry.value, 4.0 / 12.0_f64.sqrt());
}
