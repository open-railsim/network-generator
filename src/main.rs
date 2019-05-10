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
struct Station {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Edge {
    name: String,
    age: u8,
    phones: Vec<String>,
}

/// Load json from file.
fn load_stations(path: &PathBuf) -> Result<Station, Error> {
    // open json file.
    let file = File::open(path)?;

    // create a reader.
    let reader = BufReader::new(file);

    // parse json.
    let json_value: Station = serde_json::from_reader(reader).expect("Unable to read JSon file.");

    Ok(json_value)
}

/// Load json from file.
fn load_edges(path: &PathBuf) -> Result<Edge, Error> {
    // open json file.
    let file = File::open(path)?;

    // create a reader.
    let reader = BufReader::new(file);

    // parse json.
    let json_value: Edge = serde_json::from_reader(reader).expect("Unable to read JSon file.");

    Ok(json_value)
}

fn main() {
    // read cli args.
    let cli = Cli::from_args();

    // get stations.
    let stations = load_stations(&cli.stations).expect("Unable to read stations.");

    // get edges.
    let edges = load_edges(&cli.edges).expect("Unable to read edges.");

    println!("{:?}", stations);
    println!("{:?}", edges);
    // println!("{:?}", edges[0]);
}
