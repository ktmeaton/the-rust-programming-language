use std::collections::HashMap;

use log::debug;
use itertools::Itertools;
use eyre::Report;

use crate::traits::Summary;
use crate::mutation::Mutation;
use crate::dataset::sequences::Sequences;
use crate::dataset::Dataset;

#[derive(Debug)]
pub struct BarcodeMatch {
    consensus_population: String,
    top_populations: Vec<String>,
    barcode: Vec<Mutation>,
    support: Vec<Mutation>,
    missing: Vec<Mutation>,
    conflict_ref: Vec<Mutation>,
    conflict_alt: Vec<Mutation>,
}

impl BarcodeMatch {
    pub fn new() -> Self {
        BarcodeMatch {
            consensus_population: String::new(),
            top_populations: Vec::new(),
            barcode: Vec::new(),
            support: Vec::new(),
            missing: Vec::new(),
            conflict_ref: Vec::new(),
            conflict_alt: Vec::new(),            
        }
    }

    pub fn search(&mut self, mutations: &Vec<Mutation>, missing: &Vec<isize>, barcode_summary : &HashMap<String, isize>, dataset: &Dataset) -> Result<(), Report> {

        let mut max_total = 0 as isize;

        for (_population, total) in barcode_summary {
            if total >= &max_total{
                max_total = *total;
            }
        }
    
        // --------------------------------------------------------------------    
        // Search for top_populations and consensus_population   

        for (population, count) in barcode_summary{
            if count >= &max_total{
                self.top_populations.push(population.to_string());
            }
        }

        // If we don't have a phylogeny, just take first top_populations
        if dataset.phylogeny.is_empty() {
            self.consensus_population = self.top_populations[0].clone();
        }
        else {
            println!("top_populations: {:?}", self.top_populations);
            self.consensus_population = self.top_populations[0].clone();
        }

        self.barcode = dataset.populations.sequences[&self.consensus_population].substitutions.clone();

        for sub in &self.barcode{
            if mutations.contains(&sub){
                self.support.push(*sub);
            }
            else if missing.contains(&sub.coord){
                self.missing.push(*sub);
            }
            else {
                self.conflict_ref.push(*sub);
            }
        }

        for sub in mutations{
            if !self.barcode.contains(&sub){
                self.conflict_alt.push(*sub);
            }
        }  

        debug!("\n{}", self.summary());
        
        Ok(())
    }
}

impl Summary for BarcodeMatch {
    fn summary(&self) -> String {
        format!(
            "consensus_population: {}\ntop_populations: {}\nbarcode: {}\nmissing: {}\nconflict_ref: {}\nconflict_alt: {}\n",
            self.consensus_population,
            self.top_populations.iter().join(", "),
            self.barcode.iter().join(", "),
            self.missing.iter().join(", "),
            self.conflict_ref.iter().join(", "),
            self.conflict_alt.iter().join(", "),
        )
    }
}