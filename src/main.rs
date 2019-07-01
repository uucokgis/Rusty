#![allow(unused_variables)]

// < >

fn main() {
    let myarray = [1,2,3,4,5];
    let myslice: &[i32] = &myarray[0..5];

    for i in myslice.iter() {
        println!("my i : {}", i);
    }
}


