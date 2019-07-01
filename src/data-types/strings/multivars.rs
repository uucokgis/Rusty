// multi vars and behaviours
// also scope drop issues

fn main(){
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{} world!", s2);
}

fn int_vars() {
    let y = "asdasd";
    let k = 5;

    let x = y;

    println!("y nedir : {} ; x nedir : {}", y, x);
    // it just makes like this
    /*
    Bind the value 5 to x; then make a copy ( is it shallow copy? dont know.)
    of the value in x then bind it to y.

    What about string?
    */
    // bu davranışı sadece String iken yapıyor literal iken değil !

    let s1 = String::from("Naber");
    let s2 = s1;

    println!("y nedir : {} ; x nedir : {}", s1, s2); // patlak kod.

    // yukarıdaki gibi yapacak zannedecekkeeen.. hayır öyle yapmıyor.
    // burada şekiller olduğu için kitaba gitmen gerek:
    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move


//    let k1 = String::from("scope test");
//    let k2 = k1;
}

fn clone_sample() {
    let s1= String::from("Hellüü");
    let s2 = s1.clone();
}