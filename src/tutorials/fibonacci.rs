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



// Another example
#[macro_use]
extern crate timeit;

fn main () {
    timeit!( {fibonacci_golden()});
//    fibonacci_golden();
}

fn fibonacci_golden() {
    let golden_ratio = (1.0f64 + 5.0f64.powf(0.5)) / 2.0f64;
    println!("Golden ratio : {}", golden_ratio);
    let up: f64 = golden_ratio.powi(100000) - (1.0 - golden_ratio).powi(100000);
    let dwn: f64 = 5.0f64.sqrt();

    println!("result : {}", up / dwn);
}
