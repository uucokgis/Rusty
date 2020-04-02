use crate::config_types::CLIConfig;
use std::env;
fn main () {
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
    use std::path::Path;
    use crate::config_types::CLIConfig;
    use std::error::Error;

    pub fn read_cli() -> CLIConfig{
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

    pub fn search_word <'a> (word: &'a str, contents: &'a str) -> Vec<&'a str> {
        let mut result: Vec<&str> = Vec::new();
        for w in contents.lines() {
            if w.contains(word) {
                result.push(w);
            }
        }

        result
    }

    pub fn search_word_insensitive <'a> (word: &'a str, contents: &'a str, case_sensitive: Option<bool>) -> Vec<&'a str> {
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
    pub struct CLIConfig {
        pub query: String,
        pub filename: String

    }

    impl CLIConfig {
        pub fn new(args: &[String]) -> Result<CLIConfig, &'static str> {
            if args.len() != 3 {
                return Err("There has to be 2 argument");
            }

            else {
                let filename = args[1].clone();
                let query = args[2].clone();

                Ok(
                    CLIConfig {
                        query,
                        filename
                    }
                )
            }

        }
    }
}



