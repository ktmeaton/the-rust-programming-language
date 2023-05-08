use bio::io::fasta;
use csv;
use itertools::Itertools;

trait Summary {
    fn summary(&self) -> String;
}

struct Genome {
    id: String,
    missing: Vec<usize>,
    deletions: Vec<usize>,
    substitutions: Vec<Substitution>,
}

impl Genome {
    fn new(id: String) -> Self {
        Genome {
            id,
            missing: Vec::new(),
            deletions: Vec::new(),
            substitutions: Vec::new(),
        }
    }

    fn add_missing(&mut self, coord: usize) {
        self.missing.push(coord);
    }
    fn add_deletion(&mut self, coord: usize) {
        self.deletions.push(coord);
    }
    fn add_substitution(&mut self, sub: Substitution) {
        self.substitutions.push(sub);
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

impl std::fmt::Display for Genome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

struct Substitution {
    coord: usize,
    reference: char,
    alt: char,
}

impl std::fmt::Display for Substitution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}{}", self.reference, self.coord, self.alt)
    }
}

fn main() {
    // Import data sources: 
    //  1. reference, query sequences
    //  2. populations, barcodes or alignment?

    let mask = 0;

    // Read in reference and query sequences
    let reference_path = "data/reference.fasta";
    let sequences_path = "data/sequences.fasta";

    let mut reference_reader = fasta::Reader::from_file(reference_path).unwrap();
    let mut sequences_reader = fasta::Reader::from_file(sequences_path).unwrap();

    let reference = reference_reader.records().next().unwrap().unwrap();
    let genome_length = reference.seq().len();

    // Read in populations, barcodes
    // genome.parse_sequence();
    for record in sequences_reader.records() {
        let sample = record.unwrap();
        let it = sample.seq().iter().zip(reference.seq().iter());

        let mut genome = Genome::new(
            sample.id().to_string(),
            //sample.seq().to_string(),
        );

        for (i, (s, r)) in it.enumerate() {
            // Genomic coordinates are 1-based
            let coord = i + 1;
            // Convert from &u8
            let mut s = *s as char;
            let r = *r as char;

            // Mask 5' and 3' end
            if coord <= mask || coord > genome_length - mask {
                s = 'N';
            }

            // Decide if missing, deletion, sub or other
            match s {
                'N' => genome.add_missing(coord),
                '-' => genome.add_deletion(coord),
                s if s != r => {
                    let sub = Substitution {
                        coord: coord,
                        reference: r,
                        alt: s,
                    };
                    genome.add_substitution(sub);
                }
                _ => continue,
            }
        }
        println!("{}", genome.summary());
    }

    // genome.barcode_search()
    let barcodes_path = "data/barcodes.csv";
    let mut barcodes_reader = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_path(barcodes_path).unwrap();
    let headers = barcodes_reader.headers().unwrap();
    println!("{:?}", headers);
    for result in barcodes_reader.records(){
        let record = result.unwrap();
        println!("{:?}", record);
    }

}
