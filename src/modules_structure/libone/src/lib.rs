pub mod libonemod {
    pub struct MyStruct {
        fieldone: String,
        fieldtwo: String
    }

    impl MyStruct {
        pub fn new(fieldone: String, fieldtwo: String) -> MyStruct {
            MyStruct {
                fieldone,
                fieldtwo
            }
        }
    }
    
    
}