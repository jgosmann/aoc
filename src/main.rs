#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate solver_dispatch;

mod aoc_client;
mod cache;
mod session_id_store;
mod solvers;

use anyhow::Context;
use aoc_client::AocClient;
use cache::FileCache;
use chrono::{Datelike, FixedOffset, NaiveDate, Utc};
use clap::{Args, Parser, Subcommand};
use dirs::cache_dir;
use lazy_init::Lazy;
use reqwest::Url;
use session_id_store::SessionIdStore;
use solvers::Solver;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct MainArgs {
    /// Command to run. Default is "solve".
    #[command(subcommand)]
    command: Option<Command>,

    #[command(flatten)]
    solve_args: SolveArgs,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Set the session ID for interacting with the AoC API.
    SetSessionId,
    /// Solve puzzles.
    Solve(SolveArgs),
}

#[derive(Args, Clone, Debug)]
struct SolveArgs {
    /// Days of the advent calendar to solve. Defaults to the current
    /// day (EST/UTC-5, the timezone in which puzzles are published at
    /// midnight).
    #[arg(short = 'd', long = "days")]
    days: Option<Vec<u32>>,

    /// Year of the of the advent calendar to solve. Defaults to the current
    /// year.
    #[arg(short = 'y', long = "year")]
    year: Option<i32>,
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

fn get_current_aoc_date() -> NaiveDate {
    Utc::now()
        .with_timezone(&FixedOffset::west_opt(5 * 60 * 60).unwrap())
        .date_naive()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = MainArgs::parse();
    let session_id_store = SessionIdStore::new()?;

    let command = args.command.unwrap_or(Command::Solve(args.solve_args));
    match command {
        Command::SetSessionId => {
            session_id_store.prompt()?;
        }
        Command::Solve(solve_args) => {
            let date = (solve_args.year, solve_args.days);
            let (year, days) = if let (Some(year), Some(days)) = date {
                (year, days)
            } else {
                let current_date = get_current_aoc_date();
                (
                    date.0.unwrap_or(current_date.year()),
                    date.1.unwrap_or_else(|| vec![current_date.day()]),
                )
            };

            let client: Lazy<AocClient> = Lazy::new();
            let create_client = || {
                AocClient::new(
                    Url::parse("https://adventofcode.com/")
                        .context("client base URL")
                        .expect("cannot create HTTP client"),
                    session_id_store.session_id().expect("missing session ID"),
                )
                .expect("cannot create AoC client")
            };
            let cache_path = cache_dir().map_or_else(
                || {
                    eprintln!("Warning: couldn't locate cache directory, using ./aoc-cache");
                    "./aoc-cache".into()
                },
                |cache_base| cache_base.join("aoc"),
            );
            let input_cache = FileCache::new(cache_path, |key: InputKey| {
                let client = client.get_or_create(create_client);
                async move { client.get_input(key.year, key.day).await }
            })
            .await?;

            for &day in days.iter() {
                println!();
                println!("{}, day {}", year, day);

                let input = input_cache.get(&InputKey::from_yd(year, day)).await?;
                let solver: Box<dyn Solver> = solver_dispatch!(input, year, day)?;
                println!("{}", solver.solve_part_1()?);
                println!("{}", solver.solve_part_2()?);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::MainArgs;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        MainArgs::command().debug_assert()
    }
}
