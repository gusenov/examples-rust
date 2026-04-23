fn main() {
    // Variables and Mutability

    let x = 5;
    println!("The value of x is: {x}");
    //x = 6;  // immutability error
    println!("The value of x is: {x}");

    // Declaring Constants

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // example of a constant declaration:
    //const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // Rust’s naming convention for constants is to use all uppercase with underscores between words. 
    // The compiler is able to evaluate a limited set of operations at compile time, which lets us choose to write out this value in a way that’s easier to understand and verify, rather than setting this constant to the value 10,800.

    // Shadowing

    let x = 5;
    // By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have completed.
    let x = x + 1;
    {
        // overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    //let mut spaces = "   ";
    //spaces = spaces.len();  // compile-time error

    // when we use the let keyword again, we can change the type of the value but reuse the same name.
    //let spaces = "   ";  // string type
    //let spaces = spaces.len();  // number type


}
