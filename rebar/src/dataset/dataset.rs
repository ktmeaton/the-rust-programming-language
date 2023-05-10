// Standard library
use std::io;
use std::collections::HashMap;

use eyre::Report;

// This crate
use crate::genome::genome::Genome;
use crate::mutation::substitution::Substitution;

#[derive(Debug)]
pub struct Dataset {
    genomes: HashMap<String,Genome>,
    mutations: HashMap<Substitution, Vec<String>>
}

impl Dataset {
    pub fn new(genomes: HashMap<String,Genome>) -> Self {
        let mut dataset = Dataset {
            genomes,
            mutations : HashMap::new(),
        };
        dataset.set_mutations().unwrap();
        dataset
    }

    pub fn set_mutations(&mut self) -> Result<(), Report> {

        let mut mutations: HashMap<Substitution, Vec<String>> = HashMap::new();

        for (id, genome) in &self.genomes{
            for sub in genome.substitutions.iter(){
                if !mutations.contains_key(&sub){
                    mutations.insert(*sub, Vec::new());
                }
                mutations
                    .get_mut(&sub)
                    .unwrap_or_else(|| panic!("Substitution not found in HashMap"))
                    .push((*genome.id).to_string())
            }
        }

        self.mutations = mutations;

        Ok(())

    }  
}