use crate::reader::CSVReader;
use std::fmt;

#[derive(Debug)]
pub struct PathAnalyzer {
    path: String,
}

impl PathAnalyzer {
    pub fn new(path: String) -> PathAnalyzer {
        PathAnalyzer { path }
    }
}

// impl fmt::Debug for PathAnalyzer {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", &self.path)
//     }
// }