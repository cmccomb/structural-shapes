#![cfg(feature = "aisc")]

use std::collections::{HashMap, HashSet};
use structural_shapes::{AiscSection, SectionDimensions, SectionFamily, StructuralShape};
use uom::si::{area::square_inch, length::inch};

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= expected.abs().max(1e-12) * 1e-12,
        "actual: {}, expected: {}",
        actual,
        expected
    );
}

#[test]
fn published_reference_values_and_units() {
    let beam = AiscSection::W12X26;
    close(beam.area().get::<square_inch>(), 7.65);
    close(beam.moi_x().value, 204.0 * 0.0254_f64.powi(4));
    close(beam.moi_y().value, 17.3 * 0.0254_f64.powi(4));
    close(
        beam.torsional_constant().unwrap().value,
        0.3 * 0.0254_f64.powi(4),
    );
    close(beam.mass_per_length().value, 26.0 * 0.45359237 / 0.3048);
    close(
        beam.polar_moi().value,
        beam.moi_x().value + beam.moi_y().value,
    );
    assert!(beam.polar_moi() > beam.torsional_constant().unwrap());
    close(
        beam.elastic_section_modulus_x().value,
        33.4 * 0.0254_f64.powi(3),
    );
    close(
        beam.elastic_section_modulus_y().value,
        5.34 * 0.0254_f64.powi(3),
    );
    close(
        beam.plastic_section_modulus_x().value,
        37.2 * 0.0254_f64.powi(3),
    );
    close(
        beam.plastic_section_modulus_y().value,
        8.17 * 0.0254_f64.powi(3),
    );
    close(beam.radius_of_gyration_x().get::<inch>(), 5.17);
    close(beam.radius_of_gyration_y().get::<inch>(), 1.51);
    close(
        beam.warping_constant().unwrap().value,
        607.0 * 0.0254_f64.powi(6),
    );
    close(
        beam.effective_radius_of_gyration().unwrap().get::<inch>(),
        1.75,
    );
    close(beam.flange_centroid_distance().unwrap().get::<inch>(), 11.8);
    assert_eq!(beam.shear_center_distance(), None);
    assert_eq!(beam.torsional_section_modulus(), None);

    let square = AiscSection::HSS6X6X1_4;
    close(square.area().get::<square_inch>(), 5.24);
    close(square.moi_x().value, 28.6 * 0.0254_f64.powi(4));
    assert_eq!(square.moi_x(), square.moi_y());
    close(
        square.torsional_section_modulus().unwrap().value,
        15.4 * 0.0254_f64.powi(3),
    );
    assert_eq!(square.warping_constant(), None);
    close(
        AiscSection::C12X20_7
            .shear_center_distance()
            .unwrap()
            .get::<inch>(),
        0.87,
    );
    let rectangle = AiscSection::HSS8X4X1_4;
    close(rectangle.moi_x().value, 42.5 * 0.0254_f64.powi(4));
    close(rectangle.moi_y().value, 14.4 * 0.0254_f64.powi(4));
    let round = AiscSection::HSS6_625X0_280;
    close(round.area().get::<square_inch>(), 5.2);
    close(round.moi_x().value, 26.4 * 0.0254_f64.powi(4));
}

