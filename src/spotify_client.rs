use std::fmt;
use serde_json;

use crate::QueueBody;

#[derive(Debug)]
struct HttpErr(u16);

impl fmt::Display for HttpErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpErr Code: {}", self.0)
    }
}

impl std::error::Error for HttpErr {}

pub async fn add_to_queue(
    client: reqwest::Client,
    payload: QueueBody<'_>,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.spotify.com/v1/me/player/queue/";

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&payload).inspect_err(
            |e| println!("Failed to create body: {}", e),
        )?)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::NO_CONTENT => Ok(()),
        other => Err(HttpErr(other.as_u16()).into())
    }
}
