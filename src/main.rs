fn main() {
    let some_u8_value = None;
    let mut l: i32 = 0; // it has to be initialized
    if let Some(mut l) = some_u8_value {
        l = 3;
    }

    if let None = some_u8_value {
        l = 5;
    }
    println!("l : {}", l);
}