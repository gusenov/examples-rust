use std::fs::File;
use std::io::{Read};
use std::num::ParseIntError;

//fn get_username(user_id: i32) -> Result<String, std::io::Error> {
//    // Either returns Ok(String) on success
//    // or Err(std::io::Error) on failure
//}

fn _parse_user_age(input: &str) -> Result<i32, ParseIntError> {
    input.trim().parse::<i32>()
}

fn _read_number_from_file_v1(filename: &str) -> Result<i32, String> {
    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => return Err(format!("Could not open file '{}': {}", filename, e)),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(e) => return Err(format!("Could not read file '{}': {}", filename, e)),
    };

    match contents.trim().parse::<i32>() {
        Ok(n) => Ok(n),
        Err(e) => Err(format!("Could not parse '{}' as integer: {}", contents.trim(), e)),
    }
}

// This is much cleaner.
//
// ? needs to convert different error types (io::Error, ParseIntError) into a common type. 
// Box<dyn std::error::Error> is a trait object that can hold any error - it’s the quick and dirty solution.
//
// The downside is that we have lost our custom messages. 
// The caller is now going to get the raw underlying errors.
fn _read_number_from_file_v2(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number = contents.trim().parse::<i32>()?;
    Ok(number)
}


fn main() {
    let _age_result1 = _parse_user_age("32"); // Returns Ok(i32)
    
    let input = "twenty-five";
    let _age_result2 = _parse_user_age(input); // Returns  Err(ParseIntError)

    let _age_result3 = _parse_user_age(input); // Returns  Err(ParseIntError)

    // Example 1
    match _age_result1 {
        Ok(age) => println!("User is {} years old", age),
        Err(e) => panic!("Invalid age provided: {}", e)
    }

    // Example 2
    match _age_result2 {
        Ok(age) => println!("User is {} years old", age),
        Err(e) => println!("Invalid age provided: {}", e),
    }

    // Example 3
    let _result: Result<i32, String> = match _age_result3 {
        Ok(age) => Ok(age),
        Err(e) => Err(format!("Could not parse '{}' as integer: {}", input, e)),
    };
}
