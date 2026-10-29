use serde::{Deserialize, Serialize};
use serde_json;
use std::{error::Error, fmt};

use crate::QueueBody;

#[derive(Debug)]
struct HttpErr(u16);

#[derive(Serialize)]
struct RecentlyPlayedReq {
    limit: i32,
    before: i32,
}

#[derive(Deserialize, Debug)]
pub struct RecentlyPlayedResp {
    items: Vec<Item>
}

#[derive(Deserialize, Debug)]
pub struct Item {
    track: Track
}

#[derive(Deserialize, Debug)]
pub struct Track {
    #[serde(rename = "name")]
    track_title: String,
    #[serde(rename = "id")]
    track_id: String,
    artists: Vec<Artist>
}

#[derive(Deserialize, Debug)]
pub struct Artist {
    #[serde(rename = "name")]
    artist_name: String,
    #[serde(rename = "id")]
    artist_id: String,
    #[serde(rename = "href")]
    artist_link: String,
}

impl fmt::Display for HttpErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpErr Code: {}", self.0)
    }
}

impl Error for HttpErr {}

pub async fn add_to_queue(
    client: reqwest::Client,
    body: QueueBody<'_>,
) -> Result<(), Box<dyn Error>> {
    let url = "https://api.spotify.com/v1/me/player/queue/";

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).inspect_err(|e| {
            println!("Failed to create body: {}", e)
        })?)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::NO_CONTENT => Ok(()),
        other => Err(HttpErr(other.as_u16()).into()),
    }
}

pub async fn fetch_recently_played(
    client: reqwest::Client,
    before: i32,
) -> Result<Vec<RecentlyPlayedResp>, Box<dyn Error>> {
    let url =
        "https://api.spotify.com/v1/me/player/recently-played";

    let query_params = RecentlyPlayedReq { limit: 50, before };

    let response = client
        .get(url)
        .header("Content-Type", "application/json")
        .query(&query_params)
        .send()
        .await?;

    let body_text = response.text().await?;
    let recently_played: Vec<RecentlyPlayedResp> =
        serde_json::from_str(&body_text)?;

    Ok(recently_played)
}
