use std::{fs::File, net::Ipv4Addr};

fn _trying_to_access_an_array_past_the_end_of_its_length() {
    let my_items = vec!["apple", "coin", "keys"];
    println!("{}", my_items[5]);
}
// thread 'main' (273309) panicked at src/main.rs:5:28:
// index out of bounds: the len is 3 but the index is 5
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

fn _get_user(user_id: i32) {
    if user_id < 0 {
        panic!("user_id cannot be negative, got {}", user_id);
    }
    // fetch user from database
}

fn main() {
    //_trying_to_access_an_array_past_the_end_of_its_length();

    //_get_user(-1);
    // thread 'main' (275400) panicked at src/main.rs:13:9:
    // user_id cannot be negative, got -1
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    let filename = "file";
    let _file = File::open(filename).unwrap();
    // panics with: called `Result::unwrap()` on an `Err` value: Os { code: 2,
    // kind: NotFound, message: "No such file or directory" }

    // A slightly better alternative is to use expect().
    // We can add some context to our error message but ultimately we still end up with a panic.

    let _file = File::open(filename).expect(&format!("File '{}' could not be opened", filename));
    // panics with: File 'config.txt' could not be opened: Os { code: 2,
    // kind: NotFound, message: "No such file or directory" }

    let _localhost: Ipv4Addr = "127.0.0.1"
        .parse()
        .expect("hardcoded IP address should be valid");
}
