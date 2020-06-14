use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        println!("MyBox dereffed");
        &self.0
    }

}
fn main() {
    fn hello(x: &str) {
        println!("Hello , {}", x);
    }

    let mut m = MyBox(String::from("Umut "));
    hello(&m);
    hello(&(*m));

}