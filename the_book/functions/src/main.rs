// Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words. 

fn main() {
    println!("Hello, world!");

    // Functions

    another_function();




    // Parameters

    another_func(5);

    print_labeled_measurement(5, 'h');




    // Statements and Expressions

    // Rust is an expression-based language

    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value.

    // Creating a variable and assigning a value to it with the let keyword is a statement.
    let y = 6;
    println!("The value of y is: {y}");

    // Function definitions are also statements

    // calling a function is not a statement

    // Statements do not return values. 
    // Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error:
    //let x = (let y = 6);
    // The let y = 6 statement does not return a value, so there isn’t anything for x to bind to.

    // the 6 in the statement let y = 6; is an expression that evaluates to the value 6. 
    // Calling a function is an expression. 
    // Calling a macro is an expression. 
    // A new scope block created with curly brackets is an expression
    let y = 
    
    // expression
    {
        let x = 3;
        x + 1
        // Expressions do not include ending semicolons. 
        // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. 
        // Keep this in mind as you explore function return values and expressions
    };

    println!("The value of y is: {y}");




    // Functions with Return Values

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");

}

// Note that we defined another_function after the main function in the source code; we could have defined it before as well. 
// Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.
fn another_function() {
    println!("Another function.");
}

// We can define functions to have parameters, which are special variables that are part of a function’s signature. 
// When a function has parameters, you can provide it with concrete values for those parameters. 
// Technically, the concrete values are called arguments, but in casual conversation, people tend to use the words parameter and argument interchangeably for either the variables in a function’s definition or the concrete values passed in when you call a function.

// In function signatures, you must declare the type of each parameter. 
// This is a deliberate decision in Rust’s design: 
// Requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean. 
// The compiler is also able to give more-helpful error messages if it knows what types the function expects.

fn another_func(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5  // with no semicolon because it’s an expression whose value we want to return
    
    // There are no function calls, macros, or even let statements in the five function—just the number 5 by itself. 
    // That’s a perfectly valid function in Rust. 
}

fn plus_one(x: i32) -> i32 {
    x + 1

    // But what happens if we place a semicolon at the end of the line containing x + 1, changing it from an expression to a statement?

    // The definition of the function plus_one says that it will return an i32, but statements don’t evaluate to a value, which is expressed by (), the unit type. 
    // Therefore, nothing is returned, which contradicts the function definition and results in an error.
}
