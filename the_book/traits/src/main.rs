#![allow(unused)]

// A trait defines the functionality a particular type has and can share with other types. 
// We can use traits to define shared behavior in an abstract way. 
// We can use trait bounds to specify that a generic type can be any type that has certain behavior.

// Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.

// One restriction to note is that we can implement a trait on a type only if either the trait or the type, or both, are local to our crate.
// But we can’t implement external traits on external types.




// A Summary trait that consists of the behavior provided by a summarize method
pub trait Summary {
    //fn summarize(&self) -> String;

    // Defining a Summary trait with a default implementation of the summarize method
    //fn summarize(&self) -> String {
    //    String::from("(Read more...)")
    //}

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//impl Summary for NewsArticle {
//    fn summarize(&self) -> String {
//        format!("{}, by {} ({})", self.headline, self.author, self.location)
//    }
//}

// To use a default implementation to summarize instances of NewsArticle, we specify an empty impl block
//impl Summary for NewsArticle {}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

//impl Summary for SocialPost {
//    fn summarize(&self) -> String {
//        format!("{}: {}", self.username, self.content)
//    }
//}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}




// Trait Bound Syntax

pub fn notify(item: &impl Summary) {
// The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound; it looks like this:
//pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Using impl Trait is appropriate if we want this function to allow item1 and item2 to have different types (as long as both types implement Summary). 
pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
}

// If we want to force both parameters to have the same type, however, we must use a trait bound, like this:
pub fn notify1<T: Summary>(item1: &T, item2: &T) {
}




// Multiple Trait Bounds with the + Syntax
use std::fmt::Display;
pub fn notify_(item: &(impl Summary + Display)) {
}

pub fn notify__<T: Summary + Display>(item: &T) {
}




// Clearer Trait Bounds with where Clauses
use std::fmt::Debug;
// So, instead of writing this:
//fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// we can use a where clause, like this:
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}




// Returning Types That Implement Traits
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}

// However, you can only use impl Trait if you’re returning a single type.
fn returns_summarizable_(switch: bool) -> impl Summary {
    //if switch {
    //    NewsArticle {
    //        headline: String::from(
    //            "Penguins win the Stanley Cup Championship!",
    //        ),
    //        location: String::from("Pittsburgh, PA, USA"),
    //        author: String::from("Iceburgh"),
    //        content: String::from(
    //            "The Pittsburgh Penguins once again are the best \
    //             hockey team in the NHL.",
    //        ),
    //    }
    //} else {
        SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            repost: false,
        }
    //}
}


// Using Trait Bounds to Conditionally Implement Methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements another trait. 
// Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are used extensively in the Rust standard library. 
// For example, the standard library implements the ToString trait on any type that implements the Display trait. 
// The impl block in the standard library looks similar to this code:
//impl<T: Display> ToString for T {
//    // --snip--
//}




fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };
    println!("1 new post: {}", post.summarize());

    //let article = NewsArticle {
    //    headline: String::from("Penguins win the Stanley Cup Championship!"),
    //    location: String::from("Pittsburgh, PA, USA"),
    //    author: String::from("Iceburgh"),
    //    content: String::from(
    //        "The Pittsburgh Penguins once again are the best \
    //         hockey team in the NHL.",
    //    ),
    //};
    //println!("New article available! {}", article.summarize());

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };
    println!("1 new post: {}", post.summarize());

}