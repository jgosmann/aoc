mod aoc_client;

use anyhow::Context;
use aoc_client::AocClient;
use inquire::Password;
use reqwest::Url;
use secrecy::Secret;
use tokio_stream::StreamExt;

fn get_session_id() -> anyhow::Result<Secret<String>> {
    let entry = keyring::Entry::new("adventofcode", "session_id")?;
    Ok(Secret::new(match entry.get_password() {
        Ok(password) => password,
        Err(keyring::Error::NoEntry) => {
            let session_id = Password::new("Your Advent of Code session id:")
                .without_confirmation()
                .prompt()
                .context("password input")?;
            entry.set_password(&session_id)?;
            session_id
        }
        err => err.context("credential store")?,
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AocClient::new(
        Url::parse("https://adventofcode.com/").context("client base URL")?,
        get_session_id()?,
    )?;
    let mut response_stream = client.get_input(2022, 11).await?;
    while let Some(piece) = response_stream.next().await {
        let piece = piece?;
        println!("{:?}", piece);
    }
    Ok(())
}
