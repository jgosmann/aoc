mod aoc_client;
mod session_id_store;

use anyhow::Context;
use aoc_client::AocClient;
use clap::{Parser, Subcommand};
use reqwest::Url;
use session_id_store::SessionIdStore;
use tokio_stream::StreamExt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Set the session ID for interacting with the AoC API.
    SetSessionId,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let session_id_store = SessionIdStore::new()?;

    if let Some(command) = args.command {
        match command {
            Command::SetSessionId => {
                session_id_store.prompt()?;
            }
        }
    } else {
        let client = AocClient::new(
            Url::parse("https://adventofcode.com/").context("client base URL")?,
            session_id_store.session_id()?,
        )?;
        let mut response_stream = client.get_input(2022, 11).await?;
        while let Some(piece) = response_stream.next().await {
            let piece = piece?;
            println!("{:?}", piece);
        }
    }

    Ok(())
}
