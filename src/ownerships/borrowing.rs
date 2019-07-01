// Borrowing cases

fn main () {
    test();
}

fn test() {
    let mut latter = "asdad";
    let r1 = &mut latter;
    let r2 = &mut latter; // bir kere ödünç alınan şey bir daha alınamaz.

    let mut stringg = String::from("adede");
    let k1 = &mut stringg;
    let k2 = &mut stringg; // can't be borrowed after once done

    println!("latter nedir : {}", latter);

    println!("r1 nedir : {}", r1);
    println!("r2 nedir : {}", r2);

    println!("stringg nedir : {}", stringg);
    println!("k1 nedir : {}", k1);
    println!("k2 nedir : {}", k2);

}


