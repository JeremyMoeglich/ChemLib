use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct ElementData {
    pub name: String,
    pub appearance: Option<String>,
    pub atomic_mass: f64,
    pub boil: Option<f64>,
    pub category: String,
    pub density: Option<f64>,
    pub discovered_by: Option<String>,
    pub melt: Option<f64>,
    pub molar_heat: Option<f64>,
    pub named_by: Option<String>,
    pub number: usize,
    pub period: usize,
    pub phase: String,
    pub source: String,
    pub bohr_model_image: Option<String>,
    pub bohr_model_3d: Option<String>,
    pub spectral_img: Option<String>,
    pub summary: String,
    pub symbol: String,
    pub xpos: usize,
    pub ypos: usize,
    pub shells: Vec<usize>,
    pub electron_configuration: String,
    pub electron_configuration_semantic: String,
    pub electron_affinity: Option<f64>,
    pub electronegativity_pauling: Option<f64>,
    pub ionization_energies: Vec<f64>,
    pub cpk_hex: Option<String>,
    pub image: Image,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Image {
    pub title: String,
    pub url: String,
    pub attribution: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct ElementDataList {
    pub elements: Vec<ElementData>,
}

lazy_static! {
    pub static ref ELEMENT_DATA: HashMap<String, ElementData> = {
        let text = include_str!("elements.json");
        let elements: ElementDataList =
            serde_json::from_str(text).unwrap();
        let mut map = HashMap::new();
        for element in elements.elements {
            map.insert(element.symbol.clone(), element);
        }
        map
    };
}
