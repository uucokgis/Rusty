mod threedim;
use excel_importer::excel::ExcelManager;

fn main() {
    let t = threedim::Triangle::new(1.0, 2.2, 3.1);
    println!("t: {:?}", t);

    let a = ExcelManager::new(String::from("asdasd"));
    println!("a: {:?}", a);
}
