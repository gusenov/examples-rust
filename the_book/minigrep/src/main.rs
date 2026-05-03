// main.rs handles running the program

#![allow(unused)]

use std::env;
use std::fs;
use std::process;
use std::error::Error;

//use minigrep::search;
use minigrep::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();
    // Note that std::env::args will panic if any argument contains invalid Unicode. 
    // If your program needs to accept arguments containing invalid Unicode, use std::env::args_os instead. 
    // That function returns an iterator that produces OsString values instead of String values.

    //dbg!(args);

    //let query = &args[1];
    //let file_path = &args[2];
    //let (query, file_path) = parse_config(&args);

    //let config = parse_config(&args);
    //let config = Config::new(&args);
    //let config = Config::build(&args).unwrap_or_else(|err| {
    let config = Config::build(env::args()).unwrap_or_else(|err| {        
        //println!("Problem parsing arguments: {err}");
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //println!("Searching for {query}");
    //println!("In file {file_path}");
    //println!("In file {file_path}");

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.file_path);
    //println!("In file {}", config.file_path);

    //let contents = fs::read_to_string(file_path)
    //let contents = fs::read_to_string(config.file_path)
    //    .expect("Should have been able to read the file");

    //println!("With text:\n{contents}");

    //run(config);

    if let Err(e) = run(config) {
        // We use if let rather than unwrap_or_else to check whether run returns an Err value and to call process::exit(1) if it does. 
        // The run function doesn’t return a value that we want to unwrap in the same way that Config::build returns the Config instance. 
        // Because run returns () in the success case, we only care about detecting an error, so we don’t need unwrap_or_else to return the unwrapped value, which would only be ().

        // The bodies of the if let and the unwrap_or_else functions are the same in both cases: We print the error and exit.

        //println!("Application error: {e}");
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Config { query, file_path, ignore_case }
    }

    //fn build(args: &[String]) -> Result<Config, &'static str> {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        //if args.len() < 3 {
        //    return Err("not enough arguments");
        //}

        args.next();

        //let query = args[1].clone();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        //let file_path = args[2].clone();
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        //Ok(Config { query, file_path })
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

//fn parse_config(args: &[String]) -> (&str, &str) {
fn parse_config(args: &[String]) -> Config {
    //let query = &args[1];
    //let file_path = &args[2];
    //(query, file_path)

    let query = args[1].clone();
    let file_path = args[2].clone();
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    Config { query, file_path, ignore_case }
}

//fn run(config: Config) {
fn run(config: Config) -> Result<(), Box<dyn Error>> {
// Box<dyn Error> means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be. 
// This gives us flexibility to return error values that may be of different types in different error cases. 
// The dyn keyword is short for dynamic.

    //let contents = fs::read_to_string(config.file_path)
    //    .expect("Should have been able to read the file");
    let contents = fs::read_to_string(config.file_path)?;
    // Rather than panic! on an error, ? will return the error value from the current function for the caller to handle.

    //println!("With text:\n{contents}");

    let results : Vec<&str> = if config.ignore_case {
        search_case_insensitive(&config.query, &contents).collect()
    } else {
        search(&config.query, &contents).collect()
    };

    //for line in search(&config.query, &contents) {
    for line in results {
        println!("{line}");
    }

    Ok(())
}
