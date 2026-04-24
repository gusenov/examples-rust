#![allow(unused)]

fn main() {
    
    // References and Borrowing

    let s1 = String::from("hello");
    
    // The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
    let len = calculate_length(&s1);
    // We call the action of creating a reference borrowing. 
    // As in real life, if a person owns something, you can borrow it from them. 
    // When you’re done, you have to give it back. You don’t own it.

    println!("The length of '{s1}' is {len}.");

    let s = String::from("hello");
    change(&s);  
    // Just as variables are immutable by default, so are references. 
    // We’re not allowed to modify something we have a reference to.



    
    // Mutable References

    let mut s = String::from("hello");
    change_(&mut s);


    // Mutable references have one big restriction: 
    // If you have a mutable reference to a value, you can have no other references to that value.
    let mut s = String::from("hello");
    let r1 = &mut s;
    //let r2 = &mut s;
    // error[E0499]: cannot borrow `s` as mutable more than once at a time
    //println!("{r1}, {r2}");


    // As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;

    // Rust enforces a similar rule for combining mutable and immutable references. 
    // This code results in an error:
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    //let r3 = &mut s; // BIG PROBLEM
    //println!("{r1}, {r2}, and {r3}");
    // We also cannot have a mutable reference while we have an immutable one to the same value.
    // Users of an immutable reference don’t expect the value to suddenly change out from under them! 
    // However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.

    // Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. 
    // For instance, this code will compile because the last usage of the immutable references is in the println!, before the mutable reference is introduced:
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.
    let r3 = &mut s; // no problem
    println!("{r3}");
    // The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the mutable reference r3 is created. 
    // These scopes don’t overlap, so this code is allowed: 
    // The compiler can tell that the reference is no longer being used at a point before the end of the scope.




    // Dangling References
    // In languages with pointers, it’s easy to erroneously create a dangling pointer—a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory. 
    // In Rust, by contrast, the compiler guarantees that references will never be dangling references: 
    // If you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

    //let reference_to_nothing = dangle();
    // this function's return type contains a borrowed value, but there is no value
    // for it to be borrowed from

}

// Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *. 

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.


fn change(some_string: &String) {
    //some_string.push_str(", world");
    // error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
}

fn change_(some_string: &mut String) {
    some_string.push_str(", world");
}


//fn dangle() -> &String { // dangle returns a reference to a String
//
//    let s = String::from("hello"); // s is a new String
//
//    &s // we return a reference to the String, s
//} // Here, s goes out of scope and is dropped, so its memory goes away.
  // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
