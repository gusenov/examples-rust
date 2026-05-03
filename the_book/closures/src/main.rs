#![allow(unused)]

use std::time::Duration;
use std::thread;




#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );




    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };




    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // The add_one_v3 and add_one_v4 lines require the closures to be evaluated to be able to compile because the types will be inferred from their usage. 
    // This is similar to let v = Vec::new(); needing either type annotations or values of some type to be inserted into the Vec for Rust to be able to infer the type.
    //let add_one_v3 = |x|             { x + 1 };
    //let add_one_v4 = |x|               x + 1  ;



    let example_closure = |x| x;
    // The first time we call example_closure with the String value, the compiler infers the type of x and the return type of the closure to be String.
    let s = example_closure(String::from("hello"));
    //If we then try to call example_closure with an integer, we’ll get an error.
    //let n = example_closure(5);




    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");




    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {list:?}");

    


    //  If the main thread performed more operations before calling join on the new thread, the new thread might finish before the rest of the main thread finishes, or the main thread might finish first. 
    // If the main thread maintained ownership of list but ended before the new thread and drops list, the immutable reference in the thread would be invalid. 
    // Therefore, the compiler requires that list be moved into the closure given to the new thread so that the reference will be valid.
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();




    // Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:
    // - FnOnce applies to closures that can be called once. 
    //   All closures implement at least this trait because all closures can be called. 
    //   A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits because it can only be called once.
    // - FnMut applies to closures that don’t move captured values out of their body but might mutate the captured values. 
    //   These closures can be called more than once.
    // - Fn applies to closures that don’t move captured values out of their body and don’t mutate captured values, as well as closures that capture nothing from their environment. 
    //   These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.


    // The reason sort_by_key is defined to take an FnMut closure is that it calls the closure multiple times: once for each item in the slice. 
    // The closure |r| r.width doesn’t capture, mutate, or move anything out from its environment, so it meets the trait bound requirements.
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    list.sort_by_key(|r| r.width);
    println!("{list:#?}");




    // an example of a closure that implements just the FnOnce trait, because it moves a value out of the environment. 
    // The compiler won’t let us use this closure with sort_by_key.
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    //let mut sort_operations = vec![];
    let mut num_sort_operations = 0;

    let value = String::from("closure called");
    list.sort_by_key(|r| {
        
        // The closure captures value and then moves value out of the closure by transferring ownership of value to the sort_operations vector. 
        // This closure can be called once; trying to call it a second time wouldn’t work, because value would no longer be in the environment to be pushed into sort_operations again! 
        // Therefore, this closure only implements FnOnce. 
        // When we try to compile this code, we get this error that value can’t be moved out of the closure because the closure must implement FnMut
        //sort_operations.push(value);
        
        // To fix this, we need to change the closure body so that it doesn’t move values out of the environment. 
        // Keeping a counter in the environment and incrementing its value in the closure body is a more straightforward way to count the number of times the closure is called.
        num_sort_operations += 1;

        r.width
    });
    
    //println!("{list:#?}");
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}