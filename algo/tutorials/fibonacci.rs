fn fibonacci(number: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..number {
        let tmp = a;
        a = b;
        b = a + tmp;
    }

    return b
}

fn main() {
    println!("fibonacci  : {}", fibonacci(30));
}