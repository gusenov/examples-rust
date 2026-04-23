fn main() {
    println!("Hello, world!");

    // Rust is a statically typed language, which means that it must know the types of all variables at compile time. 
    // The compiler can usually infer what type we want to use based on the value and how we use it.

    let guess: u32 = "42".parse().expect("Not a number!");
    // If we don’t add the : u32 type annotation shown in the preceding code, Rust will display the following error, which means the compiler needs more information from us to know which type we want to use
    println!("guess = {}", guess);

    // Scalar Types

    // A scalar type represents a single value. 
    // Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

    // Integer Types

    // Signed
    // Signed numbers are stored using two’s complement representation.
    // from -(2^(n-1)) to 2^(n-1) - 1
    let i_i8 = 98_222;  // Decimal
    let i_i16 = 0xff;  // Hex
    let i_i32 = 0o77;  // Octal
    let i_i64 = 0b1111_0000;  // Binary
    let i_i128 = 0i128;
    let i_isize = 0;  // Architecture-dependent
    // 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

    // So how do you know which type of integer to use? 
    // If you’re unsure, Rust’s defaults are generally good places to start: Integer types default to i32.

    // Unsigned
    // from 0 to 2^n - 1
    let u_u8 = b'A';  // Byte (u8 only)
    let u_u16 = 0u16;
    let u_u32 = 0u32;
    let u_u64 = 0u64;
    let u_u128 = 0u128;
    let u_usize = 0;  // Architecture-dependent
    // The primary situation in which you’d use isize or usize is when indexing some sort of collection.

    println!("i_i8 = {}", i_i8);
    println!("i_i16 = {}", i_i16);
    println!("i_i32 = {}", i_i32);
    println!("i_i64 = {}", i_i64);
    println!("i_i128 = {}", i_i128);
    println!("i_isize = {}", i_isize);

    println!("u_u8 = {}", u_u8);
    println!("u_u16 = {}", u_u16);
    println!("u_u32 = {}", u_u32);
    println!("u_u64 = {}", u_u64);
    println!("u_u128 = {}", u_u128);
    println!("u_usize = {}", u_usize);

    // Floating-Point Types
    // Floating-point numbers are represented according to the IEEE-754 standard.

    // The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. 
    let f_f64 = 2.0; // f64

    let f_f32: f32 = 3.0; // f32

    // All floating-point types are signed.

    println!("f_f64 = {}, f_f32 = {}", f_f64, f_f32);

    // Numeric Operations

    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("4 * 30 = {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {}", quotient);
    let truncated = -5 / 3; // Results in -1
    println!("-5 / 3 = {}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {}", remainder);

    // Boolean Type

    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t = {}, f = {}", t, f);

    // Booleans are one byte in size.

    // Character Type

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);

    // Note that we specify char literals with single quotation marks, as opposed to string literals, which use double quotation marks. 
    // Rust’s char type is 4 bytes in size and represents a Unicode scalar value, which means it can represent a lot more than just ASCII. 
    // Accented letters; Chinese, Japanese, and Korean characters; emojis; and zero-width spaces are all valid char values in Rust. 
    // Unicode scalar values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. 
    // However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust.

    // Compound Types
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    // The Tuple Type
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
    // Tuples have a fixed length: Once declared, they cannot grow or shrink in size.

    // The variable tup binds to the entire tuple because a tuple is considered a single compound element.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access.
    println!("The value of tup is: {} {} {}", tup.0, tup.1, tup.2);

    let tup = (500, 6.4, 1);
    // we can use pattern matching to destructure a tuple value
    let (x, y, z) = tup;  // destructuring
    println!("The value of x, y, z is: {x} {y} {z}");

    // The Array Type

    // Unlike a tuple, every element of an array must have the same type. 
    // Unlike arrays in some other languages, arrays in Rust have a fixed length.

    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]);

    // Arrays are useful when you want your data allocated on the stack, the same as the other types we have seen so far, rather than the heap or when you want to ensure that you always have a fixed number of elements. 
    // An array isn’t as flexible as the vector type, though. 
    // A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size because its contents live on the heap. 
    // If you’re unsure whether to use an array or a vector, chances are you should use a vector.

    // However, arrays are more useful when you know the number of elements will not need to change. 
    // For example, if you were using the names of the month in a program, you would probably use an array rather than a vector because you know it will always contain 12 elements:

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("{}", months[0]);

    // You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[0]);

    // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:
    let a = [3; 5];
    println!("{}", a[0]);

    // Array Element Access
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first = {}, second = {}", first, second);

}
