#![allow(unused)]

fn main() {
    // Patterns can be made up of literal values, variable names, wildcards, and many other things
    // The power of match comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.

    // The Option<T> match Pattern
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);




    // Catch-All Patterns

    let dice_roll = 9;
    //  you roll a 3 on a dice roll, your player doesn’t move but instead gets a fancy new hat. 
    // If you roll a 7, your player loses a fancy hat. 
    // For all other values, your player moves that number of spaces on the game board.
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // _ Placeholder
    // if you roll anything other than a 3 or a 7, you must roll again. 
    // We no longer need to use the catch-all value, so we can change our code to use _ instead of the variable named other:
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    // nothing else happens on your turn if you roll anything other than a 3 or a 7. 
    // We can express that by using the unit value as the code that goes with the _ arm:
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}




enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}




fn value_in_cents2(coin: Coin) -> u8 {
    match coin {
        // If you want to run multiple lines of code in a match arm, you must use curly brackets
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}




#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

// A Coin enum in which the Quarter variant also holds a UsState value
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents3(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}




fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// We didn’t handle the None case, so this code will cause a bug.
//fn plus_one(x: Option<i32>) -> Option<i32> {
//    match x {
//        Some(i) => Some(i + 1),
//    }
//}




fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
