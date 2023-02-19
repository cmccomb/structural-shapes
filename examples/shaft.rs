use structural_shapes::{meters, StructuralShape};
use uom::fmt::DisplayStyle;
use uom::si::f64::{Pressure, Torque};
use uom::si::pressure::{megapascal, pascal};
use uom::si::torque::newton_meter;

fn main() {
    // Define height of cross-section
    let r = meters(0.25);

    // Define the shape of the cross-section
    let J = StructuralShape::new_pipe(0.5, 0.05).polar_moi();

    // Define the moment resisted by the cross-section
    let T = Torque::new::<newton_meter>(10000.0);

    // Make a formatter to use with MPa
    let mpa = Pressure::format_args(megapascal, DisplayStyle::Abbreviation);

    // Compute and print stress
    println!("{}", mpa.with(T * r / J));
}
