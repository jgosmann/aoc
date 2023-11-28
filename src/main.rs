mod aoc_client;
mod session_id_store;

use anyhow::Context;
use aoc_client::AocClient;
use reqwest::Url;
use session_id_store::SessionIdStore;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AocClient::new(
        Url::parse("https://adventofcode.com/").context("client base URL")?,
        SessionIdStore::new()?.session_id()?,
    )?;
    let mut response_stream = client.get_input(2022, 11).await?;
    while let Some(piece) = response_stream.next().await {
        let piece = piece?;
        println!("{:?}", piece);
    }
    Ok(())
}
