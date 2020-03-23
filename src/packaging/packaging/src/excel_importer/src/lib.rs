pub mod analyzer;
pub mod reader;


pub mod excel {
    use std::fmt;
    pub use crate::analyzer::PathAnalyzer;
    pub use crate::reader::CSVReader;

    #[derive(Debug)]
    pub struct ExcelManager {
        r: CSVReader,
    }

    impl ExcelManager {
        pub fn new(path: String) -> ExcelManager {
            let p = PathAnalyzer::new(path);
            let r = CSVReader::new(p);

            ExcelManager { r }
        }
    }

    // impl fmt::Debug for ExcelManager {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         let p = &self.r.path;
    //         write!(f, "{}", p)
    //     }
    // }
}
