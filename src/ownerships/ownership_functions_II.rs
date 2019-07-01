//// ownership functions - II


fn main() {
    let s = String::from("hello");

    change(&s);

    println!("s'in ownership'liği halen düşmedi : {}", s);

    let some_string_new = change(&s);
    println!("new some string : {}", some_string_new);
}

// & ile actual value aldığımız için ona böyle bir iş yapamıyoruz.

fn change(some_string: &String) -> (String) { // some_string: %mut String yazsaydık hata vermezdi.
    println!("some string nedir : {}", some_string);
    let new_str = some_string;
    println!("new str cap nedir {}", new_str.len());

    let mut some_string = "asdad";

    some_string.to_string()

    // işi bitirdikten sonra some_string'i döndürmemize gerek yok. Çünkü bunu ownership'i geri
    // iade etmek için yaparız. Burada ownership hiç bizim olmadı ki geri döndürelim..
}