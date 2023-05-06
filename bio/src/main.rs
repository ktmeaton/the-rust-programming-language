use bio::io::fasta;
use std::io;

fn main() {
    
    // Read in reference.
    let reference_path = "data/reference.fasta";
    let sequences_path = "data/sequences.fasta";

    let mut ref_reader = fasta::Reader::from_file(reference_path)
        .unwrap();
    let mut seq_reader = fasta::Reader::from_file(sequences_path)
        .unwrap();

    println!("{:?}", ref_reader);
    println!("{:?}", seq_reader);

}
