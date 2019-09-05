use std::collections::HashMap;

/*
Of our three common collections, this one is the least often used, so it’s not included in the
features brought into scope automatically in the prelude. Hash maps also have less support from the
standard library; there’s no built-in macro to construct them, for example.
*/

pub mod basics {
    pub fn main() {
        // initializing
        let mut scores = HashMap::new();
        // inserting
        scores.insert("A", 65); // coercing to literal
        scores.insert("Yellow", 9815);

        // using collect via vector of tuples
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        // here is cokomelli
        /*
    The type annotation HashMap<_, _> is needed here because it’s possible to collect into many
    different data structures and Rust doesn’t know which you want unless you specify.
    For the parameters for the key and value types, however, we use underscores, and Rust can infer
    the types that the hash map contains based on the types of the data in the vectors.

    ** Ownerships of Heap Maps
    For types that implement the Copy trait, like i32, the values are copied into the hash map.
    For owned values like String, the values will be moved and the hash map will be the owner of
    those values.
    */
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        println!("{:?}", scores.get(&"Blue".to_string()));

        // Another sample
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);

        // this is buggy because that ownership of fieldname passed into hashmap
        println!("field name : {}", field_name);

//    If we insert references to values into the hash map, the values won’t be moved into the hash
//    map. The values that the references point to must be valid for at least as long as the hash
//    map is valid.
        // but you can do this:
        println!("field value from hash map : {:?}", map.get(&field_name).unwrap());

        // So it returns Option<T> value. You can check it using match:
        let k = map.get(&"asdas".to_string());
        match k {
            Some(fielld_name) => println!("IT is not empty"),
            None => println!("It is empty you motherfucker !")
        }
    }
}

pub mod advanced_hashmaps {
    /*
    By default, HashMap uses a “cryptographically strong”1 hashing function that can provide
    resistance to Denial of Service (DoS) attacks. This is not the fastest hashing algorithm
    available, but the trade-off for better security that comes with the drop in performance is
    worth it. If you profile your code and find that the default hash function is too slow for your
    purposes, you can switch to another function by specifying a different hasher. A hasher is a
    type that implements the BuildHasher trait. We’ll talk about traits and how to implement them
    in Chapter 10. You don’t necessarily have to implement your own hasher from scratch; crates.io
    has libraries shared by other Rust users that provide hashers implementing many common hashing
    algorithms.

    */
    pub use std::collections::HashMap as dict;
    pub fn updating () {
        let mut scores = dict::new();
        scores.insert(String::from("Blue"), 5);
        scores.insert(String::from("Blue"), 10); // overwrites
        println!("{:?}", scores);
    }

    pub fn inserting_on_non_existance () {
        let mut scores = dict::new();
        scores.insert(String::from("Blue"), 12);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(20);

        println!("{:?}", scores);
    }

    pub fn inserting_on_existance () {
        let mut scores = dict::new();
        scores.insert(String::from("Blue"), 15);

    }

    pub fn interesting_example () {
        let mut map = dict::new();
        let word = String::from("Hello world wonderful world umut");

        for w in word.split_whitespace() {
            /*
             Here we store that mutable reference in the count variable, so in order to assign to
             that value, we must first dereference count using the asterisk (*). The mutable
             reference goes out of scope at the end of the for loop, so all of these changes are
             safe and allowed by the borrowing rules.
             */
            let count = map.entry(w).or_insert(0);
            println!("count : {}", count);
            println!("how many times occurs {:?} word : {:?}", w, count);
            *count += 1; // We need to add. Otherwise all will be count as 0
        }
    }

    // hashmap with generics
    fn _create_hashmap(param1: u32, param2: u32) -> HashMap<u32, u32> {
        let mut v: HashMap<u32, u32> = HashMap::new();
        v.insert(param1, param2);
        v
    }

    fn _create_hashmap_generic <U> (param1: U, param2: U) -> HashMap<U, U>
        where U: Eq + Hash
    {
        let mut v:HashMap<U, U>= HashMap::new();
        v.insert(param1, param2);
        v
    }

    fn _create_hashmap_multigeneric<U, V> (param1: U, param2: V) -> HashMap<U, V>
        where U: Eq + Hash,
              V: Eq + Hash
    {
        let mut v: HashMap<U, V> = HashMap::new();
        v.insert(param1, param2);
        v
    }
}