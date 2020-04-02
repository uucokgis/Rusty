// lets create an iterator for your object

fn main () {
    let mut c = Counter::new(Some(5));
    let mut c2 = Counter::new(None);

    println!("c next : {:?}", c.next());
    println!("c next : {:?}", c.next());
    println!("c next : {:?}", c.next());

    println!("c2 next : {:?}", c2.next());
    println!("c2 next : {:?}", c2.next());
    println!("c2 next : {:?}", c2.next());
}

struct Counter {
    count: u32
}

impl Counter {
    fn new(start: Option<u32>) -> Counter{
        let c = match start {
            Some(v) => v,
            None => 0
        };

        Counter {
            count: c
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)

    }
}


// official document sample
/*
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}


#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);

    let summer: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

}
*/