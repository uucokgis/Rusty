// string slice

fn main() {
    let kelime = String::from("İlk kelimeyi bulalım");
    let ilk_kelime = ilk_kelimeyi_bul(&kelime);
    print!("ilk kelime nedir : {} \n", ilk_kelime);

    let ikinci_kelime = ikinci_kelimeyi_bul(&kelime);
    println!("ikinci kelime nedir : {}", ikinci_kelime);

}

fn ilk_kelimeyi_bul(s: &String) -> &str {
    let bytelar = s.as_bytes();

    for (i, &item) in bytelar.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];

        }
    }

    &s[..]
}

fn ikinci_kelimeyi_bul(s: &String) -> &str {
    let bytelar = s.as_bytes();
    let mut sayac = 0;
    let mut bosluk_indis = 0;

    for (i, &item) in bytelar.iter().enumerate() {
        if item == b' ' {
            sayac += 1;

            if sayac == 2 {
                return &s[bosluk_indis..i]

            }
            bosluk_indis = i;
        }
    }
    return &s
}

// algo doğru da syntax'tan patlıyoruz.
fn find_nth_word(s: &String, n: &i32) -> &str { // şunu ayarlıcam, beceremedim amk.
    let bytelar = s.as_bytes();
    let mut sayac = 0;
    let mut bosluk_indis = 0;

    for (i, &item) in bytelar.iter().enumerate() {
        if item == b' ' {
            sayac += 1;

            if sayac == n {
                let nth_word = &s[bosluk_indis..i];
                return &s[bosluk_indis..i]
            }
            bosluk_indis = i;
        }

    }
    return &s

}