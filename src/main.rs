mod spotify_client;

use serde::Serialize;
use tokio;
use reqwest;

#[derive(Serialize)]
struct QueueBody<'a> {
    uri: &'a str,
    device_id: &'a str,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
}
