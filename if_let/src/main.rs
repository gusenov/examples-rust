#![allow(unused)]

fn main() {

    // you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.

    // If you have a situation in which your program has logic that is too verbose to express using a match, remember that if let and let...else are in your Rust toolbox as well.
    
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // The syntax if let takes a pattern and an expression separated by an equal sign. 
    // It works the same way as a match, where the expression is given to the match and the pattern is its first arm. 
    // In this case, the pattern is Some(max), and the max binds to the value inside the Some. 
    // We can then use max in the body of the if let block in the same way we used max in the corresponding match arm. 
    // The code in the if let block only runs if the value matches the pattern.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }




    let coin = Coin::Quarter(UsState::Alaska);
    
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

}

#[derive(Clone, Copy, Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}




impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn describe_state_quarter2(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

// If the pattern matches, it will bind the value from the pattern in the outer scope. 
// If the pattern does not match, the program will flow into the else arm, which must return from the function.
fn describe_state_quarter3(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}