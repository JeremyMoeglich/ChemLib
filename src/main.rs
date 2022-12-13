use chem::core::{molecule::Molecule, atom::Atom, element::Element, chem_expression::ChemExpression, molecule_data::get_molecule_data};

fn main() {
    let expr = get_molecule_data();
    println!("{:#?}", expr.len());
}
