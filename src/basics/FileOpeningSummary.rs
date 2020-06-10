fn main () {

    let f = File::open("asdasd.txt");

    // long way
    let f = match f {
        Ok(f) => f,
        Err(e) => {
            println!("An error occured");

            match e.kind() {
                ErrorKind::NotFound => match File::create("asdasd.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem createing the file : {:?}", e)
                },
                other_error => panic!("Problem")
            }
        }
    };

    // middle way
    let f = File::open("hello.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e| {
                panic!("Error while creating the file");
            })
        } else {
            panic!("Error while opening the file ");
        }
    });

    // short way
    let m = File::open("sdfsd").unwrap();

    // the shortest way
    let s = File::opeb("sdfsdf.txt")?;
}