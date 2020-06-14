fn main() {
    fn is_even(n: u32) -> bool {
        n % 2 == 0
    }

    fn _is_odd(n: u32) -> bool {
        n % 2 != 0
    }

    // 161700
    let sum_even_numbers_till_closure =
        (0..)
            .map(|x| x * x)
            .take_while(|&x| x < 10000)
            .filter(|&x| is_even(x))
            .fold(0, |s, i| s + i);

    let mut sum_even_numbers_till_function = 0;

    for i in 0.. {
        let x = i * i;
        if x < 10000 {
            if is_even(x) {
                sum_even_numbers_till_function += x;
            }
        } else {
            break;
        }
    }
    println!("toplam : closure {}", sum_even_numbers_till_closure);
    println!("toplam : function {}", sum_even_numbers_till_function);
}