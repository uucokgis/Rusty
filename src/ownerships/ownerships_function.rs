// ->
// ownership and functions - I

fn main () {
//    let x = String::from("Naber lan");
//
//    let y = 5;
//
//    takes_ownership(x);
//    makes_copy(y);
//
//    println!("x nedir : {}", x); // x bir String olduğu için ownership gider.

    // return as tuple
    let s1 = String::from("naber la");
    let (s2, lenght) = return_multi_vals(s1);

    println!("{} degiskeninin uzunlugu : {}", s2, lenght);
    yanis();
}
//
//fn takes_ownership(kelime: String) {
//    println!("{}", kelime);
//}
//
//fn makes_copy(sayi: i32) {
//    println!("{}", sayi);
//}
//
//fn gives_ownership() -> String { // biz uretmedik ama urettigimizin ownership'ligini
//    // veriyoruz.
//    let kelime = String::from("Naber la");
//
//    kelime
//}
//
//fn takes_and_gives_back(kelime: String) -> String {
//    kelime
//}

fn return_multi_vals(s: String) -> (String, usize) {
    let lenght = s.len();

    (s, lenght)
}

// eğer ownership'liğini almadan sadece bir iş yapıp sonucunu döndürmek isteseydik:
// fonksiyona variable'ı girerken ve fonksiyonun içerisinde var'i isteyip tanımlarken & kullanacaktık.
// bir de let tanımlamaları falan da gitmiş baksana.
fn yanis() {
    let s1 = String::from("Hellüüüö");
    let lent = calculate_lenght(&s1);

    println!("{} uzunlugu : {}", s1, lent);
}

fn calculate_lenght(s: &String) -> usize {
    s.len()
}


fn ownership_testleri() {
    let kelime = String::from("Naber lan düdüüüük. Bence python daha iyi ehee");
    let (yeni_kelime, kelime_uzunluk) = alver(kelime);

    println!("Kelime nedir : {}", kelime); // burası patlatır çünkü ownership değişti.
    println!("Yeni kelime nedir, ownership değişti : {}", yeni_kelime);
    println!("Yeni kelime uzunluğu : {}", kelime_uzunluk);


}

fn alver(x: String) -> (String, usize) {
    let uzunluk = x.len();
    (x, uzunluk)
}