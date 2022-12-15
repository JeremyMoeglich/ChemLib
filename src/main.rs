use chem::core::molecule_data::MOLECULE_DATA;

fn main() {
    let data = &MOLECULE_DATA;
    macro_rules! stats {
        ($name:ident) => {
            println!(
                "{:?} {:?}",
                stringify!($name),
                data.iter().filter(|x| x.$name.is_some()).count()
            );
        };
    }
    macro_rules! stats_vec {
        ($name:ident) => {
            println!(
                "{:?} {:?}",
                stringify!($name),
                data.iter().filter(|x| !x.$name.is_empty()).count()
            );
        };
    }
    stats!(inchi);
    stats!(molecular_weight);
    stats!(inchikey);
    stats_vec!(broader);
    stats_vec!(narrower);
    stats_vec!(related);
    stats!(example);
    stats!(hidden_label);
    stats!(notation_smiles);
    stats!(definition);
    stats!(pref_label);
    stats!(cas_number);
    stats_vec!(alt_labels);
    
    println!(r#""total": {:?}"#, data.len());
}
