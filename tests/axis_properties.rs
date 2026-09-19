use structural_shapes::{CompositeShape, StructuralShape};

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1e-12 * expected.abs().max(1.0),
        "actual: {}, expected: {}",
        actual,
        expected
    );
}

#[test]
fn asymmetric_rectangle_and_box_have_distinct_axes() {
    let rectangle = StructuralShape::new_rectangle(4.0, 2.0);
    close(rectangle.moi_x().value, 2.0 * 4.0_f64.powi(3) / 12.0);
    close(rectangle.moi_y().value, 4.0 * 2.0_f64.powi(3) / 12.0);
    let box_beam = StructuralShape::new_boxbeam(4.0, 2.0, 0.25);
    close(
        box_beam.moi_x().value,
        (2.0 * 4.0_f64.powi(3) - 1.5 * 3.5_f64.powi(3)) / 12.0,
    );
    close(
        box_beam.moi_y().value,
        (4.0 * 2.0_f64.powi(3) - 3.5 * 1.5_f64.powi(3)) / 12.0,
    );
}

#[test]
fn ibeam_matches_three_nonoverlapping_rectangles() {
    let (height, width, web, flange) = (6.0_f64, 4.0_f64, 0.5_f64, 0.75_f64);
    let section = StructuralShape::new_ibeam(height, width, web, flange);
    let clear_height = height - 2.0 * flange;
    let flange_offset = (height - flange) / 2.0;
    let ix = web * clear_height.powi(3) / 12.0
        + 2.0 * (width * flange.powi(3) / 12.0 + width * flange * flange_offset.powi(2));
    let iy = clear_height * web.powi(3) / 12.0 + 2.0 * flange * width.powi(3) / 12.0;
    close(
        section.area().value,
        2.0 * width * flange + clear_height * web,
    );
    close(section.moi_x().value, ix);
    close(section.moi_y().value, iy);
    assert!(ix > iy);
}

#[test]
fn all_primitives_obey_parallel_axis_theorem() {
    for shape in [
        StructuralShape::new_rod(0.4),
        StructuralShape::new_pipe(0.4, 0.05),
        StructuralShape::new_rectangle(0.8, 0.3),
        StructuralShape::new_boxbeam(0.8, 0.3, 0.02),
        StructuralShape::new_ibeam(0.8, 0.3, 0.02, 0.03),
        StructuralShape::new_channel(0.8, 0.3, 0.02, 0.03),
        StructuralShape::new_tee(0.8, 0.3, 0.02, 0.03),
        StructuralShape::new_angle(0.8, 0.3, 0.02),
        StructuralShape::new_double_angle(0.8, 0.3, 0.02, 0.015),
    ] {
        for (x, y) in [(2.0, 0.0), (0.0, 3.0), (-2.0, 3.0)] {
            let mut moved = shape;
            moved.with_cog(x, y);
            close(
                moved.moi_x().value,
                shape.moi_x().value + shape.area().value * y * y,
            );
            close(
                moved.moi_y().value,
                shape.moi_y().value + shape.area().value * x * x,
            );
        }
    }
}

#[test]
fn composite_recentering_removes_parallel_axis_terms() {
    let centered = CompositeShape::new()
        .add(StructuralShape::new_rectangle(4.0, 2.0))
        .sub(StructuralShape::new_rectangle(2.0, 1.0));
    let mut translated = CompositeShape::new()
        .add(StructuralShape::new_rectangle(4.0, 2.0).with_cog(3.0, -2.0))
        .sub(StructuralShape::new_rectangle(2.0, 1.0).with_cog(3.0, -2.0));
    close(
        translated.moi_x().value,
        centered.moi_x().value + centered.area().value * 4.0,
    );
    close(
        translated.moi_y().value,
        centered.moi_y().value + centered.area().value * 9.0,
    );
    translated.update_cog();
    close(translated.moi_x().value, centered.moi_x().value);
    close(translated.moi_y().value, centered.moi_y().value);
}
