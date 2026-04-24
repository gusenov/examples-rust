#![allow(unused)]

// Slices let you reference a contiguous sequence of elements in a collection. A slice is a kind of reference, so it does not have ownership.

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!




    // String Slices
    // A string slice is a reference to a contiguous sequence of the elements of a String, and it looks like this:
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    // We create slices using a range within square brackets by specifying [starting_index..ending_index], where starting_index is the first position in the slice and ending_index is one more than the last position in the slice. 
    // Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to ending_index minus starting_index. 
    // So, in the case of let world = &s[6..11];, world would be a slice that contains a pointer to the byte at index 6 of s with a length value of 5.

    // With Rust’s .. range syntax, if you want to start at index 0, you can drop the value before the two periods.
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];

    // By the same token, if your slice includes the last byte of the String, you can drop the trailing number. That means these are equal:
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    // You can also drop both values to take a slice of the entire string. So, these are equal:
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];

    // Note: String slice range indices must occur at valid UTF-8 character boundaries. 
    // If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.

    // We now have a straightforward API that’s much harder to mess up because the compiler will ensure that the references into the String remain valid.

    let mut s = String::from("hello world");
    let word = first_word2(&s);
    //s.clear(); // error!
    println!("the first word is: {word}");
    // Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference. 
    // Because clear needs to truncate the String, it needs to get a mutable reference. 
    // The println! after the call to clear uses the reference in word, so the immutable reference must still be active at that point. 
    // Rust disallows the mutable reference in clear and the immutable reference in word from existing at the same time, and compilation fails.




    // String Literals as Slices
    // Recall that we talked about string literals being stored inside the binary. 
    // Now that we know about slices, we can properly understand string literals:
    let s = "Hello, world!";
    // The type of s here is &str: It’s a slice pointing to that specific point of the binary. 
    // This is also why string literals are immutable; &str is an immutable reference.




    // String Slices as Parameters

    // Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word3(&my_string[0..6]);
    let word = first_word3(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word3(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word3(&my_string_literal[0..6]);
    let word = first_word3(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word3(my_string_literal);




    // Other Slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    // This slice has the type &[i32]. 
    // It works the same way as string slices do, by storing a reference to the first element and a length. 

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // iter is a method that returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple instead.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

//fn second_word(s: &String) -> (usize, usize) {

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

//fn second_word(s: &String) -> &str {

//fn first_word(s: &String) -> &str {

// it allows us to use the same function on both &String values and &str values.
fn first_word3(s: &str) -> &str {
    &s[..]
}