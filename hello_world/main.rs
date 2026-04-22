// The main function is special: It is always the first code that runs in every executable Rust program.

fn main() {
    
    // println! calls a Rust macro. 
    // If it had called a function instead, it would be entered as println (without the !). 
    // Rust macros are a way to write code that generates code to extend Rust syntax

    // using a ! means that you’re calling a macro instead of a normal function and that macros don’t always follow the same rules as functions.

    println!("Hello, world!")
}