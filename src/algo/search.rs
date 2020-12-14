use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SearchType {
    Normal,
    Greedy,
    Uniform,
}

impl fmt::Display for SearchType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
