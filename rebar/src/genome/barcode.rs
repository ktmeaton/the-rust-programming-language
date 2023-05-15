use std::collections::HashMap;

use log::debug;
use itertools::Itertools;
use eyre::Report;

use crate::traits::Summary;
use crate::mutation::Mutation;
use crate::dataset::Dataset;

#[derive(Debug)]
pub struct BarcodeMatch {
    pub population: String,
    pub top_populations: Vec<String>,
    pub barcode: Vec<Mutation>,
    pub support: Vec<Mutation>,
    pub missing: Vec<Mutation>,
    pub conflict_ref: Vec<Mutation>,
    pub conflict_alt: Vec<Mutation>,
    pub recombinant: String,
    pub is_recombinant: bool,
    pub recursive: bool,
}

impl BarcodeMatch {
    pub fn new() -> Self {
        BarcodeMatch {
            population: String::new(),
            top_populations: Vec::new(),
            barcode: Vec::new(),
            support: Vec::new(),
            missing: Vec::new(),
            conflict_ref: Vec::new(),
            conflict_alt: Vec::new(),
            recombinant: String::new(),
            is_recombinant: false,
            recursive: false,
        }
    }

    pub fn search(
        &mut self, 
        mutations: &Vec<Mutation>, 
        missing: &Vec<isize>, 
        barcode_total : &HashMap<String, isize>, 
        dataset: &Dataset) 
        -> Result<(), Report> {

        let mut max_total = 0 as isize;

        for (_population, total) in barcode_total {
            if total >= &max_total{
                max_total = *total;
            }
        }
    
        // --------------------------------------------------------------------    
        // Search for top_populations and population   

        for (population, count) in barcode_total{
            if count >= &max_total{
                self.top_populations.push(population.to_string());
            }
        }

        // If we don't have a phylogeny, just take first top_populations
        if dataset.phylogeny.is_empty() {
            self.population = self.top_populations[0].clone();
        }
        // Otherwise, consensus is mrca of top
        else {
            self.population = dataset.phylogeny.get_common_ancestor(&self.top_populations).unwrap();
            self.set_recombinant_status(dataset).unwrap();
        }

        self.barcode = dataset.populations.sequences[&self.population].substitutions.clone();

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

    pub fn set_recombinant_status(&mut self, dataset: &Dataset) -> Result<(), Report>  {

        let recombinants = dataset.phylogeny.get_recombinants().unwrap();
        for recombinant in recombinants {
            let descendants = &dataset.phylogeny.get_descendants(&recombinant).unwrap();
            if descendants.contains(&self.population){
                self.recombinant = recombinant.clone();
                self.is_recombinant = true;
            }            
        }      
        Ok(())

    }
}

impl Summary for BarcodeMatch {
    fn summary(&self) -> String {
        format!(
            "\n
            population: {}
            recombinant:          {}
            recursive:            {}
            top_populations:      {}
            barcode:              {}
            missing:              {}
            conflict_ref:         {}
            conflict_alt:         {}
            ",
            self.population,
            self.recombinant,
            self.recursive,            
            self.top_populations.iter().join(", "),
            self.barcode.iter().join(", "),
            self.missing.iter().join(", "),
            self.conflict_ref.iter().join(", "),
            self.conflict_alt.iter().join(", "),
        )
    }
}