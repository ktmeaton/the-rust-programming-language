//use clap::Parser;
use bio::io::fasta;
use std::collections::HashMap;

use rebar::genome::genome::Genome;
use rebar::dataset::dataset::Dataset;

//const PANGO_SEQUENCES_URL : &str = "https://raw.githubusercontent.com/corneliusroemer/pango-sequences/main/data/pango-consensus-sequences_genome-nuc.fasta.zst";

// /// Simple program to greet a person
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     /// Name of the person to greet
//     #[arg(short, long)]
//     name: String,

//     /// Number of times to greet
//     #[arg(short, long, default_value_t = 1)]
//     count: u8,
// }

fn main() {

    let mask = 200;
    let dataset = "dataset/sars-cov-2-latest";
    let reference_path = format!("{}/{}", dataset, "reference.fasta");
    let parents_path = format!("{}/{}", dataset, "parents.fasta");

    let reference_reader = fasta::Reader::from_file(reference_path).unwrap();
    let parents_reader = fasta::Reader::from_file(parents_path).unwrap();

    let reference = reference_reader.records().next().unwrap().unwrap();
    // Convert seq from &[u8] into Vec<char> because we might modify it by masking
    // Source: https://gist.github.com/jimmychu0807/9a89355e642afad0d2aeda52e6ad2424#file-string-conversion-rs-L24
    // Author: Jimmy Chu
    let reference_seq = reference.seq().iter().map(|b| *b as char).collect::<Vec<_>>();
    let mut genomes: HashMap<String, Genome> = HashMap::new();
    
    // Read in populations, barcodes
    for record in parents_reader.records() {

        let sample = record.unwrap();

        // Debug
        if sample.id() != "XA" && sample.id() != "XB" {continue}


        let id = String::from(sample.id());
        let alphabet = vec!['A', 'C', 'G', 'T'];
        let seq = sample.seq().iter().map(|b| *b as char).collect::<Vec<_>>();
        let mut genome = Genome::new(id, seq, alphabet);

        genome.parse_sequence(&reference_seq, mask);
        genomes.insert(genome.id.clone(), genome);
    }

    //println!("{:?}", genomes);
    let dataset = Dataset::new(genomes);
    //println!("{:?}", dataset);
    //let result = dataset.create_barcodes();
 
}