#[test]
fn added_properties_preserve_every_source_value_and_missing_cell() {
    let mut lines = include_str!("../data/aisc-v16.csv").lines();
    let headers: Vec<_> = lines.next().unwrap().split(',').collect();
    let mut counts = [0; 5];
    for line in lines {
        let columns: Vec<_> = line.split(',').collect();
        let section: AiscSection = columns[2].parse().unwrap();
        let source = |key: &str| {
            let text = columns[headers.iter().position(|h| *h == key).unwrap()];
            if text.is_empty() {
                None
            } else {
                Some(text.parse::<f64>().unwrap())
            }
        };
        for (key, property, exponent) in [
            ("Sx", section.elastic_section_modulus_x().value, 3),
            ("Sy", section.elastic_section_modulus_y().value, 3),
            ("Zx", section.plastic_section_modulus_x().value, 3),
            ("Zy", section.plastic_section_modulus_y().value, 3),
            ("rx", section.radius_of_gyration_x().value, 1),
            ("ry", section.radius_of_gyration_y().value, 1),
        ] {
            close(property, source(key).unwrap() * 0.0254_f64.powi(exponent));
        }
        for (i, (key, property, exponent)) in [
            ("Cw", section.warping_constant().map(|v| v.value), 6),
            ("C", section.torsional_section_modulus().map(|v| v.value), 3),
            ("eo", section.shear_center_distance().map(|v| v.value), 1),
            (
                "rts",
                section.effective_radius_of_gyration().map(|v| v.value),
                1,
            ),
            ("ho", section.flange_centroid_distance().map(|v| v.value), 1),
        ]
        .iter()
        .enumerate()
        {
            match (property, source(key)) {
                (Some(actual), Some(expected)) => {
                    close(*actual, expected * 0.0254_f64.powi(*exponent));
                    counts[i] += 1;
                }
                (None, None) => {}
                _ => panic!("availability mismatch for {} {}", section, key),
            }
        }
        let moments = section.area_moments();
        assert_eq!(moments.product_moi(), section.product_moi());
        if section.family() == SectionFamily::Angle {
            assert!(section.product_moi().value < 0.0);
            let (major, minor) = moments.principal_moments();
            // Independent eigenvalue check against rounded published Iw and Iz.
            // Source fields are rounded independently, so exact equality is not expected.
            for (computed, key) in [(major.value, "Iw"), (minor.value, "Iz")] {
                let expected = source(key).unwrap() * 0.0254_f64.powi(4);
                assert!(
                    (computed / expected - 1.0).abs() < 0.02,
                    "{} {}",
                    section,
                    key
                );
            }
        } else {
            close(section.product_moi().value, 0.0);
        }
    }
    assert_eq!(counts, [895, 714, 72, 427, 427]);
}

#[test]
fn single_angle_products_have_explicit_orientation_and_equal_leg_limit() {
    let equal = AiscSection::L4X4X1_2;
    close(equal.product_moi().value, -3.27 * 0.0254_f64.powi(4));
    let (major, minor) = equal.area_moments().principal_moments();
    close(major.value, 8.79 * 0.0254_f64.powi(4));
    close(minor.value, 2.25 * 0.0254_f64.powi(4));
    close(
        equal
            .area_moments()
            .principal_axis_angle()
            .unwrap()
            .get::<uom::si::angle::degree>(),
        45.0,
    );
    let unequal = AiscSection::L8X4X1_2;
    assert!(unequal.idealized_shape().product_moi().value < 0.0);
    close(
        unequal.product_moi().value / 0.0254_f64.powi(4),
        -9.112141328183078,
    );
}

#[test]
fn parsing_accepts_documented_labels_and_rejects_unknowns() {
    assert_eq!(" w12x26\n".parse(), Ok(AiscSection::W12X26));
    assert_eq!("hss6x6x1/4".parse(), Ok(AiscSection::HSS6X6X1_4));
    assert_eq!("HSS6X6X.250".parse(), Ok(AiscSection::HSS6X6X1_4));
    assert_eq!(AiscSection::HSS6X6X1_4.to_string(), "HSS6X6X1/4");
    assert_eq!("c12x20.7".parse(), Ok(AiscSection::C12X20_7));
    assert_eq!("Pipe24SCH20".parse(), Ok(AiscSection::Pipe24STD));
    assert_eq!("pipe2std".parse(), Ok(AiscSection::Pipe2STD));
    assert_eq!(
        "2l8x4x1/2x3/8llbb".parse(),
        Ok(AiscSection::DoubleAngle8X4X1_2X3_8LLBB)
    );
    for input in ["", "W12X999", "W12 X26", "W310X39", "C99X999", "HSS6X6X1_4"] {
        let error = input.parse::<AiscSection>().unwrap_err();
        assert_eq!(error.input(), input);
        assert!(error.to_string().contains("unknown AISC section"));
    }
}

