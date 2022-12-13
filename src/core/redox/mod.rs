use lazy_static::lazy_static;

use super::chem_expression::ChemExpression;

#[derive(Clone, Debug, PartialEq)]
pub struct RedoxData {
    reduced: ChemExpression,
    oxidized: ChemExpression,
    potential: f64,
}

impl<'a> RedoxData {
    pub fn new(reduced: ChemExpression, oxidized: ChemExpression, potential: f64) -> Self {
        Self {
            reduced,
            oxidized,
            potential,
        }
    }

    pub fn reduced(&'a self) -> &'a ChemExpression {
        &self.reduced
    }

    pub fn oxidized(&'a self) -> &'a ChemExpression {
        &self.oxidized
    }

    pub fn potential(&self) -> f64 {
        self.potential
    }
}

lazy_static! {
    pub static ref REDOX_DATA: Vec<RedoxData> = {
        //read data from redox.txt
        let mut data = Vec::new();
        let text = include_str!("redox.txt");
        for line in text.lines() {
            let mut split = line.split("|");
            let tuple = (split.next().unwrap(), split.next().unwrap(), split.next().unwrap());
            let reduced = ChemExpression::parse(tuple.0).unwrap();
            let oxidized = ChemExpression::parse(tuple.1).unwrap();
            let potential = tuple.2.parse().unwrap();
            data.push(RedoxData::new(reduced, oxidized, potential));
        };
        data
    };
}
