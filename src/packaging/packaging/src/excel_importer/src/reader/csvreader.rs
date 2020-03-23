use crate::analyzer::PathAnalyzer;

#[derive(Debug)]
pub struct CSVReader {
    pub path: PathAnalyzer,
}

impl CSVReader {
    pub fn read_excel() {}

    pub fn new(p: PathAnalyzer) -> CSVReader {
        CSVReader { path: p }
    }
}
