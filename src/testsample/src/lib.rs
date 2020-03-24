/* Not: Bazı diğer dizinlerde örnekler bulabilirsiniz:
see: src/packaging/packaging/src/excel_importer/src/reader/csvreader.rs
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess{
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got : {}", value);
        }

        Guess {
            value
        }
    }

    pub fn new_no_over_100control(value: i32) -> Guess {
        if value < 1 {
            panic!("The value cannot be negative, got : {}", value)
        }

        Guess {value}
    }

    pub fn new_expected(value: i32) -> Guess {
        if value < 1 {
            panic!("Value must not be negative, got : {}", value);
        }
        else if value > 100 {
            panic!("Value must not be higher than 100, got : {}", value);
        }

        Guess {value}
    }
}


#[cfg(test)]
mod tests{
//    use super::*;
    use super::{Rectangle, Guess};

    #[test]
    fn larger_can_hold_smaller () {
        let larger = Rectangle{width: 8, height: 5};
        let smaller = Rectangle{width: 5, height: 12};

        assert!(!smaller.can_hold(&larger)); // passed

        // we know what has to be
//        assert_eq!(true, smaller.can_hold(&larger));  // failed

        // we don't know what has to be but now what must not to be
        assert_ne!(true, smaller.can_hold(&larger));  // passed:
    }

    // adding custom failure messages
    //  dummy function
    pub fn greetings(name: &str) -> String {
        format!("Hello, {}", name)
    }

    #[test]
    fn greeting_contains_name () {
        let result = greetings("Carol");
        assert!(result.contains("Carol"));  // passed
    }

    // should_panic example
    #[test]
    #[should_panic]
    fn greater_than_100() {
        // leave one of this statement to run
//        Guess::new(50);  // it will fail
//        Guess::new(200); // it will pass. Because it "should panic"

        Guess::new_no_over_100control(200);  // failed. Because it "should panic" but passed due to neglect of over than 100
    }

    // should panic example: expected
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_expected() {
        Guess::new_expected(200);  // you can see both error messages
    }

    // Result<T,E> example
    /*
    You can’t use the #[should_panic] annotation on tests that use Result<T, E>. Instead, you should
    return an Err value directly when the test should fail.
    */
    #[test]
    fn it_works() -> Result<(), String>{
        if 2 + 2 == 4 {
            Ok(())
        }
        else {
            Err(String::from("Two plus two does not equal four"))
        }
    }

    // to see all options of cargo test: cargo test --help
    // see both: cargo test -- --help

    /*
    Çoklu testleri paralelde thread'ler üzerinden çalıştırabilirsin.

    Because the tests are running at the same time, make sure your tests don’t depend on each other
    or on any shared state, including a shared environment, such as the current working directory
    or environment variables.

    If you don’t want to run the tests in parallel or if you want more fine-grained control over the
    number of threads used, you can send the --test-threads flag and the number of threads you want
    to use to the test binary
    see: cargo test -- --test-threads=1 // no paralel. or you may type 5, 20 whatever you want

    NSA println! gibi Display trait'li fonksiyonları test esnasında görmezsin. Ancak
    patladığı zaman test hatası çıkar ve sonrasında kendi yazdırdığın hatayı görebilirsin ki bu da
    senin testinin neden patladığını anlatır.

    If we want to see printed values for passing tests as well, we can disable the output capture behavior by using the --nocapture flag:
    cargo test -- --nocapture

    // running a subset of tests by name
    Sometimes, running a full test suite can take a long time. If you’re working on code in a
    particular area, you might want to run only the tests pertaining to that code. You can choose
    which tests to run by passing cargo test the name or names of the test(s) you want to run as an argument.
    try : cargo test it_works

    Mesela içinde greater geçen fonksiyonları test etsin istiyorsanız:
    cargo test greater

    Bazı fonksiyonları ise uzunca bir süre teste almak istemeyebilirsiniz:
    #[test]
    #[ignore]
    fn expensive_test() {
    // code that takes an hour to run
    }

    Eğer sadece bu ignore edilmiş fonksiyonları test etmek istiyorsanız:
    cargo test -- --ignored
    */

    /*
    TEST ORGANIZATION
    As mentioned at the start of the chapter, testing is a complex discipline, and different people
    use different terminology and organization. The Rust community thinks about tests in terms of
    two main categories: unit tests and integration tests. Unit tests are small and more focused,
    testing one module in isolation at a time, and can test private interfaces. Integration tests are
    entirely external to your library and use your code in the same way any other external code would,
    using only the public interface and potentially exercising multiple modules per test.
    */

    // Rust'ta private fonksiyonlar da teste tabii tutulabilir. Bu durum diğer dillerde zor veya
    // imkansız gibi bi şeymiş.. Neden öyle anlamadım ama google'da da bu konuda çok muhabbet yazıyordu

    /*
    Integration test'leri kütüphanenin tamamı için yapılır. unit testler ise fonksiyonlar üzerinde
    daha özelde yapılır. unit testlerde interface'ler de teste tabii tutulur.

    Integration testlerde kütüphanenizin public API'leri test edilir. integration testleri yapmak için
    tests dizini gerekir.
    */
}