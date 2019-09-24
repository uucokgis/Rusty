use add_lib;

fn main() {
    println!("Hello, world!");

    let result = add_lib::add_one(5);
    println!("result from add-lib : {}", result);
}
