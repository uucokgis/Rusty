#![allow(unused_variables)]

// < >

fn main () {
    println!("Hello canım");

    let c = f_to_c_converter(68.2);
    println!("c nedir : ", c);
}

fn f_to_c_converter(fahrenheit : f64) {
    (fahrenheit - 32) / 180;
}