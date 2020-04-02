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
