#![allow(unused)]

// enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
    // we’ve created a variable m that has the value Message::Write(String::from("hello")), and that is what self will be in the body of the call method when m.call() runs.




    // The Option Enum

    // The Option type encodes the very common scenario in which a value could be something, or it could be nothing.
    
    // Rust doesn’t have the null feature that many other languages have. 
    // Null is a value that means there is no value there. 
    // In languages with null, variables can always be in one of two states: null or not-null.

    // The Option<T> enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly. 
    // Its variants are also included in the prelude: You can use Some and None directly without the Option:: prefix. 
    // The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>.

    let some_number = Some(5);  // The type of some_number is Option<i32>.
    let some_char = Some('e');  // The type of some_char is Option<char>

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // For example, this code won’t compile, because it’s trying to add an i8 to an Option<i8>:
    //let sum = x + y;
}

fn route(ip_kind: IpAddrKind) {}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// However, representing the same concept using just an enum is more concise: Rather than an enum inside a struct, we can put data directly into each enum variant. 
// This new definition of the IpAddr enum says that both V4 and V6 variants will have associated String values:
enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}




// Let’s look at how the standard library defines IpAddr.

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr4 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}




enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// The following structs could hold the same data that the preceding enum variants hold:
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
// But if we used the different structs, each of which has its own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message enum

// we’re also able to define methods on enums
impl Message {
    
    // The body of the method would use self to get the value that we called the method on.
    fn call(&self) {
        // method body would be defined here
    }
}