#[test]
fn complete_catalog_roundtrips_and_matches_source_extract() {
    let mut seen = HashSet::new();
    let mut families = HashMap::new();
    for line in include_str!("../data/aisc-v16.csv").lines().skip(1) {
        let columns: Vec<_> = line.split(',').collect();
        let section: AiscSection = columns[2].parse().unwrap();
        assert!(seen.insert(section), "duplicate {}", section);
        assert_eq!(section.edi_designation(), columns[1]);
        assert_eq!(columns[1].parse::<AiscSection>(), Ok(section));
        assert_eq!(section.to_string().parse::<AiscSection>(), Ok(section));
        *families.entry(section.family()).or_insert(0) += 1;
        let value = |index: usize| columns[index].parse::<f64>().unwrap();
        close(
            section.mass_per_length().value,
            value(3) * 0.45359237 / 0.3048,
        );
        close(section.area().get::<square_inch>(), value(4));
        close(section.moi_x().value, value(14) * 0.0254_f64.powi(4));
        close(section.moi_y().value, value(15) * 0.0254_f64.powi(4));
        if columns[16].is_empty() {
            assert_eq!(columns[0], "2L");
            assert_eq!(section.torsional_constant(), None);
        } else {
            close(
                section.torsional_constant().unwrap().value,
                value(16) * 0.0254_f64.powi(4),
            );
        }
        match section.dimensions() {
            SectionDimensions::WideFlange {
                depth,
                flange_width,
                web_thickness,
                flange_thickness,
            }
            | SectionDimensions::IBeam {
                depth,
                flange_width,
                web_thickness,
                flange_thickness,
            }
            | SectionDimensions::Channel {
                depth,
                flange_width,
                web_thickness,
                flange_thickness,
            }
            | SectionDimensions::Tee {
                depth,
                flange_width,
                web_thickness,
                flange_thickness,
            } => {
                assert!(["W", "M", "S", "HP", "C", "MC", "WT", "MT", "ST"].contains(&columns[0]));
                close(depth.get::<inch>(), value(5));
                close(flange_width.get::<inch>(), value(8));
                close(web_thickness.get::<inch>(), value(10));
                close(flange_thickness.get::<inch>(), value(11));
            }
            SectionDimensions::Angle {
                vertical_leg,
                horizontal_leg,
                thickness,
            } => {
                assert_eq!(columns[0], "L");
                close(vertical_leg.get::<inch>(), value(17));
                close(horizontal_leg.get::<inch>(), value(5));
                close(thickness.get::<inch>(), value(18));
            }
            SectionDimensions::DoubleAngle {
                back_to_back_leg,
                outstanding_leg,
                thickness,
                gap,
            } => {
                assert_eq!(columns[0], "2L");
                close(back_to_back_leg.get::<inch>(), value(5));
                close(outstanding_leg.get::<inch>(), value(17));
                close(thickness.get::<inch>(), value(18));
                assert!(gap.value.is_finite() && gap.value >= 0.0);
            }
            SectionDimensions::HollowRectangle {
                height,
                width,
                nominal_thickness,
                design_thickness,
            } => {
                assert_eq!(columns[0], "HSS");
                close(height.get::<inch>(), value(6));
                close(width.get::<inch>(), value(9));
                close(nominal_thickness.get::<inch>(), value(12));
                close(design_thickness.get::<inch>(), value(13));
            }
            SectionDimensions::HollowRound {
                outer_diameter,
                nominal_thickness,
                design_thickness,
            } => {
                assert!(["HSS", "PIPE"].contains(&columns[0]));
                close(outer_diameter.get::<inch>(), value(7));
                close(nominal_thickness.get::<inch>(), value(12));
                close(design_thickness.get::<inch>(), value(13));
            }
            _ => panic!("unexpected dimensions for {}", section),
        }
        let shape = section.idealized_shape();
        assert!(shape.area().value.is_finite() && shape.area().value > 0.0);
        assert!(shape.moi_x().value.is_finite() && shape.moi_x().value > 0.0);
        assert!(shape.moi_y().value.is_finite() && shape.moi_y().value > 0.0);
    }
    assert_eq!(families[&SectionFamily::WideFlange], 289);
    assert_eq!(families[&SectionFamily::SquareHollow], 126);
    assert_eq!(families[&SectionFamily::RectangularHollow], 399);
    assert_eq!(families[&SectionFamily::RoundHollow], 189);
    for (family, count) in [
        (SectionFamily::MiscellaneousBeam, 16),
        (SectionFamily::AmericanStandardBeam, 28),
        (SectionFamily::BearingPile, 22),
        (SectionFamily::AmericanStandardChannel, 32),
        (SectionFamily::MiscellaneousChannel, 40),
        (SectionFamily::Angle, 137),
        (SectionFamily::WideFlangeTee, 289),
        (SectionFamily::MiscellaneousTee, 14),
        (SectionFamily::AmericanStandardTee, 28),
        (SectionFamily::DoubleAngle, 639),
        (SectionFamily::Pipe, 51),
    ] {
        assert_eq!(families[&family], count);
    }
    assert_eq!(families.len(), 15);
    assert_eq!(seen.len(), 2299);
    assert_eq!(AiscSection::iter().len(), seen.len());
    assert_eq!(AiscSection::iter().collect::<HashSet<_>>(), seen);
}

