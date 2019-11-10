use clap::{App, Arg};
use futures::future::join_all;
use reqwest::{Client, Error, Response};
use reqwest::blocking::{Client as BClient, Response as BResponse};

static URL: &str = "http://worldtimeapi.org/api/timezone/America/New_York";

async fn make_async_request(client: &Client) -> Result<Response, Error> {
    client.get(URL).send().await
}

#[tokio::main]
async fn async_main() {
    let client = Client::new();

    // Make 5 requests
    let futures: Vec<_> = (0..5).map(|_| make_async_request(&client)).collect();
    // Join on all of them
    let resolved_futures: Vec<Result<Response, Error>> = join_all(futures).await;

    // Get the body from each requests
    let text_futures = resolved_futures.into_iter().map(|resp| resp.unwrap().text());
    // Join on all of the futures
    let text = join_all(text_futures).await;

    text.into_iter().for_each(|t| println!("{}", t.unwrap()));
}

fn make_blocking_request(client: &BClient) -> Result<BResponse, Error> {
    client.get(URL).send()
}

fn blocking_main() {
    let client = BClient::new();

    // Make 5 requests
    let responses: Vec<_> = (0..5).map(|_| make_blocking_request(&client)).collect();
    // Get the body from each requests
    let response_bodies = responses.into_iter().map(|resp| resp.unwrap().text().unwrap());
    
    response_bodies.into_iter().for_each(|b| println!("{}", b));
}

fn main() {
    let args = App::new("async-reqwest")
        .version("0.1")
        .author("Parker Timmerman")
        .arg(
            Arg::with_name("async")
            .short("a")
            .long("async")
        ).get_matches();

    if args.is_present("async") {
        println!("Running async!");
        async_main();
    } else {
        println!("Running synchronously!");
        blocking_main();
    }
}