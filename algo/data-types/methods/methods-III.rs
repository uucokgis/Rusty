// methods - III / Associated Functions, Multiple implementations (Impl)
#![allow(unused_variables)]

fn main() {
    #[derive(Debug)]

    struct Rectangle {
        width: u32,
        height : u32,
    }

    impl Rectangle {
        fn square(size: u32) -> Rectangle { // ben sanki bunu static method gibi düşünüyorum
            Rectangle{width: size, height: size}
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let sq = Rectangle::square(3); // çağırılması da böyle :: ile oluyor.

    let r1 = Rectangle{width: 5, height: 10};
    let r2 = Rectangle{width: 10, height: 5};

    let k = r1.can_hold(&r2);
    println!("k nedir : {}", k);
}