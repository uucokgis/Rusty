pub mod basics {
    // Tuples group together values of different types
    // Max 12 elements !
    // i8 - 0-127 8 bits.

    pub fn run () {
        let person: (&str, &str, i8) = ("Brad", "Mass", 37);
        let personv2: (String, i32) = (String::from("Umut"), 26);

        println!("{} is from {} and is {} age", person.0, person.1, person.2);
        println!("{} is {} age from whole string", personv2.0, personv2.1);
    }
}