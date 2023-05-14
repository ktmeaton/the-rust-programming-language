
use std::env;
use std::path::Path;

// External crates
use log::info;

// This crate
use rebar::dataset::Dataset;
use rebar::dataset::sequences::Sequences;

fn main() {

    let verbosity = "debug";

    // Set default logging level if RUST_LOG is not set.
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", verbosity)
    }

    env_logger::init();

    let dataset_name = "sars-cov-2";
    let dataset_tag = "nightly";
    let dataset_dir = Path::new("dataset/sars-cov-2-nightly");

    let mask = 200;

    let sequences_path = Path::new("data/XBB.1.16.fasta");
    let reference_path = dataset_dir.join("reference.fasta");
    let populations_path = dataset_dir.join("populations.fasta");
    let phylogeny_path = dataset_dir.join("graph.dot");

    // ------------------------------------------------------------------------
    // Dataset

    let mut dataset = Dataset::new();

    // Phylogeny
    info!("Preparing dataset phylogeny: {}", &phylogeny_path.display());    
    dataset.phylogeny.build_graph(&dataset_name, &dataset_tag, &dataset_dir).expect("Failed to build phylogeny.");
    dataset.phylogeny.export_graph(&dataset_dir).expect("Failed to export phylogeny.");

    // // Sequences
    // info!("Preparing dataset sequences: {}", &populations_path.display());
    // dataset.populations.set_sequences(&reference_path, &populations_path, &mask).unwrap();
    // dataset.populations.set_mutations().unwrap();

    // // ------------------------------------------------------------------------
    // // Run

    // info!("Importing query sequences: {}", &sequences_path.display());    
    // let mut query = Sequences::new();
    // query.set_sequences(&reference_path, &sequences_path, &mask).unwrap();
    // query.summarise_barcodes(&dataset).unwrap();


}

