use std::{env, fs};
use std::path::Path;

fn main() {
    // iterator style
    let contents = reading_file::read_contents(reading_file::read_cli_iterator());
    println!("contents of the file with iterator : {}\n\n", contents);

    let contents = reading_file::read_contents(reading_file::read_cli());
    println!("contents of the file : \n{}", contents);

    let searched_words = reading_file::search_word_insensitive("To", &contents, Option::from(false));
    println!("searched words: {:?}", searched_words);

    // by the way you can set case_sensitive as environment variable and read it
    // 1 is True, 2 is false
    let case_sensitive_env = env::var("CASE_INSENSITIVE").is_err();
    println!("case insensitive environment variable : {}", case_sensitive_env);


}


mod reading_file {
    use super::config_types;
    use std::{env, process};
    use std::fs;
    use crate::config_types::CLIConfig;
    use std::error::Error;

    pub fn read_cli() -> CLIConfig {
        let args: Vec<String> = env::args().collect();
        let config = config_types::CLIConfig::new(&args)
            .unwrap_or_else(|err| {
                println!("Problem parsing the arguments {}", err);
                process::exit(1);
            });

        // let config = match config {
        //     Ok(c) => {
        //         println!("query: {} ", c.query);
        //         println!("filename : {}", c.filename);
        //         c
        //     }
        //     _ => {
        //         panic!("asdasd");
        //     }
        // };
        config
    }

    pub fn read_cli_iterator() -> CLIConfig {
        let args = env::args();
        let config = config_types::CLIConfig::new_with_iterator(args)
            .unwrap_or_else(|err| {
                println!("Problem parsing the arguments");
                process::exit(1);
            });

        config
    }

    pub fn read_cli_ue(config: CLIConfig) -> Result<(), Box<dyn Error>> {
        // ue means: solve the error if raises where it exposes.
        let contents = fs::read_to_string(config.filename)?;
        println!("with text: \n{}", contents);
        Ok(())
    }

    pub fn read_contents(config: config_types::CLIConfig) -> String {
        fs::read_to_string(config.filename)
            .expect("Error occured while reading the file")
    }

    pub fn search_word<'a>(word: &'a str, contents: &'a str) -> Vec<&'a str> {
        let mut result: Vec<&str> = Vec::new();
        for w in contents.lines() {
            if w.contains(word) {
                result.push(w);
            }
        }

        result
    }

    pub fn search_word_insensitive<'a>(word: &'a str, contents: &'a str, case_sensitive: Option<bool>) -> Vec<&'a str> {
        let case_sensitive = match case_sensitive {
            None => {
                println!("Case sensitive set false");
                false
            }
            Some(q) => q
        };

        let mut result: Vec<&str> = Vec::new();
        for w in contents.lines() {
            if case_sensitive {
                if w.to_lowercase().contains(&word.to_lowercase()) {
                    result.push(w);
                }
            } else {
                if w.contains(&word) {
                    result.push(w);
                }
            }
        }

        result
    }
}

mod config_types {
    use std::env;

    pub struct CLIConfig {
        pub query: String,
        pub filename: String,
    }

    impl CLIConfig {
        pub fn new(args: &[String]) -> Result<CLIConfig, &'static str> {
            if args.len() != 3 {
                return Err("There has to be 2 argument");
            } else {
                let filename = args[2].clone();
                let query = args[1].clone();

                Ok(
                    CLIConfig {
                        query,
                        filename,
                    }
                )
            }
        }

        pub fn new_with_iterator(mut args: env::Args) -> Result<CLIConfig, &'static str> {
            args.next();

            let query = match args.next() {
                Some(ar) => ar,
                None => return Err("There is no query param")
            };

            let filename = match args.next() {
                Some(f) => f,
                None => return Err("There is no filename param")
            };

            Ok(CLIConfig {
                query,
                filename,
            })
        }
    }
}

#[test]
fn read_one_line() {
    let file_path = "/home/umut/CLionProjects/Rusty/src/cli_test/data/poem.txt".trim();
    let query = "To";

    let one_line_contens = fs::read_to_string(Path::new(&file_path))
        .unwrap();
    let filtered_contens = one_line_contens.lines().filter(|l| l.contains(query)).collect::<Vec<&str>>();
    println!("fff : {:?}", filtered_contens);
}
