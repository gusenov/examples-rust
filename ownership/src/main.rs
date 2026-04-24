#![allow(unused)]

fn main() {

    // The variable s refers to a string literal, where the value of the string is hardcoded into the text of our program. 

    {                      // s is not valid here, since it's not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid


    // The types covered previously are of a known size, can be stored on the stack and popped off the stack when their scope is over, and can be quickly and trivially copied to make a new, independent instance if another part of code needs to use the same value in a different scope. 
    // But we want to look at data that is stored on the heap and explore how Rust knows when to clean up that data, and the String type is a great example.

    // This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. 

    // The double colon :: operator allows us to namespace this particular from function under the String type rather than using some sort of name like string_from.
    let s = String::from("hello");


    // In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. 
    // This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability. 
    // Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.


    // In languages with a garbage collector (GC), the GC keeps track of and cleans up memory that isn’t being used anymore, and we don’t need to think about it. 
    // In most languages without a GC, it’s our responsibility to identify when memory is no longer being used and to call code to explicitly free it, just as we did to request it. 
    // Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. 
    // If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.

    // Rust takes a different path: The memory is automatically returned once the variable that owns it goes out of scope.
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid

    // When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.

    // Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns.




    // Variables and Data Interacting with Move

    let s1 = String::from("hello");
    let s2 = s1; 
    //println!("{s1}, world!");
    // error[E0382]: borrow of moved value: `s1`
    // You’ll get an error like this because Rust prevents you from using the invalidated reference

    // If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. 
    // But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move. 
    // In this example, we would say that s1 was moved into s2.

    // That solves our problem! With only s2 valid, when it goes out of scope it alone will free the memory, and we’re done.

    // In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. 
    // Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.




    // Scope and Assignment

    let mut s = String::from("hello");
    
    // When you assign a completely new value to an existing variable, Rust will call drop and free the original value’s memory immediately. 
    s = String::from("ahoy");

    println!("{s}, world!");




    // Variables and Data Interacting with Clone

    // If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive. 
    // It’s a visual indicator that something different is going on.




    // Stack-Only Data: Copy

    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.

    // Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are. 
    // If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

    // Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait. 
    // If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error.

    // So, what types implement the Copy trait? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy. 
    // Here are some of the types that implement Copy:
    // - All the integer types, such as u32.
    // - The Boolean type, bool, with values true and false.
    // - All the floating-point types, such as f64.
    // - The character type, char.
    // - Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

    
    

    // Ownership and Functions

    // The mechanics of passing a value to a function are similar to those when assigning a value to a variable. 
    // Passing a variable to a function will move or copy, just as assignment does
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    // If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error. 
    // These static checks protect us from mistakes. 

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.




    // Return Values and Scope
    // Returning values can also transfer ownership.

    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    // The ownership of a variable follows the same pattern every time: Assigning a value to another variable moves it. 
    // When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.


} 
// Here, x goes out of scope, then s. However, because s's value was moved,
// nothing special happens.

// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.




fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.




fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}




// What if we want to let a function use a value but not take ownership? 
// It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

fn foo() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    // Rust does let us return multiple values using a tuple
    (s, length)
}

// But this is too much ceremony and a lot of work for a concept that should be common. 
// Luckily for us, Rust has a feature for using a value without transferring ownership: references.