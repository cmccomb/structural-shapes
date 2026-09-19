#![cfg(feature = "aisc")]

use std::collections::{HashMap, HashSet};
use structural_shapes::{AiscSection, SectionDimensions, SectionFamily, StructuralShape};
use uom::si::{area::square_inch, length::inch};

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= expected.abs() * 1e-12,
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
    close(beam.torsional_constant().value, 0.3 * 0.0254_f64.powi(4));
    close(beam.mass_per_length().value, 26.0 * 0.45359237 / 0.3048);
    close(
        beam.polar_moi().value,
        beam.moi_x().value + beam.moi_y().value,
    );
    assert!(beam.polar_moi() > beam.torsional_constant());

    let square = AiscSection::HSS6X6X1_4;
    close(square.area().get::<square_inch>(), 5.24);
    close(square.moi_x().value, 28.6 * 0.0254_f64.powi(4));
    assert_eq!(square.moi_x(), square.moi_y());
    let rectangle = AiscSection::HSS8X4X1_4;
    close(rectangle.moi_x().value, 42.5 * 0.0254_f64.powi(4));
    close(rectangle.moi_y().value, 14.4 * 0.0254_f64.powi(4));
    let round = AiscSection::HSS6_625X0_280;
    close(round.area().get::<square_inch>(), 5.2);
    close(round.moi_x().value, 26.4 * 0.0254_f64.powi(4));
}

#[test]
fn parsing_accepts_documented_labels_and_rejects_unknowns() {
    assert_eq!(" w12x26\n".parse(), Ok(AiscSection::W12X26));
    assert_eq!("hss6x6x1/4".parse(), Ok(AiscSection::HSS6X6X1_4));
    assert_eq!("HSS6X6X.250".parse(), Ok(AiscSection::HSS6X6X1_4));
    assert_eq!(AiscSection::HSS6X6X1_4.to_string(), "HSS6X6X1/4");
    for input in [
        "",
        "W12X999",
        "W12 X26",
        "W310X39",
        "C12X20.7",
        "HSS6X6X1_4",
    ] {
        let error = input.parse::<AiscSection>().unwrap_err();
        assert_eq!(error.input(), input);
        assert!(error.to_string().contains("unknown AISC W/HSS section"));
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
        close(
            section.torsional_constant().value,
            value(16) * 0.0254_f64.powi(4),
        );
        match section.dimensions() {
            SectionDimensions::WideFlange {
                depth,
                flange_width,
                web_thickness,
                flange_thickness,
            } => {
                assert_eq!(columns[0], "W");
                close(depth.get::<inch>(), value(5));
                close(flange_width.get::<inch>(), value(8));
                close(web_thickness.get::<inch>(), value(10));
                close(flange_thickness.get::<inch>(), value(11));
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
                assert_eq!(columns[0], "HSS");
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
    assert_eq!(seen.len(), 1003);
    assert_eq!(AiscSection::iter().len(), seen.len());
    assert_eq!(AiscSection::iter().collect::<HashSet<_>>(), seen);
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
