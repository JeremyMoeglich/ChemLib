use flate2::read::GzDecoder;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::io::Read;

fn decompress_data() -> String {
    let compressed: &[u8] = include_bytes!("MEiLD_dataset.jsonld.gz");
    let mut decoder = GzDecoder::new(compressed);
    let mut decoded = String::new();
    decoder.read_to_string(&mut decoded).unwrap();
    decoded
}

fn get_raw_molecule_data() -> Vec<RawMoleculeData> {
    let decompressed = decompress_data();
    let data: Vec<RawMoleculeData> = serde_json::from_str(&decompressed).unwrap();
    data
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMoleculeData {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type", default)]
    _schemas: Vec<String>,
    #[serde(rename = "http://dbpedia.org/ontology/inchi", default)]
    inchi: Vec<RawMoleculeDataValue>,
    #[serde(rename = "http://dbpedia.org/ontology/molecularWeight", default)]
    molecular_weight: Vec<RawMoleculeDataValue>,
    #[serde(rename = "http://dbpedia.org/property/inchikey", default)]
    inchikey: Vec<RawMoleculeDataValue>,
    #[serde(rename = "http://www.w3.org/2000/01/rdf-schema#seeAlso", default)]
    _see_also: Vec<RawMoleculeDataId>,
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
    _patent: Vec<RawMoleculeDataId>,
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

#[derive(Debug)]
pub struct MoleculeData {
    pub id: usize,
    pub inchi: Option<String>,
    pub molecular_weight: Option<f64>,
    pub inchikey: Option<String>,
    pub broader: Vec<usize>,
    pub narrower: Vec<usize>,
    pub related: Vec<usize>,
    pub example: Option<String>,
    pub hidden_label: Option<String>,
    pub notation_smiles: Option<String>,
    pub definition: Option<String>,
    pub pref_label: Option<String>,
    pub cas_number: Option<String>,
    pub alt_labels: Vec<String>,
}

fn extract_id(url: &str) -> usize {
    let prefix = "CHEBI_";
    let last_path = url.split('/').last().unwrap();
    let id = last_path.trim_start_matches(prefix);
    id.parse().unwrap()
}

pub fn get_molecule_data() -> Vec<MoleculeData> {
    let raw_data = get_raw_molecule_data();
    let mut data = Vec::new();

    for raw_molecule_data in raw_data {
        let molecule_data = MoleculeData {
            id: extract_id(&raw_molecule_data.id),
            inchi: raw_molecule_data.inchi.get(0).map(|x| x.value.clone()),
            molecular_weight: raw_molecule_data
                .molecular_weight
                .get(0)
                .map(|x| x.value.parse().unwrap()),
            inchikey: raw_molecule_data.inchikey.get(0).map(|x| x.value.clone()),
            broader: raw_molecule_data
                .broader
                .iter()
                .map(|x| extract_id(&x.id))
                .collect(),
            narrower: raw_molecule_data
                .narrower
                .iter()
                .map(|x| extract_id(&x.id))
                .collect(),
            related: raw_molecule_data
                .related
                .iter()
                .map(|x| extract_id(&x.id))
                .collect(),
            example: raw_molecule_data.example.get(0).map(|x| x.value.clone()),
            hidden_label: raw_molecule_data.hidden_label.get(0).map(|x| x.value.clone()),
            notation_smiles: raw_molecule_data.notation.get(0).map(|x| x.value.clone()),
            definition: raw_molecule_data.definition.get(0).map(|x| x.value.clone()),
            pref_label: raw_molecule_data.pref_label.get(0).map(|x| x.value.clone()),
            cas_number: raw_molecule_data.cas_number.get(0).map(|x| x.value.clone()),
            alt_labels: raw_molecule_data
                .alt_label
                .iter()
                .map(|x| x.value.clone())
                .collect(),
        };
        data.push(molecule_data);
    }

    data
}

lazy_static! {
    pub static ref MOLECULE_DATA: Vec<MoleculeData> = get_molecule_data();
}
