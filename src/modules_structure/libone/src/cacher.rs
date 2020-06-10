// Cache'leme mekanizması örnek

use std::thread::sleep;
use std::time::Duration;

pub mod caching {
    pub struct Cacher<T> where T: Fn(u32, u32) -> u32 {
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
}