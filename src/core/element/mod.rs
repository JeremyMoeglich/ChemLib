use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit0, combinator::map, IResult,
};

use crate::core::atom::Atom;

use super::atom::parse_atom;

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub atom: Atom,
    neutrons: f64,
    electrons: usize,
}

impl Element {
    pub fn new_all(atom: Atom, neutrons: f64, electrons: usize) -> Self {
        Self {
            atom,
            neutrons,
            electrons,
        }
    }
    pub fn new_atom(atom: Atom) -> Self {
        let default_neutrons = atom.default_neutrons();
        let default_electrons = atom.default_electrons();
        Self {
            atom,
            neutrons: default_neutrons,
            electrons: default_electrons,
        }
    }
    pub fn new_ion(atom: Atom, electrons: usize) -> Self {
        let default_neutrons = atom.default_neutrons();
        Self {
            atom,
            neutrons: default_neutrons,
            electrons,
        }
    }
    pub fn new_isotope(atom: Atom, neutrons: f64) -> Self {
        let default_electrons = atom.default_electrons();
        Self {
            atom,
            neutrons,
            electrons: default_electrons,
        }
    }

    pub fn neutrons(&self) -> f64 {
        self.neutrons
    }

    pub fn electrons(&self) -> usize {
        self.electrons
    }

    pub fn protons(&self) -> usize {
        self.atom.protons()
    }

    pub fn mass(&self) -> f64 {
        self.protons() as f64 + self.neutrons()
    }

    pub fn parse(input: &str) -> Result<Self, String> {
        match parse_element(input) {
            Ok((input, element)) => match input {
                "" => Ok(element),
                _ => Err(format!("Invalid element: {}", input)),
            },
            Err(_) => Err(format!("Invalid element: {}", input)),
        }
    }
}

enum Charge {
    Positive,
    Negative,
    Neutral,
}

pub fn parse_element(input: &str) -> IResult<&str, Element> {
    let (input, atom) = parse_atom(input)?;
    let (input, charge) = alt((
        map(tag("+"), |_| Charge::Positive),
        map(tag("-"), |_| Charge::Negative),
        map(tag(""), |_| Charge::Neutral),
    ))(input)?;
    let (input, value) = {
        let (input, value) = match charge {
            Charge::Positive => digit0(input)?,
            Charge::Negative => digit0(input)?,
            Charge::Neutral => (input, ""),
        };
        match charge {
            Charge::Positive => (input, -value.parse::<isize>().unwrap_or(0)),
            Charge::Negative => (input, value.parse::<isize>().unwrap_or(0)),
            Charge::Neutral => (input, 0),
        }
    };
    let default_electrons = atom.default_electrons();
    if default_electrons as isize + value < 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )));
    }
    Ok((
        input,
        Element::new_ion(atom, default_electrons + value as usize),
    ))
}
