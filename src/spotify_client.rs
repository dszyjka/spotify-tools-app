use serde::{Deserialize, Serialize};
use serde_json;
use std::{error::Error, fmt};

use crate::QueueParams;

#[derive(Serialize)]
struct RecentlyPlayedReq {
    limit: i32,
    before: i32,
}

#[derive(Deserialize, Debug)]
pub struct Items<T> {
    items: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub struct Track {
    #[serde(rename = "name")]
    track_title: String,
    #[serde(rename = "id")]
    track_id: String,
    artists: Vec<Artist>,
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

#[derive(Serialize, Debug)]
pub struct TopItemsReq {
    limit: i32,
    offset: i32,
    time_range: String,
}

#[derive(Debug)]
struct HttpErr(u16);

impl fmt::Display for HttpErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpErr Code: {}", self.0)
    }
}

impl Error for HttpErr {}

pub async fn add_to_queue(
    client: reqwest::Client,
    query_params: &QueueParams<'_>,
) -> Result<(), Box<dyn Error>> {
    let url = "https://api.spotify.com/v1/me/player/queue";

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .query(query_params)
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
) -> Result<Items<Track>, Box<dyn Error>> {
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
    let recently_played: Items<Track> =
        serde_json::from_str(&body_text)?;

    Ok(recently_played)
}

pub async fn fetch_user_top_artists(
    client: reqwest::Client,
) -> Result<Items<Artist>, Box<dyn Error>> {
    let url = "https://api.spotify.com/v1/me/top/artists";

    let query_params = TopItemsReq {
        limit: 50,
        time_range: String::from("medium_term"),
        offset: 0,
    };

    let response = client
        .get(url)
        .header("Content-Type", "application/json")
        .query(&query_params)
        .send()
        .await?;

    let body_text = response.text().await?;
    let top_items: Items<Artist> = serde_json::from_str(&body_text)?;

    Ok(top_items)
}
