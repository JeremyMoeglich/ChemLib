use std::{str::FromStr};

use enum_index::EnumIndex;
use enum_index_derive::EnumIndex;
use nom::IResult;
use strum_macros::{Display, EnumString};

use super::element_data::ELEMENT_DATA;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, EnumString, Display, EnumIndex)]
pub enum Atom {
    H,
    He,
    Li,
    Be,
    B,
    C,
    N,
    O,
    F,
    Ne,
    Na,
    Mg,
    Al,
    Si,
    P,
    S,
    Cl,
    Ar,
    K,
    Ca,
    Sc,
    Ti,
    V,
    Cr,
    Mn,
    Fe,
    Co,
    Ni,
    Cu,
    Zn,
    Ga,
    Ge,
    As,
    Se,
    Br,
    Kr,
    Rb,
    Sr,
    Y,
    Zr,
    Nb,
    Mo,
    Tc,
    Ru,
    Rh,
    Pd,
    Ag,
    Cd,
    In,
    Sn,
    Sb,
    Te,
    I,
    Xe,
    Cs,
    Ba,
    La,
    Ce,
    Pr,
    Nd,
    Pm,
    Sm,
    Eu,
    Gd,
    Tb,
    Dy,
    Ho,
    Er,
    Tm,
    Yb,
    Lu,
    Hf,
    Ta,
    W,
    Re,
    Os,
    Ir,
    Pt,
    Au,
    Hg,
    Tl,
    Pb,
    Bi,
    Po,
    At,
    Rn,
    Fr,
    Ra,
    Ac,
    Th,
    Pa,
    U,
    Np,
    Pu,
    Am,
    Cm,
    Bk,
    Cf,
    Es,
    Fm,
    Md,
    No,
    Lr,
    Rf,
    Db,
    Sg,
    Bh,
    Hs,
    Mt,
    Ds,
    Rg,
    Cn,
    Nh,
    Fl,
    Mc,
    Lv,
    Ts,
    Og,
}

impl Atom {
    pub fn parse(input: &str) -> Result<Self, String> {
        Atom::from_str(input).map_err(|_| format!("Invalid atom: {}", input))
    }
    pub fn string(&self) -> String {
        self.to_string()
    }
    pub fn protons(&self) -> usize {
        self.enum_index() + 1
    }
    pub fn default_electrons(&self) -> usize {
        let proton_count = self.protons();
        let shell_size = |n| 2 * ((n + 1) / 2);
        let mut n: usize = 0;
        let mut count = 0;
        while count < proton_count {
            n += 1;
            count += shell_size(n);
        }
        count - shell_size(n) + 1
    }
    pub fn default_neutrons(&self) -> f64 {
        let mass = ELEMENT_DATA.get(&self.string()).unwrap().atomic_mass;
        let protons = self.protons();
        mass - protons as f64
    }
}

pub fn parse_atom(input: &str) -> IResult<&str, Atom> {
    let (input, atom) = nom::character::complete::alpha1(input)?;
    match Atom::parse(atom) {
        Ok(atom) => Ok((input, atom)),
        Err(_) => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Alpha,
        ))),
    }
}