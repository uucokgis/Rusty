// struct - III


fn main() {
    // tuple structs
    struct Color(i32, i32, i32);
    struct  Point(i32, i32, i32);

    let black = Color(3,5,12);
    let corner = Point(3, 3, 3);

    println!("black one : {}", black.0);
    println!("corner last : {}", corner.2);
}