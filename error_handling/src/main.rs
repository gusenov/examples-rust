#![allow(unused)]

// Rust groups errors into two major categories: recoverable and unrecoverable errors. 
// For a recoverable error, such as a file not found error, we most likely just want to report the problem to the user and retry the operation. 
// Unrecoverable errors are always symptoms of bugs, such as trying to access a location beyond the end of an array, and so we want to immediately stop the program.

// Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions. 
// Rust doesn’t have exceptions. 
// Instead, it has the type Result<T, E> for recoverable errors 
// and the panic! macro that stops execution when the program encounters an unrecoverable error.




//enum Result<T, E> {
//    Ok(T),
//    Err(E),
//}
// Note that, like the Option enum, the Result enum and its variants have been brought into scope by the prelude, 
// so we don’t need to specify Result:: before the Ok and Err variants in the match arms.

use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::io::ErrorKind;
use std::error::Error;

//fn main() {
fn main() -> Result<(), Box<dyn Error>> {
// you can read Box<dyn Error> to mean “any kind of error.” 




    //panic!("crash and burn");




    let v = vec![1, 2, 3];
    //v[99];

    // In C, attempting to read beyond the end of a data structure is undefined behavior. 
    // You might get whatever is at the location in memory that would correspond to that element in the data structure, even though the memory doesn’t belong to that structure. 
    // This is called a buffer overread and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn’t be allowed to that is stored after the data structure.

    // To protect your program from this sort of vulnerability, if you try to read an element at an index that doesn’t exist, Rust will stop execution and refuse to continue.




    let greeting_file_result = File::open("hello.txt");
    //let greeting_file = match greeting_file_result {
    //    Ok(file) => file,
    //    Err(error) => panic!("Problem opening the file: {error:?}"),
    //};

    //let greeting_file = match greeting_file_result {
    //    Ok(file) => file,
    //    Err(error) => match error.kind() {
    //        ErrorKind::NotFound => match File::create("hello.txt") {
    //            Ok(fc) => fc,
    //            Err(e) => panic!("Problem creating the file: {e:?}"),
    //        },
    //        _ => {
    //            panic!("Problem opening the file: {error:?}");
    //        }
    //    },
    //};

    // here’s another way to write the same logic as shown in Listing 9-5, this time using closures and the unwrap_or_else method:
    //let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //    if error.kind() == ErrorKind::NotFound {
    //        File::create("hello.txt").unwrap_or_else(|error| {
    //            panic!("Problem creating the file: {error:?}");
    //        })
    //    } else {
    //        panic!("Problem opening the file: {error:?}");
    //    }
    //});

    // If the Result value is the Ok variant, unwrap will return the value inside the Ok. 
    // If the Result is the Err variant, unwrap will call the panic! macro for us. 
    //let greeting_file = File::open("hello.txt").unwrap();

    // expect method lets us also choose the panic! error message.
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
    // We use expect in the same way as unwrap: to return the file handle or call the panic! macro.
    // In production-quality code, most Rustaceans choose expect rather than unwrap and give more context about why the operation is expected to always succeed. 
    // That way, if your assumptions are ever proven wrong, you have more information to use in debugging.

    // The ? operator can only be used in functions whose return type is compatible with the value the ? is used on. 
    // This is because the ? operator is defined to perform an early return of a value out of the function, in the same manner as the match expression
    //let greeting_file = File::open("hello.txt")?;
    // we’re only allowed to use the ? operator in a function that returns Result, Option, or another type that implements FromResidual.
    // To fix the error, you have two choices. 
    // One choice is to change the return type of your function to be compatible with the value you’re using the ? operator on as long as you have no restrictions preventing that. 
    // The other choice is to use a match or one of the Result<T, E> methods to handle the Result<T, E> in whatever way is appropriate.


    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

// Propagating Errors
// reads a username from a file. 
// If the file doesn’t exist or can’t be read, this function will return those errors to the code that called the function.
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// A function that returns errors to the calling code using the ? operator
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;

    // Error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert values from one type into another. 
    // When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function. 
    // This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.

    // For example, we could change the read_username_from_file function in Listing 9-7 to return a custom error type named OurError that we define. 
    // If we also define impl From<io::Error> for OurError to construct an instance of OurError from an io::Error, then the ? operator calls in the body of read_username_from_file will call from and convert the error types without needing to add any more code to the function.

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// chaining method calls immediately after the ?
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// fs::read_to_string function that opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// As with using ? on Result, you can only use ? on Option in a function that returns an Option. 
// The behavior of the ? operator when called on an Option<T> is similar to its behavior when called on a Result<T, E>: If the value is None, the None will be returned early from the function at that point. 
// If the value is Some, the value inside the Some is the resultant value of the expression, and the function continues.
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// Note that you can use the ? operator on a Result in a function that returns Result, and you can use the ? operator on an Option in a function that returns Option, but you can’t mix and match. The ? operator won’t automatically convert a Result to an Option or vice versa; in those cases, you can use methods like the ok method on Result or the ok_or method on Option to do the conversion explicitly.

