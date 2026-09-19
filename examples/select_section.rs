//! Select the lightest W section meeting geometric constraints.
use structural_shapes::{meters, AiscSection, SectionDimensions, SectionFamily};
use uom::si::linear_mass_density::kilogram_per_meter;

fn main() {
    let reference = AiscSection::W12X26;
    let selected = AiscSection::iter()
        .filter(|section| section.family() == SectionFamily::WideFlange)
        .filter(|section| section.moi_x() >= reference.moi_x())
        .filter(|section| match section.dimensions() {
            SectionDimensions::WideFlange { depth, .. } => depth <= meters(0.36),
            _ => false,
        })
        .min_by(|a, b| {
            a.mass_per_length()
                .value
                .partial_cmp(&b.mass_per_length().value)
                .expect("catalog masses are finite")
        });

    if let Some(section) = selected {
        println!(
            "{}: {:.2} kg/m, Ix = {:.6e} m^4",
            section,
            section.mass_per_length().get::<kilogram_per_meter>(),
            section.moi_x().value,
        );
    }
}
