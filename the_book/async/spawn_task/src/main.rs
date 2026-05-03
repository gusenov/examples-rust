#![allow(unused)]

use std::time::Duration;

// Creating a new task to print one thing while the main task prints something else

fn main() {
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // Using await with a join handle to run a task to completion
        handle.await.unwrap();
    });
}