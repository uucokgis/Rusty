/*
Rust groups errors into two major categories: recoverable and unrecoverable errors. For a
recoverable error, such as a file not found error, it’s reasonable to report the problem to the user
and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a
location beyond the end of an array.

Rust doesn’t have exceptions.
Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution
when the program encounters an unrecoverable error.
see: Cargo.toml in this directory
*/

fn main () {
    pub mod basics {
        use std::fs::File;
        use std::io::ErrorKind;

        fn openingfile() {
            let f = File::open("hello.txt");

            let f = match f {
                Ok(nonsense) => nonsense,  // does not matter the name of it. But they have to be same
                Err(errur) => match errur.kind() {
                    ErrorKind::NotFound => match File::create("helloagain.txt") {  // if we cant find, we can create
                        Ok(nomatteragain) => nomatteragain,  // again
                        Err(err) => panic!("We tried to create the file but not successfull : {:?}", err.to_string())
                    }

                    other_error => panic!("Unexpected error raised : {}", errur.to_string())
                }
            };
        }

        fn openingfilev2() {
            // without using match
            let f = File::open("hello.txt").unwrap_or_else(|error| {
                if error.kind() == ErrorKind::NotFound {
                    File::create("helloagain.txt").unwrap_or_else(|error| {
                        panic!("Problem creating the file ! Error is : {:?}", error);
                    })
                } else {
                    panic!("Problem opening the file ! Error is {:?}", error);
                }
            });
        }

        // basic one. See: mod advanced to examine shortcuts etc
        fn read_username_from_file() -> Result<String, Error> {
            let f = File::open("hello.txt");

            let mut f = match f {
                Ok(file) => file,
                Err(error) => return Err(error),
            };

            /*
            Then we create a new String in variable s and call the read_to_string method on the file
            handle in f to read the contents of the file into s. The read_to_string method also returns
            a Result because it might fail, even though File::open succeeded. So we need another match
            to handle that Result: if read_to_string succeeds, then our function has succeeded, and we
            return the username from the file that’s now in s wrapped in an Ok

            */

            let mut s = String::new();
            /*
             If read_to_string fails, we return the error value in the same way that we returned the
             error value in the match that handled the return value of File::open. However, we don’t
             need to explicitly say return, because this is the last expression in the function.
             */

            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }
    }

    pub mod advanceds {

        // This mod is important because you'll see how ? mark can be used instead of match expression
        /*
        The ? operator can only be used in functions that have a return type of Result,
        because it is defined to work in the same way as the match expression we defined
        */
        fn read_username_from_file() -> Result<String, Error> {
            /*
            The ? placed after a Result value is defined to work in almost the same way as the match
            expressions we defined to handle the Result values in Listing 9-6. If the value of the
            Result is an Ok, the value inside the Ok will get returned from this expression, and the
            program will continue. If the value is an Err, the Err will be returned from the whole
            function as if we had used the return keyword so the error value gets propagated to the
            calling code.
            */
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();

            /*
            There is a difference between what the match expression from Listing 9-6 and the ? operator
            do: error values that have the ? operator called on them go through the from function,
            defined in the From trait in the standard library, which is used to convert errors from one
            type into another. When the ? operator calls the from function, the error type received is
            converted into the error type defined in the return type of the current function. This is
            useful when a function returns one error type to represent all the ways a function might
            fail, even if parts might fail for many different reasons.

            THIS IS IMPORTANT:
            As long as each error type implements the from function to define how to convert itself to
            the returned error type, the ? operator takes care of the conversion automatically.
            */

            // We could even shorten this code further by chaining method calls immediately after the ?
            // see: read_username_from_filev2 function

            f.read_to_string(&mut s)?;
            Ok(s)
        }

        fn read_username_from_filev2() -> Result<String, Error> {
            let mut s = String::new();
            let filename = "hello.txt";
            File::open(filename)?.read_to_string(&mut s);
            Ok(s)
        }

        fn read_username_from_filev3() -> Result<String, Error> {
            /*
            Reading a file into a string is a fairly common operation, so Rust provides the convenient
            fs::read_to_string function that opens the file, creates a new String, reads the contents
            of the file, puts the contents into that String, and returns it.

            Of course, using fs::read_to_string doesn’t give us the opportunity to explain all the error
            handling, so we did it the longer way first.
            */
            fs::read_to_string("hello.txt")
        }

        // what if we use ? in main function. CHANGE FUNCTION NAME TO main BEFORE COPY:
        pub fn maininterrogation () -> Result<(), Box<dyn Error>> {
            use std::error::Error;
            use std::fs::File;
            /*
            The main function is special, and there are restrictions on what its return type must be.
            One valid return type for main is (), and conveniently, another valid return type is Result<T, E>

            about box:
            The Box<dyn Error> type is called a trait object, which we’ll talk about in the
            “Using Trait Objects that Allow for Values of Different Types” section in Chapter 17.
            For now, you can read Box<dyn Error> to mean “any kind of error.” Using ? in a main
            function with this return type is allowed.
            */

            let f = File::open("hello.txt")?;
            Ok(())
        }
    }

    pub mod advancedsv2 {
        /*
        Similarly, the unwrap and expect methods are very handy when prototyping, before you’re
        ready to decide how to handle errors. They leave clear markers in your code for when you’re
        ready to make your program more robust.
        */
        use std::net::IpAddr;

        pub fn cases_iknowmore_compiler() {
            /*
            We’re creating an IpAddr instance by parsing a hardcoded string. We can see that
            127.0.0.1 is a valid IP address, so it’s acceptable to use unwrap here. However,
            having a hardcoded, valid string doesn’t change the return type of the parse method:
            we still get a Result value, and the compiler will still make us handle the Result as
            if the Err variant is a possibility because the compiler isn’t smart enough to see that
            this string is always a valid IP address. If the IP address string came from a user
            rather than being hardcoded into the program and therefore did have a possibility of
            failure, we’d definitely want to handle the Result in a more robust way instead.

            */
            let home = "127.0.0.1".parse().unwrap();

        }

        pub fn guess_game_withgetter () {
            /*
             we implement a method named value that borrows self, doesn’t have any other parameters,
             and returns an i32. This kind of method is sometimes called a getter, because its purpose
             is to get some data from its fields and return it. This public method is necessary because
             the value field of the Guess struct is private. It’s important that the value field be
             private so code using the Guess struct is not allowed to set value directly: code outside
             the module must use the Guess::new function to create an instance of Guess, thereby ensuring
             there’s no way for a Guess to have a value that hasn’t been checked by the conditions in
             the Guess::new function.
             */

            pub struct Guess {
                value: i32,
            }

            impl Guess {
                pub fn new(value: i32) -> Guess {
                    if value < 1 || value > 100 {
                        panic!("Guess value must be between 1 and 100, got {}.", value);
                    }

                    Guess {
                        value
                    }
                }

                pub fn value(&self) -> i32 {
                    self.value
                }
            }
        }
    }
}

// adding error messages
fn error_messages() {
    eprintln!("my error ");
}
