#![allow(unused)]

use std::time::Duration;

// Sending Data Between Two Tasks Using Message Passing

fn main() {
    trpl::block_on(async {
        
        // Creating an async channel and assigning the two halves to tx and rx
        let (tx, mut rx) = trpl::channel();
        let val = String::from("hi");
        tx.send(val).unwrap();
        let received = rx.recv().await.unwrap();
        println!("received '{received}'");

        // Sending and receiving multiple messages over the async channel and sleeping with an await between each message
        let (tx, mut rx) = trpl::channel();
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    });
}