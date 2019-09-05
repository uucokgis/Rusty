use std::collections::HashMap;
/*

Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs
when you need to store, access, and modify data. Here are some exercises you should now be equipped
to solve:

Given a list of integers, use a vector and return the mean (the average value), median (when sorted,
the value in the middle position), and mode (the value that occurs most often; a hash map will be
helpful here) of the list.

Convert strings to pig latin. The first consonant of each word is moved to the end of the word and
“ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the
end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

Using a hash map and vectors, create a text interface to allow a user to add employee names to a
department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
Then let the user retrieve a list of all people in a department or all people in the company by
department, sorted alphabetically.

*/

pub fn main () {
    // given list of integers
    let myarray = [1,2,3,4,5];
    let myvector_fromarr = myarray.to_vec();

    let mut counter = 0;
    for v in &myarray {
        counter += v;
    }
    // mean value
    let mean = counter / myvector_fromarr.len();
    println!("mean value of array : {}", mean);

    // median
    let median_value:i32;
    let median_index = (myvector_fromarr.len() as i32 / 2);
    if median_index % 2 == 0 {
        median_value = myvector_fromarr[median_index];
    }
    else {
        median_value = myvector_fromarr[median_index + 1];
    }
    println!("median value of array : {}", median_value);

    // mode: most frequent value
    // solution should be with hashmap. Otherwise we will create another list which includes counts
    // of variables as ordered either we will iterate each value and count frequent each time. Whenever
    // we find bigger frequency value, we'll replace it

    let mut modhashmap = HashMap::new();
    for v in myarray.iter() {
        let count = modhashmap.entry(v).or_insert(0);
        *count += 1;
        println!("How many times {} value occurs in list", count);
    }
    println!("mod hashmap : {:?}", modhashmap);

    // pig latin: Converting your word to pig latin language.
    pub fn convert_word_to_piglatin(mystring: &String) {
        // constant variables
        let consonant_literal_arrays = ["b", "c", "ç", "d", "f", "g", "h", "j", "k",
            "l", "m", "n", "p", "q", "r", "s", "ş", "t", "v", "x", "y", "z"];

        // which word will be pushed
        let adding_word = "hay";

        // which word will be converted to pig latin?
//        let mystring = String::from("Apple");
        let mut output_string = String::new();

        // so this is first letter of the word
        let k = mystring.chars().next();

        match mystring.chars().next() {
            None => panic!("This is wrong !"),
            Some(i) => {
                let str_i:&str = &i.to_string();

                if consonant_literal_arrays.contains(&str_i) {
                    println!("Starts with consonant letter");
                    let output_string = output_string + &mystring[1..] + &"-" + str_i + &adding_word;
                    println!("output string : {}", output_string);
                }
                else {
                    println!("Does not start with consonant letter");
                    let output_string = output_string + &mystring[1..] + &"-" + str_i + &adding_word;
                    println!("output string : {}", output_string);

                }
            }
        }
    }

    // example
    convert_word_to_piglatin(&"Umut".to_string());



}