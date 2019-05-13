use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct StationFields {
    #[serde(default)]
    pub departement: String,

    #[serde(default)]
    pub commune: String,

    #[serde(default)]
    pub voyageurs: String,

    #[serde(default)]
    pub libelle_gare: String,

    #[serde(default)]
    pub coordonnees_geographiques: [f64; 2],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Station {
    pub fields: StationFields,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LineString {
    pub coordinates: Vec<[f64; 3]>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiLineString {
    pub coordinates: Vec<Vec<[f64; 3]>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum GeoShape {
    LineString(LineString),
    MultiLineString(MultiLineString),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EdgeFields {
    #[serde(default)]
    pub code_ligne: String,

    pub geo_shape: GeoShape,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    pub fields: EdgeFields,
}

/// Load json from file.
pub fn load_stations(path: &PathBuf) -> Result<Vec<Station>, Error> {
    // open json file.
    let file = File::open(path)?;

    // create a reader.
    let reader = BufReader::new(file);

    // parse json.
    let json_value: Vec<Station> =
        serde_json::from_reader(reader).expect("Unable to read Stations JSon file.");

    Ok(json_value)
}

/// Load json from file.
pub fn load_edges(path: &PathBuf) -> Result<Vec<Edge>, Error> {
    // open json file.
    let file = File::open(path)?;

    // create a reader.
    let reader = BufReader::new(file);

    // parse json.
    let json_value: Vec<Edge> =
        serde_json::from_reader(reader).expect("Unable to read Edges JSon file.");

    Ok(json_value)
}
