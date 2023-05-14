pub mod sequences;
pub mod phylogeny;

pub struct Dataset {
    pub populations: sequences::Sequences,
    pub phylogeny: phylogeny::Phylogeny,
}

impl Dataset {
    pub fn new() -> Self {
        Dataset {
            populations : sequences::Sequences::new(),
            phylogeny: phylogeny::Phylogeny::new(),
        }
    }
}