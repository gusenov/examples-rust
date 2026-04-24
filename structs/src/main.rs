#![allow(unused)]

// Structs are similar to tuples, in that both hold multiple related values. 
// Like tuples, the pieces of a struct can be different types. 
// Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean. 
// Adding these names means that structs are more flexible than tuples: 
// You don’t have to rely on the order of the data to specify or access the values of an instance.

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");




    // Creating Instances with Struct Update Syntax
    //let user2 = User {
    //    active: user1.active,
    //    username: user1.username,
    //    email: String::from("another@example.com"),
    //    sign_in_count: user1.sign_in_count,
    //};

    // The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // The ..user1 must come last to specify that any remaining fields should get their values from the corresponding fields in user1, 
    // but we can choose to specify values for as many fields as we want in any order, 
    // regardless of the order of the fields in the struct’s definition.

    // Note that the struct update syntax uses = like an assignment; this is because it moves the data
    // In this example, we can no longer use user1 after creating user2 because the String in the username field of user1 was moved into user2. 
    // If we had given user2 new String values for both email and username, and thus only used the active and sign_in_count values from user1, then user1 would still be valid after creating user2. 
    // Both active and sign_in_count are types that implement the Copy trait
    // We can also still use user1.email in this example, because its value was not moved out of user1.




    // Creating Different Types with Tuple Structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // Note that the black and origin values are different types because they’re instances of different tuple structs. 
    // Each struct you define is its own type, even though the fields within the struct might have the same types. 
    // For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values. 
    
    // Otherwise, tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a . followed by the index to access an individual value. 
    // Unlike tuples, tuple structs require you to name the type of the struct when you destructure them.

    // destructure the values in the origin point into variables named x, y, and z
    let Point(x, y, z) = origin;




    // Defining Unit-Like Structs
    let subject = AlwaysEqual;




    // Ownership of Struct Data
    // It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes
    // Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.

    // Let’s say you try to store a reference in a struct without specifying lifetimes, like the following in src/main.rs; this won’t work:

    //let user1 = User2 {
    //    active: true,
    //    username: "someusername123",
    //    email: "someone@example.com",
    //    sign_in_count: 1,
    //};

}

// a struct that stores information about a user account.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// Because the parameter names and the struct field names are exactly the same, 
// we can use the field init shorthand syntax to rewrite build_user 
// so that it behaves exactly the same but doesn’t have the repetition of username and email
fn build_user2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}




// Rust also supports structs that look similar to tuples, called tuple structs. 
// Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. 
// Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);




// Defining Unit-Like Structs
// You can also define structs that don’t have any fields! 
// These are called unit-like structs because they behave similarly to (), the unit type
// Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
struct AlwaysEqual;




// Ownership of Struct Data

//struct User2 {
//    active: bool,
//    username: &str,
//    email: &str,
//    sign_in_count: u64,
//}
