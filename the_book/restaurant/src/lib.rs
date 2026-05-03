#![allow(unused)]

// The front_of_house module is defined in the crate root. 
// While front_of_house isn’t public, 
// because the eat_at_restaurant function is defined in the same module as front_of_house 
// (that is, eat_at_restaurant and front_of_house are siblings), 
// we can refer to front_of_house from eat_at_restaurant. 
mod front_of_house;
// Note that you only need to load a file using a mod declaration once in your module tree. 
// Once the compiler knows the file is part of the project (and knows where in the module tree the code resides because of where you’ve put the mod statement), other files in your project should refer to the loaded file’s code using a path to where it was declared
// In other words, mod is not an “include” operation that you may have seen in other programming languages.




// bring the crate::front_of_house::hosting module into the scope of the eat_at_restaurant function
//use crate::front_of_house::hosting;
// Adding use and a path in a scope is similar to creating a symbolic link in the filesystem.
// By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that scope, just as though the hosting module had been defined in the crate root.
// Paths brought into scope with use also check privacy, like any other paths.
//
// Note that use only creates the shortcut for the particular scope in which the use occurs.

//use crate::front_of_house::hosting::add_to_waitlist;
// Bringing the function’s parent module into scope with use means we have to specify the parent module when calling the function. 
// Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path.

// On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path.
use std::collections::HashMap;
//fn main() {
//    let mut map = HashMap::new();
//    map.insert(1, 2);
//}

// The exception to this idiom is if we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that.
//use std::fmt;
//use std::io;
//fn function1() -> fmt::Result {
//    // --snip--
//}
//fn function2() -> io::Result<()> {
//    // --snip--
//}

// Providing New Names with the as Keyword
//use std::fmt::Result;
//use std::io::Result as IoResult;
//fn function1() -> Result {
//    // --snip--
//}
//fn function2() -> IoResult<()> {
//    // --snip--
//}

// Re-exporting Names with pub use
// When we bring a name into scope with the use keyword, the name is private to the scope into which we imported it. 
// To enable code outside that scope to refer to that name as if it had been defined in that scope, we can combine pub and use. 
// This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.
pub use crate::front_of_house::hosting;
// Before this change, external code would have to call the add_to_waitlist function by using the path restaurant::front_of_house::hosting::add_to_waitlist(), 
// which also would have required the front_of_house module to be marked as pub. 
// Now that this pub use has re-exported the hosting module from the root module, 
// external code can use the path restaurant::hosting::add_to_waitlist() instead.
//
// Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. 
// For example, in this restaurant metaphor, the people running the restaurant think about “front of house” and “back of house.” 
// But customers visiting a restaurant probably won’t think about the parts of the restaurant in those terms. 
// With pub use, we can write our code with one structure but expose a different structure. 
// Doing so makes our library well organized for programmers working on the library and programmers calling the library.

// Using Nested Paths to Clean Up use Lists
//use std::cmp::Ordering;
//use std::io;
//use std::{cmp::Ordering, io};

//use std::io;
//use std::io::Write;
use std::io::{self, Write};

// Importing Items with the Glob Operator
use std::collections::*;




// part of our library crate’s public API, so we mark it with the pub keyword.
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Choosing whether to use a relative or absolute path is a decision you’ll make based on your project, 
    // and it depends on whether you’re more likely to move item definition code separately from or together with the code that uses the item. 




    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");




    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;


    hosting::add_to_waitlist();

}

mod customer {
    pub fn eat_at_restaurant() {
        //hosting::add_to_waitlist();
        // The compiler error shows that the shortcut no longer applies within the customer module
        // 
        // To fix this problem, move the use within the customer module too, 
        // or reference the shortcut in the parent module with super::hosting within the child customer module.
    }
}

fn deliver_order() {}

mod back_of_house {

    // models the situation in which a chef fixes an incorrect order and personally brings it out to the customer. 
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();

        // We think the back_of_house module and the deliver_order function are likely to stay in the same relationship to each other and get moved together should we decide to reorganize the crate’s module tree. 
        // Therefore, we used super so that we’ll have fewer places to update code in the future if this code gets moved to a different module.
    }

    fn cook_order() {}

    pub struct Breakfast {
        // This models the case in a restaurant where the customer can pick the type of bread that comes with a meal, 
        // but the chef decides which fruit accompanies the meal based on what’s in season and in stock. 
        // The available fruit changes quickly, so customers can’t choose the fruit or even see which fruit they’ll get.
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // if we make an enum public, all of its variants are then public.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public. 
    // Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub.

}

//pub fn add(left: u64, right: u64) -> u64 {
//    left + right
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}
