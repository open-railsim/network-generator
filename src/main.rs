use std::fs::File;
use std::io::Error;
use std::io::Read;
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

/// Load json from file.
fn load_json_file(path: &PathBuf) -> Result<serde_json::Value, Error> {
    // open json file.
    let mut file = File::open(path)?;

    // create mutable buffer.
    let mut buffer = String::new();

    // read the file into the buffer.
    file.read_to_string(&mut buffer)
        .expect("Unable to read file.");

    // parse json.
    let json_value: serde_json::Value =
        serde_json::from_str(&buffer).expect("Unable to read JSon file.");

    Ok(json_value)
}

fn main() {
    // read cli args.
    let cli = Cli::from_args();

    // get stations.
    let stations = load_json_file(&cli.stations).expect("Unable to read stations.");

    // get edges.
    let edges = load_json_file(&cli.edges).expect("Unable to read edges.");

    println!("{:?}", stations);
    println!("{:?}", edges);
}
