use itertools::Itertools;

use crate::mutation::substitution::Substitution;

#[derive(Debug)]
pub struct Genome {
    pub id: String,
    pub seq: Vec<char>,
    pub alphabet: Vec<char>,
    pub missing: Vec<isize>,
    pub deletions: Vec<isize>,
    pub substitutions: Vec<Substitution>,
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
        }
    }

    pub fn add_missing(&mut self, coord: isize) {
        self.missing.push(coord);
    }
    pub fn add_deletion(&mut self, coord: isize) {
        self.deletions.push(coord);
    }
    pub fn add_substitution(&mut self, sub: Substitution) {
        self.substitutions.push(sub);
    }

    pub fn parse_sequence(&mut self, reference_seq: &Vec<char>, mask: isize){

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

            // Decide if missing, deletion, sub or reference
            match s {
                'N' => self.missing.push(coord),
                '-' => self.deletions.push(coord),
                s if s != r => {
                    let sub = Substitution {
                        coord: coord,
                        reference: r,
                        alt: s,
                    };
                    self.substitutions.push(sub);
                }
                _ => continue,
            }
        }

        // Clear sequence after parsing
        self.seq = Vec::new();
    }
}

pub trait Summary {
    fn summary(&self) -> String;
}

impl Summary for Genome {
    fn summary(&self) -> String {
        format!(
            //"id: {}\nseq: {}\nalphabet: {}\nmissing: {}\ndeletions: {}\nsubstitutions: {}",
            "id: {}\nalphabet: {}\nmissing: {}\ndeletions: {}\nsubstitutions: {}",
            self.id,
            //self.seq.iter().format(""),
            self.alphabet.iter().format(", "),
            self.missing.iter().format(", "),
            self.deletions.iter().format(", "),
            self.substitutions.iter().format(", "),
        )
    }
}
