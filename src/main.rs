mod aoc_client;
mod cache;
mod session_id_store;

use anyhow::Context;
use aoc_client::AocClient;
use cache::FileCache;
use clap::{Parser, Subcommand};
use dirs::cache_dir;
use reqwest::Url;
use session_id_store::SessionIdStore;
use tokio::io::AsyncReadExt;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct InputKey {
    year: i32,
    day: u32,
}

impl InputKey {
    fn from_yd(year: i32, day: u32) -> Self {
        Self { year, day }
    }
}

impl cache::Key for InputKey {
    type Serialization = String;

    fn serialize(&self) -> Self::Serialization {
        format!("{:04}-{:02}", self.year, self.day)
    }
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
        let cache_path = cache_dir().map_or_else(
            || {
                eprintln!("Warning: couldn't locate cache directory, using ./aoc-cache");
                "./aoc-cache".into()
            },
            |cache_base| cache_base.join("aoc"),
        );
        let input_cache = FileCache::new(cache_path, |key: InputKey| {
            let client = client.clone();
            async move { client.get_input(key.year, key.day).await }
        })
        .await?;
        let mut input = input_cache.get(&InputKey::from_yd(2022, 11)).await?;
        let mut buf = String::new();
        input.read_to_string(&mut buf).await?;
        println!("{:?}", buf);
    }

    Ok(())
}
