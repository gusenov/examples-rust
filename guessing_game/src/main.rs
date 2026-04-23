// By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. 
// This set is called the prelude, and you can see everything in it in the standard library documentation.
// https://doc.rust-lang.org/stable/std/prelude/index.html

// If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement. 

use std::cmp::Ordering;

// To obtain user input and then print the result as output, we need to bring the io input/output library into scope. 
// The io library comes from the standard library, known as std:
use std::io;

// The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
use rand::Rng;

// The fn syntax declares a new function; the parentheses, (), indicate there are no parameters; and the curly bracket, {, starts the body of the function.
fn main() {
    println!("Guess the number!");

    // gives us the particular random number generator we’re going to use: one that is local to the current thread of execution and is seeded by the operating system.
    // Then, we call the gen_range method on the random number generator. 
    // This method is defined by the Rng trait that we brought into scope with the use rand::Rng; statement. 
    // The gen_range method takes a range expression as an argument and generates a random number in the range. 
    // The kind of range expression we’re using here takes the form start..=end and is inclusive on the lower and upper bounds, so we need to specify 1..=100 to request a number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // The secret_number, on the other hand, is a number type. 
    // A few of Rust’s number types can have a value between 1 and 100: i32, a 32-bit number; u32, an unsigned 32-bit number; i64, a 64-bit number; as well as others. 
    // Unless otherwise specified, Rust defaults to an i32, which is the type of secret_number unless you add type information elsewhere that would cause Rust to infer a different numerical type. 

    //println!("The secret number is: {secret_number}");

    loop {
        
    println!("Please input your guess.");

    // We use the let statement to create the variable.
    // In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change.
    //let apples = 5; // immutable
    // To make a variable mutable, we add mut before the variable name:
    //let mut bananas = 5; // mutable

    // String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
    // The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String.
    let mut guess = String::new();  // created a mutable variable that is currently bound to a new, empty instance of a String.
    // Rust has a strong, static type system. 
    // However, it also has type inference. 
    // When we wrote let mut guess = String::new(), Rust was able to infer that guess should be a String and didn’t make us write the type.

    // If we hadn’t imported the io module with use std::io; at the beginning of the program, we could still use the function by writing this function call as std::io::stdin. 
    // The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
    io::stdin()
        
        // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
        // like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.
        .read_line(&mut guess)

        // read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value. 
        // Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. 
        // We call each possible state a variant.
        // The purpose of these Result types is to encode error-handling information.
        //
        // Result’s variants are Ok and Err. 
        // The Ok variant indicates the operation was successful, and it contains the successfully generated value. 
        // The Err variant means the operation failed, and it contains information about how or why the operation failed.
        //
        // Values of the Result type, like values of any type, have methods defined on them. An instance of Result has an expect method that you can call. 
        // If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect. 
        // If the read_line method returns an Err, it would likely be the result of an error coming from the underlying operating system. 
        // If this instance of Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so that you can use it. 
        // In this case, that value is the number of bytes in the user’s input.
        .expect("Failed to read line");
        // If you don’t call expect, the program will compile, but you’ll get a warning:
        /*
           Compiling guessing_game v0.1.0 (examples-rust/guessing_game)
        warning: unused `Result` that must be used
          --> src/main.rs:29:5
           |
        29 | /     io::stdin()
        ...  |
        33 | |         .read_line(&mut guess)
           | |______________________________^
           |
           = note: this `Result` may be an `Err` variant, which should be handled
           = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
        help: use `let _ = ...` to ignore the resulting value
           |
        29 |     let _ = io::stdin()
           |     +++++++
        
        warning: `guessing_game` (bin "guessing_game") generated 1 warning
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
             Running `target/debug/guessing_game`
        Guess the number!
        Please input your guess.
        */
        // Rust warns that you haven’t used the Result value returned from read_line, indicating that the program hasn’t handled a possible error.

    // Rust allows us to shadow the previous value of guess with a new one. 
    // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess, for example.
    // 
    // For example, if the user types 5 and presses enter, guess looks like this: 5\n. The \n represents “newline.” 
    // (On Windows, pressing enter results in a carriage return and a newline, \r\n.) The trim method eliminates \n or \r\n, resulting in just 5.
    //
    // The parse method on strings converts a string to another type. Here, we use it to convert from a string to a number.
    // We need to tell Rust the exact number type we want by using let guess: u32. 
    // The colon (:) after guess tells Rust we’ll annotate the variable’s type. 
    // Rust has a few built-in number types; the u32 seen here is an unsigned, 32-bit integer. 
    // It’s a good default choice for a small positive number.
    //
    // Additionally, the u32 annotation in this example program and the comparison with secret_number means Rust will infer that secret_number should be a u32 as well. 
    // So, now the comparison will be between two values of the same type!
    let guess: u32 = 
        //guess.trim()        
        //.parse()        
        // If parse returns an Err Result variant because it couldn’t create a number from the string, the expect call will crash the game and print the message we give it. 
        // If parse can successfully convert the string to a number, it will return the Ok variant of Result, and expect will return the number that we want from the Ok value.
        //.expect("Please type a number!");

        match guess.trim()
        // parse returns a Result type and Result is an enum that has the variants Ok and Err. We’re using a match expression here
        .parse() {
            Ok(num) => num,
            // The underscore, _, is a catch-all value; in this example, we’re saying we want to match all Err values, no matter what information they have inside them.
            Err(_) => continue,
        };

    println!("You guessed: {guess}");

    //let x = 5;
    //let y = 10;
    //println!("x = {x} and y + 2 = {}", y + 2);  // x = 5 and y + 2 = 12

    // The Ordering type is another enum and has the variants Less, Greater, and Equal. 
    // These are the three outcomes that are possible when you compare two values.
    //
    // The cmp method compares two values and can be called on anything that can be compared.
    // returns a variant of the Ordering enum we brought into scope with the use statement.
    // 
    // We use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number.
    //
    // A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. 
    // Rust takes the value given to match and looks through each arm’s pattern in turn.
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }

    }

}