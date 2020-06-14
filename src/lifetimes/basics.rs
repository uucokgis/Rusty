/*
Lifetime tabiri Rust'ta ownership'liğini yitirmiş variable'ların akıbetini ele alır.
Örneğin outer ve inner diye iki scope olsun ve outer scope'ta bir variable instantiate edilsin ama değer
atanmasın.
*/

pub fn intro () {
    // outer scope

    let r;  // baslatildi.
    {
        let x = 5;  // bir deger atandı.
        r = &x; // baslatilan degiskene x'in pointer'ı atandı.
        println!("r : {}", r);  // bu calisir.
    }

    println!("r : {}", r); // BU CALISMAZ. Cunku inner scope'tan çıktığımızda x ölecektir, onun pointer'ı r
    // tarafından kullanılamaz.
}

/*
Ornegin longest fonksiyonunu yazalım. iki tane borrowed deger
alıp gene pointer dondureceksek Rust bize hata verecektir cunku dondurulen referans degeri
birincinin mi ikincinin mi oldugu anlasilamaz.

Asagidaki comment'te bulunan fonksiyon calismayacaktir. Resmi dokumanda aciklamasi şöyle:

function, so we don’t know whether the if case or the else case will execute. We also don’t know the
concrete lifetimes of the references that will be passed in, so we can’t look at the scopes as we
did in Listings 10-18 and 10-19 to determine whether the reference we return will always be valid.

The borrow checker can’t determine this either, because it doesn’t know how the lifetimes of x and y
relate to the lifetime of the return value. To fix this error, we’ll add generic lifetime parameters
that define the relationship between the references so the borrow checker can perform its analysis.

*/

//pub fn longest(x: &str, y: &str) -> &str {
//    if x.len() > y.len() {
//        x
//    }
//
//    else {
//        y
//    }
//}

pub fn real_intro () {
//    &i32        // a reference
//    &'a i32     // a reference with an explicit lifetime
//    &'a mut i32 // a mutable reference with an explicit lifetime

    /*
    One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant
    to tell Rust how generic lifetime parameters of multiple references relate to each other.

    For example, let’s say we have a function with the parameter first that is a reference to an i32
    with lifetime 'a. The function also has another parameter named second that is another reference
    to an i32 that also has the lifetime 'a. The lifetime annotations indicate that the references
    first and second must both live as long as that generic lifetime.
    */

    // Examine this function
    fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // longest fonksiyonunu silme.
    pub fn lifetime_test () {
        let first = String::from("Naber lan ");
        let sec = String::from("İyilik senden");
        println!("longest one : {}", longest(&first, &sec));

        let string1 = String::from("Long string is long");
        let result;
        let string2; // BUNU SILDIGINDE VE ASAGIYI "let string2 .. " YAPTIGINDA KOD PATLAR.
        {
            string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        }
        println!("result is : {}", result);  // Hata verecektir cunku string2 nin daha buyuk cikma
        // ihtimalinden oturu patlayabilir. Lifetime'ları aynı değil ve string2 öldü.
    }

    // Lifetime parametresini koyup koymama durumunu fonksiyonun belirler. Ornegin longest
    // fonksiyonu ne olursa olsun birinci degeri donduruyor olsaydı soyle yazilabilirdi:
    fn longestv2<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    fn longest_dummy<'a> (x: &str, y: &str) -> &'a str {
        let result = String::from("my result");
        result.as_str()
    }
}