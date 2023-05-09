use std::{cmp::Ordering};

#[derive(Debug)]
pub struct Substitution {
    pub coord: isize,
    pub reference: char,
    pub alt: char,
}

impl std::fmt::Display for Substitution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}{}", self.reference, self.coord, self.alt)
    }
}


impl PartialEq for Substitution {

    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord && self.reference == other.reference && self.alt == other.alt
    }
}

impl Eq for Substitution { }

impl Ord for Substitution{

    fn cmp(&self, other: &Self) -> Ordering {
        self.coord.cmp(&other.coord)
    }

}

impl PartialOrd for Substitution{

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.coord.partial_cmp(&other.coord)
    }
   
}