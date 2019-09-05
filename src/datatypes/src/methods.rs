pub mod basics {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // implementation block
        fn area(&self) -> u32 { // & kullanmak zorundayız. Çünkü ownership'liği almak istemiyoruz.
            // ayrıca struct'ı sadece okuyacağız ona bir şey yazmayacağız.
            // ayrıntılı bilgi için Rusty II / Methods - I bakınız.
            self.width * self.height
        }
    }

    pub fn run() {
        let rect1 = Rectangle { width: 30, height: 50 };

        println!("Bu alan : {} kadardır.", rect1.area());
    }
}

pub mod adv1 {
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

    fn run() {
        let rect1 = Rectangle{width: 30, height: 50};
        let rect2 = Rectangle{width: 10, height: 40};
        let rect3 = Rectangle{width: 15, height: 15};

        println!("rect1 rect2'yi tutabilir mi : {}", rect1.can_hold(&rect2));
        println!("rect1 rect3'yi tutabilir mi : {}", rect2.can_hold(&rect3));

        let mean: bool = rect1.can_hold(&rect2);
        println!("mean nedir : {}", mean);
    }
}

pub mod adv2 {
    fn run() {
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn square(size: u32) -> Rectangle { // ben sanki bunu static method gibi düşünüyorum
                Rectangle { width: size, height: size }
            }
        }

        impl Rectangle {
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }

        let sq = Rectangle::square(3); // çağırılması da böyle :: ile oluyor.

        let r1 = Rectangle { width: 5, height: 10 };
        let r2 = Rectangle { width: 10, height: 5 };

        let k = r1.can_hold(&r2);
        println!("k nedir : {}", k);
    }
}

pub mod methods_crashcourse {
    pub fn run () {
        greeting("umut", "aslan");

        let name = String::from("Rick");
        let name2 = String::from("Morty");

        greeting(&name, &name2);

        println!("5 + 12 : {}", add(5, 12));

        // Closure    -- GOOD ONE
        let n3 = 21;
        let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;

        println!("sum : {}", add_nums(3, 12));
    }

    fn greeting(greet: &str, name: &str) {
        println!("{} {} , nice to meet you", greet, name);
    }

    fn add(numberone: i32, numbertwo: i32) -> i32 {
        numberone + numbertwo
    }


}