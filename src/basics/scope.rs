// what is scope

fn main() {
    let mut kelime = String::from("Naber");

    kelime.push_str("");
    println!("{}", kelime);

}

fn scope_sample() {
   {                      // s is not valid here, it’s not yet declared
       let s = "hello";   // s is valid from this point forward

// do stuff with s
   }
   // this scope is now over, and s is no longer valid

   println!("s nedir : {}", s) // hata verir.
}

