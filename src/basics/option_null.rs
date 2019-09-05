fn main () {
    let a = 5;
    let b = Some(3);
    let c = None;
}

fn plus_one(x: Option<i32>) {
    match x {
        Some(x) => Some(x + 1),
        None => x
    }
}