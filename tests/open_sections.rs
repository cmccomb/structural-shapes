use structural_shapes::StructuralShape;

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1e-11 * expected.abs().max(1.0),
        "{} != {}",
        actual,
        expected
    );
}

// Independent boundary integration, rather than the implementation's rectangle decomposition.
fn polygon_properties(vertices: &[(f64, f64)]) -> (f64, f64, f64, f64) {
    let (mut twice_area, mut cx_numerator, mut cy_numerator, mut ix12, mut iy12) =
        (0.0, 0.0, 0.0, 0.0, 0.0);
    for (i, &(x, y)) in vertices.iter().enumerate() {
        let (xx, yy) = vertices[(i + 1) % vertices.len()];
        let cross = x * yy - xx * y;
        twice_area += cross;
        cx_numerator += (x + xx) * cross;
        cy_numerator += (y + yy) * cross;
        ix12 += (y * y + y * yy + yy * yy) * cross;
        iy12 += (x * x + x * xx + xx * xx) * cross;
    }
    let area = twice_area / 2.0;
    let cx = cx_numerator / (6.0 * area);
    let cy = cy_numerator / (6.0 * area);
    (
        area,
        ix12 / 12.0 - area * cy * cy,
        iy12 / 12.0 - area * cx * cx,
        cx,
    )
}

#[test]
fn open_sections_match_independent_polygon_integrals() {
    let cases = [
        (
            StructuralShape::new_channel(8.0, 4.0, 0.5, 0.75),
            vec![
                (0.0, 0.0),
                (4.0, 0.0),
                (4.0, 0.75),
                (0.5, 0.75),
                (0.5, 7.25),
                (4.0, 7.25),
                (4.0, 8.0),
                (0.0, 8.0),
            ],
        ),
        (
            StructuralShape::new_tee(8.0, 4.0, 0.5, 0.75),
            vec![
                (0.0, 7.25),
                (1.75, 7.25),
                (1.75, 0.0),
                (2.25, 0.0),
                (2.25, 7.25),
                (4.0, 7.25),
                (4.0, 8.0),
                (0.0, 8.0),
            ],
        ),
        (
            StructuralShape::new_angle(8.0, 4.0, 0.5),
            vec![
                (0.0, 0.0),
                (4.0, 0.0),
                (4.0, 0.5),
                (0.5, 0.5),
                (0.5, 8.0),
                (0.0, 8.0),
            ],
        ),
    ];
    for (shape, boundary) in cases {
        let (area, ix, iy, _) = polygon_properties(&boundary);
        close(shape.area().value, area);
        close(shape.moi_x().value, ix);
        close(shape.moi_y().value, iy);
    }
}

#[test]
fn mirrored_double_angles_obey_parallel_axis_theorem_for_each_gap() {
    let boundary = [
        (0.0, 0.0),
        (4.0, 0.0),
        (4.0, 0.5),
        (0.5, 0.5),
        (0.5, 8.0),
        (0.0, 8.0),
    ];
    let (area, ix, iy, cx) = polygon_properties(&boundary);
    for gap in [0.0, 0.375, 0.75, 1.5] {
        let shape = StructuralShape::new_double_angle(8.0, 4.0, 0.5, gap);
        close(shape.area().value, 2.0 * area);
        close(shape.moi_x().value, 2.0 * ix);
        close(
            shape.moi_y().value,
            2.0 * (iy + area * (cx + gap / 2.0).powi(2)),
        );
    }
}

#[test]
fn rotating_an_angle_swaps_its_geometric_axis_moments() {
    let long_vertical = StructuralShape::new_angle(8.0, 4.0, 0.5);
    let short_vertical = StructuralShape::new_angle(4.0, 8.0, 0.5);
    close(long_vertical.moi_x().value, short_vertical.moi_y().value);
    close(long_vertical.moi_y().value, short_vertical.moi_x().value);
}
