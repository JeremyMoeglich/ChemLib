use std::mem::size_of_val;

use chem::core::molecule_data::MOLECULE_DATA;

fn main() {
    let data = &MOLECULE_DATA;
    let size = size_of_val(&**data);
    println!("Size of expr: {}", size);
    println!("{:#?}", data.len());
}
