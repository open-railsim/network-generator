use std::path::PathBuf;
use structopt::StructOpt;

mod file_loader;
mod libnetgen;

#[derive(StructOpt, Debug)]
struct Cli {
    // path to stations list file.
    #[structopt(short = "s", long = "stations", parse(from_os_str))]
    stations: PathBuf,

    // path to edges list file.
    #[structopt(short = "e", long = "edges", parse(from_os_str))]
    edges: PathBuf,
}

fn main() {
    // read cli args.
    let cli = Cli::from_args();

    // get edges.
    let edges = file_loader::load_edges(&cli.edges).expect("Unable to read edges.");

    // get stations.
    //let stations = file_loader::load_stations(&cli.stations).expect("Unable to read stations.");

    let mut network = libnetgen::Network { lines: vec![] };

    for edge in edges {
        // get geo shape.
        let geo_shape = &edge.fields.geo_shape;

        // convert geo_shape to lines.
        match geo_shape {
            file_loader::GeoShape::LineString(line) => {
                network
                    .lines
                    .push(libnetgen::geo_shape_to_line(&line.coordinates));
            },
            file_loader::GeoShape::MultiLineString(multi_line) =>  {
                for line in libnetgen::geo_shape_to_multi_line(&multi_line.coordinates) {
                    network.lines.push(line);
                }
            },
        }
    }

    println!("{:?}", network);
}