#[test]
fn new_families_match_published_reference_values() {
    for (section, family, area, ix, iy) in [
        (
            AiscSection::M4X6,
            SectionFamily::MiscellaneousBeam,
            1.75,
            4.72,
            1.47,
        ),
        (
            AiscSection::S12X35,
            SectionFamily::AmericanStandardBeam,
            10.2,
            228.0,
            9.84,
        ),
        (
            AiscSection::HP10X42,
            SectionFamily::BearingPile,
            12.4,
            210.0,
            71.7,
        ),
        (
            AiscSection::C12X20_7,
            SectionFamily::AmericanStandardChannel,
            6.08,
            129.0,
            3.86,
        ),
        (
            AiscSection::MC6X12,
            SectionFamily::MiscellaneousChannel,
            3.53,
            18.7,
            1.85,
        ),
        (AiscSection::L8X4X1_2, SectionFamily::Angle, 5.8, 38.6, 6.75),
        (
            AiscSection::WT6X13,
            SectionFamily::WideFlangeTee,
            3.82,
            11.7,
            8.66,
        ),
        (
            AiscSection::MT3X1_85,
            SectionFamily::MiscellaneousTee,
            0.545,
            0.483,
            0.0863,
        ),
        (
            AiscSection::ST6X17_5,
            SectionFamily::AmericanStandardTee,
            5.12,
            17.2,
            4.92,
        ),
        (
            AiscSection::Pipe2STD,
            SectionFamily::Pipe,
            1.02,
            0.627,
            0.627,
        ),
        (
            AiscSection::DoubleAngle8X4X1_2LLBB,
            SectionFamily::DoubleAngle,
            11.6,
            77.2,
            22.1,
        ),
    ] {
        assert_eq!(section.family(), family);
        close(section.area().get::<square_inch>(), area);
        close(section.moi_x().value, ix * 0.0254_f64.powi(4));
        close(section.moi_y().value, iy * 0.0254_f64.powi(4));
    }
}

