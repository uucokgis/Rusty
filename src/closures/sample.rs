use std::{num, thread, time::Duration};
use rand::Rng;

extern crate rand;


fn main () {
    let simulated_user_specify_value = create_random_number(0, 10);
    let simulated_random_number = create_random_number(0, 10);
//    let random_number = create_random_number(0, 10);

    generate_workout(simulated_user_specify_value, simulated_random_number);
}


fn create_random_number(start: u32, end: u32) -> u32 {
    rand::thread_rng().gen_range(start, end)

}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly ..");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("Calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups", expensive_closure(intensity));
        println!("Next, do {} situps !", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today ! Remember to stay hydrated !");
        } else {
            println!("Today, run for {} minutes", expensive_closure(intensity));
        }
    }

    // another basic closure function
    let _closure_test = |samplenum: i32| -> i32{
        samplenum
    };

    /*
    Note that, If you don't specify type of the value, rust infer that type as type of first used
    line parameters. I mean if you use clouse function with a string for the first time, it has
    to be used with string type. Otherwise, compiler raises an error.
    */
}