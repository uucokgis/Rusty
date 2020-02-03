mod file_things {
    pub fn read_file_create_ifnot() {
        let filename = "/home/umut/Documents/test/asdasd.txt";
        let f = File::open(filename);

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(filename) {
                    Ok(fc) => {
                        println!("We provided a new file due to there is no {} file", filename);
                        fc
                    }
                    Err(e) => {
                        panic!("We couldnt also create the file. Please check the permission on it");
                    }
                },
                other_error => panic!("Encountered another error that we dont know why : {:?}", other_error)
            },
        };
    }

    pub fn read_file_create_ifnot_unwrap() {
        let f = File::open(filename).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(filename).unwrap_or_else(|error| {
                    panic!("Problem occured while creating the file : {:?}", error);
                })
            } else {
                panic!("Problem opening the file {:?}", error);
            }
        });
    }

    pub fn read_file_panic_unwrap() {
        let filename = "/home/umut/Documents/test/asdasd.txt";
        let f = File::open(filename).unwrap();
    }

    pub fn read_file_panic_expect() {
        let filename = "/home/umut/Documents/test/asdasd.txt";
        let f = File::open(filename).expect("Error occured while reading the file");
    }

    pub fn read_file_username_content() {
        fn read_username_from_file() -> Result<String, io::Error> {
            let filename = "/home/umut/Documents/test/asdasd.txt";
            let f = File::open(filename);

            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e)
            }
        }
        let content = read_username_from_file();
        content.expect("");
    }

    pub fn read_file_interrogation_mark() {
        fn read_username_from_file() -> Result<String, Error> {
            let filename = "/home/umut/Documents/test/asdasd.txt";

            let mut f = File::open(filename)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }
    }

    fn read_username_from_file_short_im() -> Result<String, Error> {
        /* Some notes about ?

        This error points out that we’re only allowed to use the ? operator in a function that
        returns Result or Option or another type that implements std::ops::Try. When you’re writing
        code in a function that doesn’t return one of these types, and you want to use ? when you
        call other functions that return Result<T, E>, you have two choices to fix this problem.

        One technique is to change the return type of your function to be Result<T, E> if you have
        no restrictions preventing that. The other technique is to use a match or one of the
        Result<T, E> methods to handle the Result<T, E> in whatever way is appropriate.
        */

        let filename = "/home/umut/Documents/test/asdasd.txt";
        let mut s = String::new();
        File::open(filename)?.read_to_string(&mut s)?;
        Ok(s)
    }


}



