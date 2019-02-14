// functions in rust

fn main() {
    println!("hello canım");
    runprint();

    let x = 5;
    let y = {
        let x = 3 + x;
        x + 1
    };

    println!("y : {}", y);
    println!("x : {}", x);


}

fn runprint() {
    print_number(5, 10);

}

fn print_number(number: i32, number2: i32) {
    let a = [number, number2];
    println!("{}", a[0]);

    let b = returned_func(a[0]);
    println!("b : {}", b);
}

fn returned_func(num:i32) -> i32 { // vay amk illa return için illa oraya -> i32 sikini yazacakmışız.
    return 5*2 + num // veya sadece 5 yaz
}

