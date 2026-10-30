mod spotify_client;

use reqwest;
use serde::Serialize;
use tokio;

#[derive(Serialize)]
struct QueueParams<'a> {
    uri: &'a str,
    device_id: &'a str,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
}
