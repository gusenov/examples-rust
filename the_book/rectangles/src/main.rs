#![allow(unused)]

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );




    // Refactoring with Tuples
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );




    // Refactoring with Structs
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    //println!("rect1 is {rect1}");
    // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`

    println!("rect1 is {rect1:?}");
    // Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug. 
    // The Debug trait enables us to print our struct in a way that is useful for developers so that we can see its value while we’re debugging our code.

    // It’s not the prettiest output, but it shows the values of all the fields for this instance, which would definitely help during debugging. 
    // When we have larger structs, it’s useful to have output that’s a bit easier to read; in those cases, we can use {:#?} instead of {:?} in the println! string. 

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );




    // Another way to print out a value using the Debug format is to use the dbg! macro, 
    // which takes ownership of an expression (as opposed to println!, which takes a reference), 
    // prints the file and line number of where that dbg! macro call occurs in your code along with the resultant value of that expression, 
    // and returns ownership of the value.

    // Note: Calling the dbg! macro prints to the standard error console stream (stderr), as opposed to println!, which prints to the standard output console stream (stdout).

    dbg!(&rect1);
    let scale = 2;
    let rect1 = Rectangle {
        // We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of the expression’s value, the width field will get the same value as if we didn’t have the dbg! call there.
        width: dbg!(30 * scale),
        height: 50,
    };

    // We don’t want dbg! to take ownership of rect1, so we use a reference to rect1 in the next call.
    dbg!(&rect1);




    // Method Syntax

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // To call this associated function, we use the :: syntax with the struct name;
    let sq = Rectangle::square(3);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}




// Refactoring with Tuples
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}




// Refactoring with Structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Our area function is very specific: It only computes the area of rectangles. 
// It would be helpful to tie this behavior more closely to our Rectangle struct because it won’t work with any other type. 
// Let’s look at how we can continue to refactor this code by turning the area function into an area method defined on our Rectangle type.

// Methods are similar to functions: 
// We declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else. 
// Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and their first parameter is always self, which represents the instance of the struct the method is being called on.

// Everything within this impl block will be associated with the Rectangle type.
impl Rectangle {

    // In the signature for area, we use &self instead of rectangle: &Rectangle. 
    // The &self is actually short for self: &Self. 
    // Within an impl block, the type Self is an alias for the type that the impl block is for. 
    // Methods must have a parameter named self of type Self for their first parameter, so Rust lets you abbreviate this with only the name self in the first parameter spot. 
    // 
    // Note that we still need to use the & in front of the self shorthand to indicate that this method borrows the Self instance, just as we did in rectangle: &Rectangle. 
    // Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably, just as they can any other parameter.

    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Note that we can choose to give a method the same name as one of the struct’s fields.
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl. 
    // We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with.

    // Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct. 
    // These are often called new, but new isn’t a special name and isn’t built into the language.
}

// Each struct is allowed to have multiple impl blocks.
impl Rectangle {
    // The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword, which in this case is Rectangle.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// The main reason for using methods instead of functions, in addition to providing method syntax and not having to repeat the type of self in every method’s signature, is for organization. 
// We’ve put all the things we can do with an instance of a type in one impl block rather than making future users of our code search for capabilities of Rectangle in various places in the library we provide.

// Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing. 
// Calling methods is one of the few places in Rust with this behavior.
//
// Here’s how it works: When you call a method with object.something(), Rust automatically adds in &, &mut, or * so that object matches the signature of the method. 
// In other words, the following are the same:
//
// p1.distance(&p2);
// (&p1).distance(&p2);
//
// The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of self. 
// Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self). 
// The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.
