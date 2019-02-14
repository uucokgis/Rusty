/*
Tipik bir tahmin oyunu. 1-100 arasında bir sayı tutuyoruz ve kullanıcının
bunu bulmasını bekliyoruz.
*/

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // önce biz bir sayı tahmin edelim

    let secret_number = rand::thread_rng().gen_range(1,100);
    println!("Bizim tahmin ettiğimiz sayı : {}", secret_number);
    println!("1 ile 100 arasında bir sayı tahmin ediniz : ");

    loop {
        // kullanıcının tahmin edecegi sayi icin bi var ayarlayalim
        // input her zaman string'tir.
        let mut guess = String::new();


        io::stdin().read_line(&mut guess)
            .expect("girdi satırını okurken bir hata oldu !");

        // sayının tipi ile oynayalım.
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Metin girme lan zürriyetsiz.");
                continue
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Çok küçük !"),
            Ordering::Greater => println!("Çok büyük !"),
            Ordering::Equal => {
                println!("Sayıyı buldunuz !");
                break;
            },
        }
    }
}
