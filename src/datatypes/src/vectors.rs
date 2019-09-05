#[allow(dead_code)]
/*
Vectors are defined with Vec<datatype> and Vec::new(); statements.
Dereferencing also here, see : from_the_book::iterating()

They are useful when you have a list of items, such as the lines of text in a file or the
prices of items in a shopping cart.
*/

pub mod basics {
    pub fn run () {
        // initializing
        let vsample: Vec<i32> = Vec::new();
    }
}
pub mod vectors_crashcourse {
    // can be growable

    pub fn run () {
        let mut numbers: Vec<i32> = vec![1,2,3,4,5];

        // get single value
        println!("first value : {}", numbers[0]);

        // re-assign value
        numbers[3] = 10;

        println!("numbers : {:?}", numbers);

        // length of Vector
        println!("numbers length : {}", numbers.len());

        // Vector are stack allocated
        println!("Vector populates {} bytes", std::mem::size_of_val(&numbers));  // mutable ones and this are same. Array was 20 bytes and vector is 24 bytes
        //  And you don't have to put 'use std::' due to built in function.

        // Get slice
        let slice: &[i32] = &numbers[0..2];  // you can take whole array also.
        println!("slice : {:?}", slice);

        // pushing some numbers
        numbers.push(55);

        println!("pushed number vectors : {:?}", numbers);
        // popping last value
        numbers.pop();
        println!("popped number vectors : {:?}", numbers);

        // loop through vector values
        for x in numbers.iter() {
            println!("Number : {}", x);
        }

        // looping and mutating same time
        for x in numbers.iter_mut() {
            *x *= 2;
            *x += 5;
        }
        println!("mutating number vectors : {:?}", numbers);
    }
}

pub mod from_the_book {
    pub fn run () {
        // initializing
        let mut v: Vec<i32> = Vec::new();

        // Creating and Assigning
        let v2 = vec![1,2,3];

        // Updating vector
        v.push(4);  // It has to be mutable as well

        {
            // dropping object like any other types
            let v3test = vec![1,2,3,4,5];
        }
//        println!("v3:  {:?} ", v3test);

        // Reading data
        // Two ways to get the third element are by using & and [], which gives us a reference,
        let third: &i32 = &v[2];
        println!("Third element is : {}", third);

        // Other way is using match
        match v.get(2) {
            Some(third) => println!("The third element is : {}", third),
            None => println!("There is no third element .")
        }

        // There is good behaviour like dict[1] and dict.get(1) in Python:
        let v4 = vec![1,2,3,4,5];
        // println!("100th element of v4 : {}", &v4[100]);  // this code buggy
        println!("100th element of v4 : {:?}", &v4.get(100));
        // or
        println!("{:?}", v4.get(100));
    }

    pub fn borrowing_and_vectors () {
        let v = vec![1,2,3,4,5,6];

        let first = &v[0];
//        v.push(7);

        println!("The first element is : {}", first);  // error occurs when i type this.
    }

    pub fn iterating () {
        println!("ITERATING A VECTOR");
        let v = vec![1,2,3,4,5];
        for i in &v {
            println!("{}", i);
        }

        // iterating with changing
        let mut v2 = vec![1,2,3,4,5,6];
        for i in &mut v2 {
            *i += 50;
        }
    }

    pub fn storing_different_types_with_enum () {
        enum DataTypes {
            Int(i32),
            Float(f32),
            Text(String)
        }

        let vectors: vec<DataTypes> = vec! [
        DataTypes::Float(4.12312),
        DataTypes::Int(5),
        DataTypes::Text(String::from("Naber"))
        ];
    }
}