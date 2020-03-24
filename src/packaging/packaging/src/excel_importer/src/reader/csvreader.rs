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

mod tests {
    use super::*;
    use std::fmt::Error;

    #[test]
    // #[should_panic(expected = "G")]
    fn read_test() {
        let path = "/home/umut/CLionProjects/Rusty/src/packaging/packaging/src/excel_importer\
        /test_data/iotDevices.csv";
        let p_analyzer: PathAnalyzer = PathAnalyzer::new(path.to_string());
        let c_reader: CSVReader = CSVReader::new(p_analyzer);

        println!("{:?}", c_reader);
        panic!("G");
    }

    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

}