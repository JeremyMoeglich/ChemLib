use flate2::read::GzDecoder;
use serde::Deserialize;

fn get_raw_molecule_data() -> Vec<RawMoleculeData> {
    let compressed: &[u8] = include_bytes!("MEiLD_dataset.jsonld.gz");
    let mut decoder = GzDecoder::new(compressed);
    let data: Vec<RawMoleculeData> = serde_json::from_reader(&mut decoder).unwrap();
    data
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawMoleculeData {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type", default)]
    schemas: Vec<String>,
    #[serde(rename = "http://dbpedia.org/ontology/inchi", default)]
    inchi: Vec<RawMoleculeDataValue>,
    #[serde(rename = "http://dbpedia.org/ontology/molecularWeight", default)]
    molecular_weight: Vec<RawMoleculeDataValue>,
    #[serde(rename = "http://dbpedia.org/property/inchikey", default)]
    inchikey: Vec<RawMoleculeDataValue>,
    #[serde(rename = "http://www.w3.org/2000/01/rdf-schema#seeAlso", default)]
    see_also: Vec<RawMoleculeDataId>,
    #[serde(rename = "http://www.w3.org/2004/02/skos/core#broader", default)]
    broader: Vec<RawMoleculeDataId>,
    #[serde(rename = "http://www.w3.org/2004/02/skos/core#narrower", default)]
    narrower: Vec<RawMoleculeDataId>,
    #[serde(rename = "http://www.w3.org/2004/02/skos/core#example", default)]
    example: Vec<RawMoleculeDataValue>,
    #[serde(rename = "http://www.w3.org/2004/02/skos/core#hiddenLabel", default)]
    hidden_label: Vec<RawMoleculeDataValue>,
    #[serde(rename = "http://www.w3.org/2004/02/skos/core#notation", default)]
    notation: Vec<RawMoleculeDataValue>,
    #[serde(rename = "http://www.w3.org/2004/02/skos/core#definition", default)]
    definition: Vec<RawMoleculeDataValue>,
    #[serde(rename = "http://www.w3.org/2004/02/skos/core#prefLabel", default)]
    pref_label: Vec<RawMoleculeDataValue>,
    #[serde(rename = "http://www.w3.org/2004/02/skos/core#related", default)]
    related: Vec<RawMoleculeDataId>,
    #[serde(rename = "http://dbpedia.org/ontology/casNumber", default)]
    cas_number: Vec<RawMoleculeDataValue>,
    #[serde(rename = "http://www.w3.org/2004/02/skos/core#altLabel", default)]
    alt_label: Vec<RawMoleculeDataValue>,
    #[serde(rename = "http://ii.uwb.edu.pl/cvme#patent", default)]
    patent: Vec<RawMoleculeDataId>,
}

#[derive(Debug, Deserialize)]
struct RawMoleculeDataValue {
    #[serde(rename = "@value")]
    value: String,
    #[serde(rename = "@type")]
    schema: Option<String>,
    #[serde(rename = "@language")]
    language: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawMoleculeDataId {
    #[serde(rename = "@id")]
    id: String,
}

pub fn get_molecule_data() -> Vec<MoleculeData> {
    let raw_data = get_raw_molecule_data();
    let mut data = Vec::new();
    for raw_molecule_data in raw_data {
        data.push(raw_molecule_data);
    }
    data
}

type MoleculeData = RawMoleculeData; // TODO: Add more fields
