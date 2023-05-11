
use std::env;

// External crates
use log::info;

// This crate
use rebar::dataset::Dataset;
use rebar::tree::{build_tree, to_newick};
//use rebar::traits::Summary;

fn main() {

    let verbosity = "debug";

    // Set default logging level if RUST_LOG is not set.
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", verbosity)
    }

    env_logger::init();

    // let mask = 200;
    // let sequences_path = format!("data/XBB.1.16.fasta");
    // let dataset_path = "dataset/sars-cov-2-latest";
    // let reference_path = format!("{}/{}", dataset_path, "reference.fasta");
    // let populations_path = format!("{}/{}", dataset_path, "populations.fasta");

    // // Dataset subcommand
    // info!("Importing populations: {}", populations_path);
    // let mut dataset = Dataset::new();
    // dataset.set_sequences(reference_path.clone(), populations_path, mask).unwrap();
    // dataset.set_mutations().unwrap();
    // //println!("{}", dataset.summary());

    // // Run subcommand
    // info!("Importing query sequences: {}", sequences_path);    
    // let mut query = Dataset::new();
    // query.set_sequences(reference_path.clone(), sequences_path, mask).unwrap();

    // query.summarise_barcodes(&dataset).unwrap();
    // //println!("{}", query.summary());

    let (tree, name_to_id) = build_tree().unwrap();
    to_newick(tree, name_to_id);

}

