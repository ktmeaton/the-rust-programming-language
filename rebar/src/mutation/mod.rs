use std::{cmp::Ordering};

#[derive(Copy, Clone, Debug, Hash)]
pub struct Mutation {
    pub coord: isize,
    pub reference: char,
    pub alt: char,
}

impl std::fmt::Display for Mutation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}{}", self.reference, self.coord, self.alt)
    }
}

impl PartialEq for Mutation {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord && self.reference == other.reference && self.alt == other.alt
    }
}

impl Eq for Mutation { }

impl Ord for Mutation{
    fn cmp(&self, other: &Self) -> Ordering {
        self.coord.cmp(&other.coord)
    }
}

impl PartialOrd for Mutation{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.coord.partial_cmp(&other.coord)
    }
}