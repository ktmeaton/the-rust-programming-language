use std::io;

use crate::genome::genome::Genome;

#[derive(Debug)]
pub struct Dataset {
    genomes: Vec<Genome>,
}

impl Dataset {
    pub fn new(genomes: Vec<Genome>) -> Self {
        Dataset {
            genomes
        }
    }

    pub fn create_barcodes(&self) -> Result<(), io::Error> {

        let mut mutations = Vec::new();
    
        for genome in self.genomes.iter(){
            for sub in genome.substitutions.iter(){
                if !mutations.contains(&sub){
                    mutations.push(sub);
                }
            }
        }
        mutations.sort();

        println!("{:?}", mutations);

        // Convert to table

        Ok(())

    }  
}