#[test]
fn double_angles_preserve_spacing_orientation_and_missing_torsion() {
    for (section, height, width, expected_gap) in [
        (AiscSection::DoubleAngle8X4X1_2LLBB, 8.0, 4.0, 0.0),
        (AiscSection::DoubleAngle8X4X1_2X3_8LLBB, 8.0, 4.0, 0.375),
        (AiscSection::DoubleAngle8X4X1_2SLBB, 4.0, 8.0, 0.0),
        (AiscSection::DoubleAngle8X4X1_2X3_4SLBB, 4.0, 8.0, 0.75),
        (AiscSection::DoubleAngle12X12X1_3_8X1_1_2, 12.0, 12.0, 1.5),
        (
            AiscSection::DoubleAngle2_1_2X1_1_2X3_16X3_4SLBB,
            1.5,
            2.5,
            0.75,
        ),
    ] {
        assert_eq!(section.torsional_constant(), None);
        assert_eq!(section.properties().torsional_constant(), None);
        match section.idealized_shape() {
            StructuralShape::DoubleAngle {
                height: h,
                width: w,
                gap,
                ..
            } => {
                close(h.get::<inch>(), height);
                close(w.get::<inch>(), width);
                close(gap.get::<inch>(), expected_gap);
            }
            _ => panic!("expected a double angle"),
        }
    }
    let touching = AiscSection::DoubleAngle8X4X1_2LLBB;
    let spaced = AiscSection::DoubleAngle8X4X1_2X3_8LLBB;
    assert_eq!(touching.moi_x(), spaced.moi_x());
    assert!(spaced.moi_y() > touching.moi_y());
    close(spaced.moi_y().value, 26.1 * 0.0254_f64.powi(4));
    close(
        AiscSection::DoubleAngle8X4X1_2SLBB.moi_x().value,
        13.5 * 0.0254_f64.powi(4),
    );
}

#[test]
fn idealized_geometry_uses_design_thickness_and_retains_catalog_values() {
    match AiscSection::W12X26.idealized_shape() {
        StructuralShape::IBeam {
            height,
            width,
            web_thickness,
            flange_thickness,
            ..
        } => {
            close(height.get::<inch>(), 12.2);
            close(width.get::<inch>(), 6.49);
            close(web_thickness.get::<inch>(), 0.23);
            close(flange_thickness.get::<inch>(), 0.38);
        }
        _ => panic!("expected an I-beam"),
    }
    let section = AiscSection::HSS6X6X1_4;
    let original = section.properties();
    let mut shape = section.idealized_shape();
    match shape {
        StructuralShape::BoxBeam {
            height,
            width,
            thickness,
            ..
        } => {
            close(height.get::<inch>(), 6.0);
            close(width.get::<inch>(), 6.0);
            close(thickness.get::<inch>(), 0.233);
        }
        _ => panic!("expected a box beam"),
    }
    assert_ne!(shape.area(), section.area()); // Sharp corners vs published HSS data.
    shape.with_cog(3.0, 2.0);
    assert_eq!(section.properties(), original);
    match AiscSection::HSS6_625X0_280.idealized_shape() {
        StructuralShape::Pipe {
            outer_radius,
            thickness,
            ..
        } => {
            close(outer_radius.get::<inch>(), 6.63 / 2.0);
            close(thickness.get::<inch>(), 0.26);
        }
        _ => panic!("expected a pipe"),
    }
}

#[test]
fn selection_filters_and_ranks_standard_sections() {
    let selected = AiscSection::iter()
        .filter(|s| s.family() == SectionFamily::WideFlange)
        .filter(|s| s.moi_x() >= AiscSection::W12X26.moi_x())
        .min_by(|a, b| {
            a.mass_per_length()
                .value
                .partial_cmp(&b.mass_per_length().value)
                .unwrap()
        })
        .unwrap();
    assert!(selected.mass_per_length() <= AiscSection::W12X26.mass_per_length());
    assert!(selected.moi_x() >= AiscSection::W12X26.moi_x());
}
