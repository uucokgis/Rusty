mod basics {
    // tuple structs are hybrid data type between structs and tuples

    // unit like structs are little bit different from tuple structs.
    // they are empty structs in fact.
    /*
    Unit structs are only useful as trait objects.
    Why should i use this?
    For instance, a library may ask you to create a structure that implements a certain trait to
    handle events. If you don’t have any data you need to store in the structure, you can create a
    unit-like struct.
    https://github.com/rust-lang/rust-by-example/issues/391
    */
}

mod samples {
    struct Color (i32, i32, i32); // they dont have named properties opposite to structs
    pub fn sampleone() {
        let black = Color(0, 0, 0);
    }

    struct Electron;
    pub fn unit_structs() {
        let x = Electron;
    }
}