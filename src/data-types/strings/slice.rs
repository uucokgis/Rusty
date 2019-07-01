// slicing

fn main() {
    let kelime = String::from("Naber lan dünya! ");

    let bitler = first_word(&kelime);
    println!("bitler nedir : {}", bitler);

    let mut temizlenecek_kelime = String::from("Ayva çiçek açmış yaz mı gelecek");
    temizlenecek_kelime.clear(); // bu içerisini "" yapıyor. Dolayısıyla da var'ın mutable olması gerek.

    let tem_bit = first_word(&temizlenecek_kelime);
    print!("{}", tem_bit);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
