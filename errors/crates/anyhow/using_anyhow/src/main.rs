use std::fs::File;
use std::io::Read;
use std::net::Ipv4Addr;

// You can think of anyhow::Result<T> as filling the same role as Result<T, Box<dyn std::error::Error>>, but with much better ergonomics and built-in context support.
// It’s a type alias for Result<T, anyhow::Error> - but you rarely need to think about that, unless you want to!
use anyhow::{anyhow, Context, Result};

// It’s a bit of a pain to write function signatures like this:
//fn my_function(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
// Box<dyn std::error::Error> to allow any error type generated within the function to be returned. 

    // ...
//}

// This was great because we had contextual error messages for what had gone wrong at each stage.
fn _read_number_from_file_v1(filename: &str) -> Result<i32, String> {
    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => return Err(format!("could not open file '{}': {}", filename, e)),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(e) => return Err(format!("could not read file '{}': {}", filename, e)),
    };

    match contents.trim().parse::<i32>() {
        Ok(n) => Ok(n),
        Err(e) => Err(format!("could not parse '{}' as integer: {}", contents.trim(), e)),
    }
}

// More ergonomic but we lost the context around the errors.
fn _read_number_from_file_v2(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number = contents.trim().parse::<i32>()?;
    Ok(number)
}

fn _read_number_from_file_v3(filename: &str) -> Result<i32> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number = contents.trim().parse::<i32>()?;
    Ok(number)
}

fn _read_number_from_file_v4(filename: &str) -> Result<i32> {
    let mut file = File::open(filename)
        // with_context(|| format!("could not open '{}'", filename)) - for dynamic messages that include variables 
        // (it takes a closure, so the message is only constructed if there’s actually an error).
        .with_context(|| format!("could not open file '{}'", filename))?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .with_context(|| format!("could not read file '{}'", filename))?;
        
    let number = contents.trim().parse::<i32>()
        // .context("something went wrong") - for simple, static messages
        .context("could not parse file contents as i32")?;
    
    Ok(number)
}

// We’ve made use of static error messages with .context(), 
// dynamic error messages with .with_context(), 
// and our own custom errors with anyhow!:
fn _read_number_from_file_v5(filename: &str) -> Result<i32> {
    let mut file = File::open(filename)
        .with_context(|| format!("could not open file '{}'", filename))?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .with_context(|| format!("could not read file '{}'", filename))?;
    
    let number = contents.trim().parse::<i32>()
        .context("could not parse file contents as i32")?;

    if number < 1 {
        return Err(anyhow!("expected a number over 0, but got {}", number));
    }
    
    Ok(number)
}

fn _foo(number: i32) -> Result<i32> {
    if number < 1 {
        // The anyhow! macro lets us create an error from scratch. 
        return Err(anyhow!("expected a number over 0, but got {}", number));
    }
    Ok(number)
}

fn main() {
    println!("Hello, world!");
}

fn _read_addr_from_file(filename: &str) -> Result<String> {
    let mut file = File::open(filename)
        .with_context(|| format!("could not open file '{}'", filename))?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .with_context(|| format!("could not read file '{}'", filename))?;
    
    let port = contents.trim().parse::<i32>()
        .context("could not parse file contents as i32")?;

    if port < 1000 {
        return Err(anyhow!("expected a port over 1000, but got {}", port));
    }

    let localhost: Ipv4Addr = "127.0.0.1"
       .parse()
    //    .expect("hardcoded IP address should be valid");
    // we could have used .context() or the anyhow! macro here instead:
        .context("hardcoded IP address should be valid")?;


    let addr = format!("{localhost}:{port}");
    
    Ok(addr)
}
