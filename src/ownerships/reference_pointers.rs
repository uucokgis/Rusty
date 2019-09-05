pub mod basics {
    // Reference pointers - Point to a resource in memory

    pub fn run () {
        // Primitive values has to be
        let arr1: [i32; 5] = [1,2,3,4,5];
        let arr2 = arr1;  // ownership did not moved to arr2 from arr1 due to primitive

        println!("values of arr2 : {:?}", arr2);

        // so lets try for vectors which is not primitive data
        let myvec1: Vec<i32> = vec![1,2,3,4,5];
        println!("my vector : {:?}", myvec1);
        let myvec2 = &myvec1;  // ownership moved to myvec2 from myvec1
        println!("my vector : {:?}", myvec2);
    }
}