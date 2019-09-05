/*
Rust strings don’t support indexing, but slicing was provided.
A String is a wrapper over a Vec<u8>
*/

pub mod basics {
    pub fn run() {
        // initializing
        let mut s1 = String::new();
        // Assigning
        let s1 = "Initial Comments";
        // it was string literal. so what about String?
        let s = "Initial Comments".to_string();
        // pushing words into string
        s1.push_str(" plus other comments");

        // interesting concatenating
        let myword = String::from("Hello, ");
        let myword2 = String::from(" World canim");
        let merged_words  = myword + &myword2;  // so ownership of myword has passed into merged_words
        // myword cannot be used.

        // So look example above. You cannot do :
            // let merged_words = myword1 + myword2;
            // println!("mywords 1 : {}", myword1);

        // tic tac toe (adding more than 2 string)
        let tic = String::from("Tic");
        let toc = String::from("Toc");
        let toe = String::from("Toe");

        let merged = tic + &toc + &toe;  // it is important that first one must be owner

        // indexing
        let hello = "Здравствуйте";
        // let s = &hello[5]; // This is wrong
        let s = &hello[0..2]; // YOU CANNOT DO : 0..1
            // also you cannot :
            // let s = &hello[1..4];

        // However, you can iterate strings:
        for c in "नमस्ते".chars() {
            println!("letter as char : {}", c);
        }
        // if you iterate bytes:
        for b in "नमस्ते".bytes() {
            println!("byte : {} ", b);
        }

        let bitler = first_word(&kelime);
        println!("bitler nedir : {}", bitler);

        let mut temizlenecek_kelime = String::from("Ayva çiçek açmış yaz mı gelecek");
        temizlenecek_kelime.clear(); // bu içerisini "" yapıyor. Dolayısıyla da var'ın mutable olması gerek.

        let tem_bit = first_word(&temizlenecek_kelime);
        print!("{}", tem_bit);
    }

    pub fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }
}

pub mod strings_slice {
    // string slice

    pub fn run() {
        let kelime = String::from("İlk kelimeyi bulalım");
        let ilk_kelime = ilk_kelimeyi_bul(&kelime);
        print!("ilk kelime nedir : {} \n", ilk_kelime);

        let ikinci_kelime = ikinci_kelimeyi_bul(&kelime);
        println!("ikinci kelime nedir : {}", ikinci_kelime);

    }

    pub fn ilk_kelimeyi_bul(s: &String) -> &str {
        let bytelar = s.as_bytes();

        for (i, &item) in bytelar.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];

            }
        }

        &s[..]
    }

    pub fn ikinci_kelimeyi_bul(s: &String) -> &str {
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
//    pub fn find_nth_word(s: &String, n: &i32) -> &str { // şunu ayarlıcam, beceremedim amk.
//        let bytelar = s.as_bytes();
//        let mut sayac = 0;
//        let mut bosluk_indis = 0;
//
//        for (i, &item) in bytelar.iter().enumerate() {
//            if item == b' ' {
//                sayac += 1;
//
//                if &sayac == n {
//                    let nth_word = &s[bosluk_indis..i];
//                    return &s[bosluk_indis..i]
//                }
//                bosluk_indis = i;
//            }
//
//        }
//        return &s
//
//    }

}

pub mod strings_crashcourse {
    pub fn run () {
        let mut hello_string = String::from("Hello");
        println!("hello string : {}", hello_string);

        // length of string
        println!("length of string : {}", hello_string.len());

        // pushing some string literal - CHAR
        hello_string.push(' ');

        // pushing some string - STRING
        hello_string.push_str("World");
        println!("last situation of hello string : {}", hello_string);

        // string literal
        let new_hello_string = "Hello Literal";
        println!("{}", &new_hello_string);

        // contains
        println!("new hello string contains 'Literal'=>  {}", new_hello_string.contains("Literal"));

        // replace
        hello_string = hello_string.replace("World", "Umut");
        println!("hello string : {}", hello_string);

        // replace for literals
        let new_hello_string = new_hello_string.replace("Literal", "Umut");  // you need to redefine due to immutability
        println!("new hello string : {}", new_hello_string);

        // loop throught string by white spaces
        for word in hello_string.split_whitespace() {
            println!("word : {}", word);
        }

        // create string with capacity
        let mut s = String::with_capacity(10);
        s.push_str("AAA AAA AA");
        println!("capacity of s : {}", s.capacity());  // it will exceeds when you over 10 capacity

        // let s2 = String::with_capacity(10);
        // s2.push_str("Test");  // it is not going to work due to immutability
        // println!("s2 : {}", s2);

        // assertion testing
        // assert_eq!(3, s.len());  // it will raise assertion PANIC
        assert_eq!(10, s.capacity());

    }

}