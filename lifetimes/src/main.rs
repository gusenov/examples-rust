#![allow(unused)]

// every reference in Rust has a lifetime, which is the scope for which that reference is valid. 
// Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. 
// We are only required to annotate types when multiple types are possible. 
// In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. 
// Rust requires us to annotate the relationships using generic lifetime parameters to ensure that the actual references used at runtime will definitely be valid.




// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. 
// Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.




//fn longest(x: &str, y: &str) -> &str {
// return type needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to x or y. 
// Actually, we don’t know either, because the if block in the body of this function returns a reference to x and the else block returns a reference to y!

// The returned reference will be valid as long as both of the parameters are valid.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}




struct ImportantExcerpt<'a> {
    part: &'a str,
}




//fn first_word<'a>(s: &'a str) -> &'a str {
// borrow checker could infer the lifetimes in these situations and wouldn’t need explicit annotations.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

// The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations. 
// The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. 
// If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error. 
// These rules apply to fn definitions as well as impl blocks.

// 1. The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
// 2. The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
// 3. The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.




// In Method Definitions

// because of the first elision rule, we’re not required to annotate the lifetime of the reference to self.
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes. 
// Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}




use std::fmt::Display;
// Let’s briefly look at the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function!
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}




fn main() {
    // At first glance, this might appear to be in conflict with Rust having no null values. 
    // However, if we try to use a variable before giving it a value, we’ll get a compile-time error, which shows that indeed Rust does not allow null values.

    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    //println!("r: {r}");   //          |




    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |




    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");




    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }



    // shows that the lifetime of the reference in result must be the smaller lifetime of the two arguments.
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    //println!("The longest string is {result}");




    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };



    
    // One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program. 
    // All string literals have the 'static lifetime, which we can annotate as follows:
    let s: &'static str = "I have a static lifetime.";
}                         // ---------+
