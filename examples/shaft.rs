use structural_shapes::{meters, StructuralShape};
use uom::fmt::DisplayStyle;
use uom::si::f64::{Pressure, Torque};
use uom::si::pressure::megapascal;
use uom::si::torque::newton_meter;

fn main() {
    // Maximum shear stress occurs at the outer surface of this circular pipe.
    let r = meters(0.5);

    // Define the shape of the cross-section
    let torsional_constant = StructuralShape::new_pipe(0.5, 0.05)
        .torsional_constant()
        .expect("exact J is available for a concentric circular pipe");

    // Define the moment resisted by the cross-section
    let torque = Torque::new::<newton_meter>(10000.0);

    // Make a formatter to use with MPa
    let mpa = Pressure::format_args(megapascal, DisplayStyle::Abbreviation);

    // tau = T*r/J is valid here for a circular section, not for a general section.
    println!("{}", mpa.with(torque * r / torsional_constant));
}
