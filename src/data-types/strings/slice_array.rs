// slice array

fn main() {
    let a = [1,2,3,4,5];

    let slice = &a[1..3];

    // to show how we slice
    for (_, item) in slice.iter().enumerate() {
        println!("item nedir : {}", item);
    }
}