// struct examples
// how we use them
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50}; // instance of rectangle struct

    println!("Area of this rectangle is : {}", area(&rect1));

    println!("rect1 : {:?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
