// fixed length. you can use vector to grow your datalist

pub mod arrays {
    pub fn basics() {
        let numbers: [i32; 5] = [1, 2, 3, 4, 5];
        let mut numbers2: [i32; 5] = [5, 4, 3, 2, 1];  // we can change items

        // get single value
        println!("first value : {}", numbers[0]);

        // re-assign value
        numbers2[3] = 10;

        println!("numbers2 : {:?}", numbers2);

        // length of array
        println!("numbers length : {}", numbers.len());

        // Arrays are stack allocated
        println!("Array populates {} bytes", std::mem::size_of_val(&numbers));  // mutable ones and this are same.
        //  And you don't have to put 'use std::' due to built in function.

        // Get slice
        let slice: &[i32] = &numbers[0..2];  // you can take whole array also.
        println!("slice : {:?}", slice);
    }

    pub fn slicing () {
        let a = [1,2,3,4,5];

        let slice = &a[1..3];

        // to show how we slice
        for (_, item) in slice.iter().enumerate() {
            println ! ("item nedir : {}", item);
        }
    }

    // iterating a list and finding the biggest number. It works with vector also
    fn find_largest(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item
            }
        }
        largest

    }

}