use nom::{character::complete::digit0, combinator::map, multi::many1, sequence::tuple, IResult};

use super::element::{parse_element, Element};

#[derive(Clone, Debug, PartialEq)]
pub struct Molecule {
    elements: Vec<(usize, Element)>,
}

impl Molecule {
    pub fn new(elements: Vec<(usize, Element)>) -> Self {
        Self { elements }
    }
    pub fn parse(input: &str) -> Result<Self, String> {
        match parse_molecule(input) {
            Ok((input, molecule)) => match input {
                "" => Ok(molecule),
                _ => Err("Failed to parse molecule (Too long)".to_string()),
            },
            Err(_) => Err("Failed to parse molecule".to_string()),
        }
    }
    pub fn ionize(&self) -> Self {
        /// Make sure electrons are distributed correctly between elements
        let mut elements = self.elements.clone();
        todo!()
    }
}

pub fn parse_molecule(input: &str) -> IResult<&str, Molecule> {
    map(
        many1(map(tuple((parse_element, digit0)), |(element, value)| {
            (element, value.parse())
        })),
        |elements| {
            let elements = elements
                .into_iter()
                .map(|(element, value)| match value {
                    Ok(value) => (value, element),
                    Err(_) => (1, element),
                })
                .collect();
            Molecule::new(elements).ionize()
        },
    )(input)
}
