// Cache'leme mekanizması örnek

use std::thread::sleep;
use std::time::Duration;

struct Cacher<T> where T: Fn(u32, u32) -> u32 {
    hesaplayan: T,
    deger: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32, u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            hesaplayan: calculation,
            deger: None,
        }
    }

    fn value(&mut self, new_value: u32, stored_value: u32) -> u32 {
        match self.deger {
            Some(v) => v,
            None => {
                let v = (self.hesaplayan)(new_value, stored_value);
                self.deger = Some(v);
                v
            }
        }
    }
}


fn main() {
    fn generate_workout(intensity: u32, random_number: u32) {
        let mut egzersiz = Cacher::new(|num, stored_num| {
            println!("Hesaplanıyor ...");
            sleep(Duration::from_secs(2));
            num
        });

        if intensity < 25 {
            println!("Today {} pushups", &egzersiz.value(intensity, random_number));
        } else {
            if random_number == 3 {
                println!("Enough today !");
            } else {
                println!("Today run {} minutes", &egzersiz.value(intensity, random_number));
            }
        }
    }

    let intensity = 27;
    let random_number = 3;

    generate_workout(intensity, random_number);
}