use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;


struct UmutCacher<T, U, V>
    where T: Fn(U) -> U
{
    calculation: T,
    value: Option<HashMap<U, V>>
}

impl <T, U, V> UmutCacher <T, U, V>
    where T: Fn(U) -> U

{
    fn new(calculation: T, arg: U) -> UmutCacher<T, U, V> {
        UmutCacher
            {
                calculation: calculation,
                value: None
            }

    }

    fn value <K, L> (&mut self, arg: u32) -> &HashMap<U, V>
    {

        match &self.value {
            Some(v) => &v,
            None => {
                // I cannot handle with these things:
                let mut myhash:HashMap<U, V> = HashMap::new();
                myhash.insert(arg, (self.calculation)(arg));
                self.value = myhash;
                myhash
            }
        }
    }

}

fn main () {

}