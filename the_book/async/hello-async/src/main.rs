#![allow(unused)]

use std::future::Future;
use trpl::{Either, Html};

fn page_title_(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}

// When Rust sees a block marked with the async keyword, it compiles it into a unique, anonymous data type that implements the Future trait.
async fn page_title(url: &str) -> Option<String> {
    // futures in Rust are lazy: they don’t do anything until you ask them to with the await keyword.
    //let response = trpl::get(url).await;
    //let response_text = response.text().await;
    let response_text = trpl::get(url).await.text().await;

    // When Rust sees a function marked with async, it compiles it into a non-async function whose body is an async block. 
    // An async function’s return type is the type of the anonymous data type the compiler creates for that async block.

    Html::parse(&response_text)
        .select_first("title")
        // Because there may not be any matching element, select_first returns an Option<ElementRef>. 
        // Finally, we use the Option::map method, which lets us work with the item in the Option if it’s present, and do nothing if it isn’t. 
        // (We could also use a match expression here, but map is more idiomatic.)
        .map(|title| title.inner_html())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    });

    // Calling page_title for two URLs to see which returns first
    trpl::block_on(async {
        let title_fut_1 = page_title__(&args[1]);
        let title_fut_2 = page_title__(&args[2]);

        let (url, maybe_title) =
            match trpl::select(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    })
}

async fn page_title__(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}
