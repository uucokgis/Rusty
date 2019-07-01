// methods
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
}

fn main() {
    let rect1 = Rectangle{width: 30, height: 50};

    println!("Bu alan : {} kadardır.", rect1.area());
}