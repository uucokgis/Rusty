mod importer;
mod dining;

/*
todo: But I cannot figure out how to call packaging/restaurant module
*/

use crate::importer::Thing as Sey;
use std::fs::File;  // use kullanımına bi örnek

fn main() {
    println!("Hello, world!");

    // importer test
    let a = Sey::new();
    println!("a: {:?}", a);
    // dining test
    let b = dining::back_of_house::Breakfast::summer("kaşarlı");
    println!("b: {:?}", b.toast);

    let f = File::open("fasd.txt").unwrap();
}
