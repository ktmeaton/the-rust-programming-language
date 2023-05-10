use std::collections::HashMap;

use log::debug;
use itertools::Itertools;
use eyre::Report;

use crate::dataset::Dataset;
use crate::mutation::Mutation;
use crate::traits::Summary;
use crate::barcode::BarcodeMatch;

#[derive(Debug)]
pub struct Genome {
    pub id: String,
    pub seq: Vec<char>,
    pub alphabet: Vec<char>,
    pub missing: Vec<isize>,
    pub deletions: Vec<Mutation>,
    pub substitutions: Vec<Mutation>,
    pub consensus_population: BarcodeMatch,
}

impl std::fmt::Display for Genome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Genome {
    pub fn new(id: String, seq: Vec<char>, alphabet: Vec<char>) -> Self {
        Genome {
            id,
            seq,
            alphabet,
            missing: Vec::new(),
            deletions: Vec::new(),
            substitutions: Vec::new(),
            consensus_population: BarcodeMatch::new(),
        }
    }

    pub fn parse_sequence(&mut self, reference_seq: &Vec<char>, mask: isize) -> Result<(), Report>  {

        let genome_length = reference_seq.len() as isize;

        // Construct iterator to traverse sample and reference bases together
        let it = self.seq.iter().zip(reference_seq.iter());
        for (i, (s, r)) in it.enumerate() {
            // Genomic coordinates are 1-based
            let coord: isize = (i + 1) as isize;
            // Mask 5' and 3' ends
            let mut s = s.clone();
            let r = r.clone();
            if coord <= mask || coord > genome_length - mask {
                s = 'N';
            }

            let mutation = Mutation {
                coord: coord,
                reference: r,
                alt: s,
            };

            match s {
                // Missing data (N)
                'N' => self.missing.push(coord),
                // Deletions              
                '-' => self.deletions.push(mutation),
                // Ambiguous data (IUPAC not in alphabet)
                s if s != r && !self.alphabet.contains(&s) => self.missing.push(coord),                
                // Substitution   
                s if s != r => self.substitutions.push(mutation),
                // Reference
                _ => continue,
            }
        }

        // Clear sequence after parsing
        self.seq = Vec::new();

        Ok(())
    }

    pub fn summarise_barcode(&mut self, dataset : &Dataset, mutations: &Vec<Mutation>) -> Result<(), Report>  {

        debug!("sequence: {}", self.id);
    
        // support: Mutation in genome is present in population's barcode.
        // conflict_alt: Mutation in genome is absent in population's barcode.
        // conflict_ref: Mutation in population's barcode is absent in genome.
        // private: Private mutations only found in genome       
        // barcode_summary: final stat, support - conflict_ref
        let mut support: HashMap<String, isize> = HashMap::new();
        let mut conflict_alt: HashMap<String, isize> = HashMap::new();
        let mut conflict_ref: HashMap<String, isize> = HashMap::new();
        let mut private: Vec<Mutation> = Vec::new();
        let mut barcode_summary: HashMap<String, isize> = HashMap::new();

        // support
        for mutation in mutations {
            // Barcode match
            if dataset.mutations.contains_key(mutation){

                let population_matches = dataset.mutations[mutation].clone();

                for population in population_matches{
                    *support.entry(population).or_insert(0) += 1;
                }
            } 
            // Private mutation
            else {
                private.push(mutation.clone())
            }
        }

        // conflict_ref, conflict_alt, and total
        for population in support.keys() {
            let population_sub = &dataset.sequences[population].substitutions;

            // conflict_ref
            let population_conflict_ref = population_sub
                .iter()
                .filter(|sub| !self.substitutions.contains(sub))
                .collect::<Vec<_>>();
            let num_conflict_ref = population_conflict_ref.len() as isize;
            conflict_ref.insert(population.clone(), num_conflict_ref);

            // conflict_alt
            let population_conflict_alt = self.substitutions
                .iter()
                .filter(|sub| !population_sub.contains(sub))
                .collect::<Vec<_>>();
            let num_conflict_alt = population_conflict_alt.len() as isize;
            conflict_alt.insert(population.clone(), num_conflict_alt);

            // total
            let num_total = support[population] - num_conflict_ref;
            barcode_summary.insert(population.clone(), num_total);

        }        
    
        let mut barcode_match = BarcodeMatch::new();
        barcode_match.search(&mutations, &self.missing, &barcode_summary, &dataset).unwrap();

        Ok(())
    }    

}

impl Summary for Genome {
    fn summary(&self) -> String {
        format!(
            "id: {}\nmissing: {}\ndeletions: {}\nsubstitutions: {}",
            self.id,
            self.missing.iter().format(", "),
            self.deletions.iter().format(", "),
            self.substitutions.iter().format(", "),
        )
    }
}
