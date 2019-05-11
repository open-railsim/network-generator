use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    // path to stations list file.
    #[structopt(short = "s", long = "stations", parse(from_os_str))]
    stations: PathBuf,

    // path to edges list file.
    #[structopt(short = "e", long = "edges", parse(from_os_str))]
    edges: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct StationFields {
    #[serde(default)]
    departement: String,

    #[serde(default)]
    commune: String,

    #[serde(default)]
    voyageurs: String,

    #[serde(default)]
    libelle_gare: String,

    #[serde(default)]
    coordonnees_geographiques: [f64; 2],
}

#[derive(Serialize, Deserialize, Debug)]
struct Station {
    fields: StationFields,
}

#[derive(Serialize, Deserialize, Debug)]
struct LineString {
    coordinates: Vec<[f64; 3]>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MultiLineString {
    coordinates: Vec<Vec<[f64; 3]>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum GeoShape {
    LineString(LineString),
    MultiLineString(MultiLineString),
}

#[derive(Serialize, Deserialize, Debug)]
struct EdgeFields {
    #[serde(default)]
    code_ligne: String,

    geo_shape: GeoShape,
}

#[derive(Serialize, Deserialize, Debug)]
struct Edge {
    fields: EdgeFields,
}

/// Load json from file.
fn load_stations(path: &PathBuf) -> Result<Vec<Station>, Error> {
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
fn load_edges(path: &PathBuf) -> Result<Vec<Edge>, Error> {
    // open json file.
    let file = File::open(path)?;

    // create a reader.
    let reader = BufReader::new(file);

    // parse json.
    let json_value: Vec<Edge> =
        serde_json::from_reader(reader).expect("Unable to read Edges JSon file.");

    Ok(json_value)
}

fn main() {
    // read cli args.
    let cli = Cli::from_args();

    // get edges.
    let edges = load_edges(&cli.edges).expect("Unable to read edges.");

    // get stations.
    let stations = load_stations(&cli.stations).expect("Unable to read stations.");

    println!("{:?}", stations[0]);
    println!("{:?}", edges[0]);
    // println!("{:?}", edges[0]);
}
