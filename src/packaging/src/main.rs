mod importer;
mod restaurant;

use crate::importer::Thing;  // use kullanımına bi örnek

fn main() {
    println!("Hello, world!");

    // importer test
    let a = Thing::new();
    println!("a: {:?}", a);

    // restaurant test
    let b = restaurant::back_of_house::Breakfast::summer("kaşarlı");
    println!("b: {:?}", b.toast);
}
