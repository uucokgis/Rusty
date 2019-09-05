use std::{thread, time::Duration};
use rand::Rng;
use std::collections::HashMap;

extern crate rand;


struct Cacher <T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: HashMap<u32, u32>
}

impl <T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T>
    {
        Cacher
            {
                calculation,
                value: HashMap::new()
            }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg)
            {
                Some(&v) => v,
                None =>
                    {
                        let v = (self.calculation)(arg);
                        self.value.insert(arg, v);
                        v
                    }
            }
    }
}


fn create_random_number(start: u32, end: u32) -> u32 {
    rand::thread_rng().gen_range(start, end)

}

fn _simulated_expensive_calculation(intensity: u32) -> u32 {
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

fn generate_workout_new(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25
    {
        println!("Today, do {} pushups !", expensive_result.value(intensity));
        println!("Next, do {} situps !", expensive_result.value(intensity));
    }

    else
    {
        if random_number == 3
        {
            println!("Take a break today !");
        }
        else
        {
            println!("Today, run for {} minutes", expensive_result.value(intensity));
        }
    }
}


fn main () {
    let simulated_user_specify_value = create_random_number(2, 10);
    let simulated_random_number = create_random_number(2, 10);
//    let random_number = create_random_number(0, 10);

//    generate_workout(simulated_user_specify_value, simulated_random_number);
    generate_workout_new(simulated_user_specify_value, simulated_random_number);
}


#[test]
fn call_with_different_values () {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 2);
    assert_eq!(v1, 1);
}