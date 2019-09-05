use std::{num, thread, time::Duration};
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

    if intensity < 25 {
        println!("Today, do {} pushups", expensive_result);
        println!("Next, do {} situps !", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today ! Remember to stay hydrated !");
        } else {
            println!("Today, run for {} minutes", expensive_result);
        }
    }
}