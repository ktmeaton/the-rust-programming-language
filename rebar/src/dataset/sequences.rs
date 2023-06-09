// Standard library
use std::collections::HashMap;
use std::path::Path;

// External crates
use bio::io::fasta;
use itertools::Itertools;
use eyre::Report;
use log::debug;

// This crate
use crate::dataset::Dataset;
use crate::genome::Genome;
use crate::mutation::Mutation;
use crate::traits::Summary;

#[derive(Debug)]
pub struct Sequences {
    pub sequences: HashMap<String,Genome>,
    pub mutations: HashMap<Mutation, Vec<String>>
}

impl Summary for Sequences {
    fn summary(&self) -> String {
        format!(
            "sequences: {}\nmutations: {}\n",
            self.sequences.keys().sorted().join(", "),
            self.mutations.keys()
                .sorted()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

impl Sequences {
    pub fn new() -> Self {
        Sequences { sequences: HashMap::new(), mutations : HashMap::new() }
    }

    pub fn set_sequences(&mut self, reference_path: &Path, sequences_path: &Path, mask : &isize) -> Result<(), Report> {
        let mut sequences: HashMap<String, Genome> = HashMap::new();

        let reference_reader = fasta::Reader::from_file(reference_path).unwrap();
        let sequences_reader = fasta::Reader::from_file(sequences_path).unwrap();
        let reference = reference_reader.records().next().unwrap().unwrap();

        // Convert seq from &[u8] into Vec<char>
        let reference_seq = reference.seq().iter().map(|b| *b as char).collect::<Vec<_>>();
        
        // Read in populations, barcodes
        for record in sequences_reader.records() {

            let parent = record.unwrap();

            // Debug
            //if parent.id() != "BJ.1" && parent.id() != "CJ.1" && parent.id() != "XBB.1.16" {continue}

            let id = String::from(parent.id());
            let alphabet = vec!['A', 'C', 'G', 'T'];
            let seq = parent.seq().iter().map(|b| *b as char).collect::<Vec<_>>();
            let mut genome = Genome::new(id, seq, alphabet);

            genome.parse_sequence(&reference_seq, *mask).unwrap();
            sequences.insert(genome.id.clone(), genome);
        }

        self.sequences = sequences;

        Ok(())
    }

    pub fn set_mutations(&mut self) -> Result<(), Report> {

        let mut mutations: HashMap<Mutation, Vec<String>> = HashMap::new();

        for (_id, genome) in &self.sequences {
            // Iterate through substitutions
            for sub in genome.substitutions.iter(){
                mutations.entry(*sub).or_insert(Vec::new());
                mutations
                    .get_mut(&sub)
                    .unwrap_or_else(|| panic!("Substitution not found in HashMap"))
                    .push((*genome.id).to_string())
            }
            // Iterate through deletions
            for del in genome.deletions.iter(){
                mutations.entry(*del).or_insert(Vec::new());
                mutations
                    .get_mut(&del)
                    .unwrap_or_else(|| panic!("Deletion not found in HashMap"))
                    .push((*genome.id).to_string())
            }
        }

        self.mutations = mutations;

        Ok(())

    }

    pub fn summarise_barcodes(&mut self, dataset: &Dataset) -> Result<(), Report> {


        for (_id, genome) in &mut self.sequences {
            let mutations = genome.substitutions.clone();

            // Compare genome to all population barcodes in the dataset
            genome.summarise_barcode(&dataset, &mutations).unwrap();
            debug!("{}", genome.summary());

            // Find the consensus population (best match)
            genome.consensus_population.search(
                &genome.substitutions, 
                &genome.missing, 
                &genome.total, 
                &dataset
            ).unwrap();
            debug!("{}", genome.consensus_population.summary());

            // Search for recombination parents if either there are conflicts,
            // or this is a known recombinant
            if genome.consensus_population.is_recombinant || genome.conflict_ref.len() > 0 {
                let mut exclude_populations: Vec<String> = Vec::new();
                let mut include_populations: Vec<String> = Vec::new();

                // Exclude descendants of the consensus from the parent search
                let descendants = dataset.phylogeny.get_descendants(&genome.consensus_population.population).unwrap();
                exclude_populations.extend(descendants);

                // Exclude ... conflict ref or alt?

                // Include populations that have the conflict alt
                let test = genome.conflict_alt.iter().filter(|(pop, count)| count > &&(1 as isize)).collect::<Vec<_>>();

                println!("exclude: {}", exclude_populations.join(", "));
                println!("include: {}", include_populations.join(", "));
            }
            break
        }

        Ok(())

    }

}
