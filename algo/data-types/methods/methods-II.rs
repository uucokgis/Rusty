// methods - II
#![allow(unused_variables)]
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // implementation block
    fn area(&self) -> u32 { // & kullanmak zorundayız. Çünkü ownership'liği almak istemiyoruz.
        // ayrıca struct'ı sadece okuyacağız ona bir şey yazmayacağız.
        // ayrıntılı bilgi için Rusty II / Methods - I bakınız.
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle{width: 30, height: 50};
    let rect2 = Rectangle{width: 10, height: 40};
    let rect3 = Rectangle{width: 15, height: 15};

    println!("rect1 rect2'yi tutabilir mi : {}", rect1.can_hold(&rect2));
    println!("rect1 rect3'yi tutabilir mi : {}", rect2.can_hold(&rect3));

    let mean: bool = rect1.can_hold(&rect2);
    println!("mean nedir : {}", mean);

}