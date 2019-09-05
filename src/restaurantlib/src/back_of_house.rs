mod back_of_house {
    pub mod cooking {
        pub fn fishing() {}

        pub fn aperatives() {}

        pub fn meze() {}
        // etc
    }

    fn cook_order() {
        cooking::aperatives();
    }
}