// The ability to run some code depending on whether a condition is true and the ability to run some code repeatedly while a condition is true are basic building blocks in most programming languages. 
// The most common constructs that let you control the flow of execution of Rust code are if expressions and loops.

fn main() {
    // if Expressions

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // condition in this code must be a bool. If the condition isn’t a bool, we’ll get an error. 
    // Rust will not automatically try to convert non-Boolean types to a Boolean.
    //if number {
    //    println!("number was three");
    //}

    if number != 0 {
        println!("number was something other than zero");
    }




    // Handling Multiple Conditions with else if

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code.




    // Using if in a let Statement

    let condition = true;
    // blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions.
    // the value of the whole if expression depends on which block of code executes.
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    //let condition = true;
    // If the types are mismatched, as in the following example, we’ll get an error:
    //let number = if condition { 5 } else { "six" };
    // variables must have a single type, and Rust needs to know definitively at compile time what type the number variable is.
    // Knowing the type of number lets the compiler verify the type is valid everywhere we use number. 
    // Rust wouldn’t be able to do that if the type of number was only determined at runtime; 
    // the compiler would be more complex and would make fewer guarantees about the code if it had to keep track of multiple hypothetical types for any variable.
    //println!("The value of number is: {number}");




    // Repetition with Loops

    //loop {
    //    println!("again!");
    //}

    // Returning Values from Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
            // You can also return from inside a loop. 
            // While break only exits the current loop, return always exits the current function.
        }
    };
    println!("The result is {result}");




    // Disambiguating with Loop Labels

    // If you have loops within loops, break and continue apply to the innermost loop at that point. 
    // You can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. 
    // Loop labels must begin with a single quote. Here’s an example with two nested loops:
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");




    // Streamlining Conditional Loops with while
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");




    // Looping Through a Collection with for

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {  // rev, to reverse the range
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
