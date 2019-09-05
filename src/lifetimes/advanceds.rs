pub fn with_structs () {
    struct ImportantExcerpt<'a>  // excerpt alıntı anlamındaymış
    {
        part: &'a str,
    }

    let novel = String::from("Call me asdasdas. some years ago...");
    let first_sentence;

    {
        first_sentence = &novel.split('.')
            .next()
            .expect("Sorry not '.' ");

    }

    let f = ImportantExcerpt {
        part: &first_sentence
    };


}

/*
Fonksiyonlara konulan lifetime annotation'ı eskiden tüm fonksiyonlara konulurdu. Rust ekibi sürekli olarak
kod yazma pratiklerini sürdürerek dilde ne kolaylaştırılabilir bunu anlamaya çalıştılar.
Eskiden -pre 1.0- fonksiyonlardaki her parametrenin lifetime'ı mutlaka gösterilmeliydi. Ancak Rust dev'ler
lifetime koyma işini bir dizi kurallarla compiler'a anlatabileceklerini farkettiler. Şu an için
biz lifetime koyuyoruz ama belki gelecekte çok daha az kullanacağız.

Kural Bir:
Referans olan her parametre kendi lifetime parametresini alır: fn foo<'a, 'b> (x: &'a str, y: &'b str)

İkinci Kural:
Eğer sadece bir tane input lifetime parametresi varsa o parametre output lifetime parametrelerine de uygulanır:
fn foo<'a> (x: &'a str) -> &'a str

Üçüncü Kural:
Eğer çok input lifetime parameter varsa ve bunlardan bir tanesi &self veya & mut self ise - bu onun
metod olduğunu gösterir - self parametresinin lifetıme'ı output lifetime parametresine uygulanır.
*/

pub fn lifetime_methods () {
    // Lifetime annotations with methods
    struct ImportantExcerpt<'a>  // excerpt alıntı anlamındaymış
    {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        //        The lifetime parameter declaration after impl and its use after the type name are required,
//        but we’re not required to annotate the lifetime of the reference to self because of the first elision rule.
        fn level(&self) -> i32 {
            3
        }

        // third rule applies:
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please : {}", announcement);
            self.part
        }
    }

// static lifetime. Aslında bildiğin global
    let s: &'static str = "I have static lifetime";
}

pub fn lifetime_sample () {
    // Lets mix up everything: generic type parameters, trait bounds, lifetimes in one function !

    fn longest_with_annotations<'a, T> (x: &'a str, y: &'a str, ann: T) -> &'a str
        where T: Display
    {
        println!("Announcement : {}", ann);
        if x.len() > y.len() {
            x
        }

        else {
            y
        }
    }

    let x = String::from("Umut");
    let y = String::from("Şerif");

    let long_one = longest_with_annotations(&x, &y, "dittiri dittiriiiit");
    println!("long item : {}", long_one);
}