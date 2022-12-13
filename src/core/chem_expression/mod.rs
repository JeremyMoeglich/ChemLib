use nom::{
    bytes::complete::tag, combinator::map, multi::separated_list1, sequence::delimited, IResult,
};

use super::{
    molecule::{parse_molecule, Molecule},
    parser_utils::ws0,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ChemExpression {
    pub elements: Vec<(usize, Molecule)>,
}

impl ChemExpression {
    pub fn new(elements: Vec<(usize, Molecule)>) -> Self {
        Self { elements }
    }
    pub fn parse(input: &str) -> Result<Self, String> {
        match parse_chem_expression(input) {
            Ok((input, chem_expression)) => match input {
                "" => Ok(chem_expression),
                _ => Err("Failed to parse chem expression (Too long)".to_string()),
            },
            Err(_) => Err("Failed to parse chem expression".to_string()),
        }
    }
}

pub fn parse_chem_expression(input: &str) -> IResult<&str, ChemExpression> {
    map(
        separated_list1(delimited(ws0, tag("+"), ws0), parse_molecule),
        |elements| {
            let elements = elements.into_iter().map(|element| (1, element)).collect();
            ChemExpression::new(elements)
        },
    )(input)
}
