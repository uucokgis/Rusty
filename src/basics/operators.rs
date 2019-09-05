// mathematic operations and booleans in rust

pub mod operators_basics {
    fn main() {
        let sum = 5 + 10;

        let difference = 95.5 - 43.2;

        let multiple = 4 * 30;

        let divider = 56.7 / 32.2;

        let remainder = 43 % 5;

        let t = true; // yea ide put this ":bool" but you dont have to say bool.

        let f: bool = false;

        let heart_eyed_cat = '😻'; // ahah. rust support even emoji
    }
}

pub mod printers {
    pub fn run () {
        // print to console
        println!("hello from the print.rs file !");

        // Basic formatting
        println!("{} is from {}", "Umut", "Çorum");

        // Positional arguments
        println!("{0} is from {1} and likes {2}", "Umut", "Çorum", "programming");

        // Named arguments
        println!("{isim} likes Rust programming for {yil} years", isim="Umut", yil=5);  // it can be string or integer. Anything that print trait can run

        // Placeholder traits
        println!("Binary : {:b}  ; Hex : {:x}  ;  Octal : {:o}", 10, 10, 10);

        // Placeholder for debug trait
        println!("I want to see my array for debug purpose{:?}", (1,2,true,"hello",5));

        // Basic math
        println!("{} + {} = {}", 10, 10, 10 + 10);
    }
}