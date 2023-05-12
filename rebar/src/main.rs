
use std::env;

// External crates
use log::info;

// This crate
use rebar::dataset::sequences::Sequences;
use rebar::dataset::phylogeny::Phylogeny;
//use rebar::traits::Summary;

fn main() {

    let verbosity = "debug";

    // Set default logging level if RUST_LOG is not set.
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", verbosity)
    }

    env_logger::init();

    let dataset_name = "sars-cov-2".to_string();
    let dataset_tag = "nightly".to_string();
    let dataset_dir = format!("dataset/{}-{}", &dataset_name, &dataset_tag);

    let mask = 200;

    let sequences_path = format!("data/XBB.1.16.fasta");
    let reference_path = format!("{}/{}", &dataset_dir, "reference.fasta");
    let populations_path = format!("{}/{}", &dataset_dir, "populations.fasta");
    let phylogeny_path = format!("{}/{}", &dataset_dir, "graph.dot");

    // ------------------------------------------------------------------------
    // Dataset

    // Sequences
    info!("Preparing dataset sequences: {}", &populations_path);
    let mut dataset = Sequences::new();
    dataset.set_sequences(&reference_path, &populations_path, &mask).unwrap();
    dataset.set_mutations().unwrap();

    // Phylogeny
    info!("Preparing dataset phylogeny: {}", &phylogeny_path);
    let mut phylogeny = Phylogeny::new();
    phylogeny.build_graph(&dataset_name, &dataset_tag, &dataset_dir).expect("Failed to build phylogeny.");
    phylogeny.export_graph(&dataset_dir).expect("Failed to export phylogeny.");

    // ------------------------------------------------------------------------
    // Run

    info!("Importing query sequences: {}", &sequences_path);    
    let mut query = Sequences::new();
    query.set_sequences(&reference_path, &sequences_path, &mask).unwrap();
    query.summarise_barcodes(&dataset).unwrap();


}

