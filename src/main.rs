#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate solver_dispatch;

mod aoc_client;
mod cache;
mod datastructures;
mod session_id_store;
mod solvers;

use ansi_term::Color::Yellow;
use ansi_term::Style;
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
use std::path::{Path, PathBuf};
use tokio::try_join;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
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
    /// Create module for a day from template.
    Create(SolveArgs),
}

#[derive(Args, Clone, Debug)]
struct SolveArgs {
    /// Days of the advent calendar to solve. Defaults to the current
    /// day (EST/UTC-5, the timezone in which puzzles are published at
    /// midnight).
    #[arg(short = 'd', long = "days")]
    days: Option<Vec<u32>>,

    /// Year of the advent calendar to solve. Defaults to the current
    /// year.
    #[arg(short = 'y', long = "year")]
    year: Option<i32>,
}

struct RequestedDays {
    pub year: i32,
    pub days: Vec<u32>,
}

impl From<SolveArgs> for RequestedDays {
    fn from(value: SolveArgs) -> Self {
        let date = (value.year, value.days);
        if let (Some(year), Some(days)) = date {
            Self { year, days }
        } else {
            let current_date = get_current_aoc_date();
            Self {
                year: date.0.unwrap_or(current_date.year()),
                days: date.1.unwrap_or_else(|| vec![current_date.day()]),
            }
        }
    }
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

async fn write_if_non_existent<P: AsRef<Path>>(path: P, content: &str) -> anyhow::Result<()> {
    if tokio::fs::try_exists(&path).await? {
        eprintln!(
            "{} {}{}{}",
            Yellow.bold().paint("Warning:"),
            Yellow.paint("file '"),
            Yellow.paint(path.as_ref().display().to_string()),
            Yellow.paint("' already exists, skipping")
        );
    } else {
        tokio::fs::write(&path, content)
            .await
            .with_context(|| format!("writing file: {}", path.as_ref().display()))?;
    }
    Ok(())
}

async fn add_module_declaration(path: impl AsRef<Path>, days_to_add: &[u32]) -> anyhow::Result<()> {
    const MODULE_DECLARATION_MARKER: &str = "// <<INSERT MARKER>>";
    let updated_module = String::from_utf8(tokio::fs::read(&path).await?)?
        .lines()
        .map(|line| {
            if line.trim() == MODULE_DECLARATION_MARKER {
                days_to_add
                    .iter()
                    .map(|day| format!("    pub mod day{};", day))
                    .chain(std::iter::once(format!(
                        "    {}",
                        MODULE_DECLARATION_MARKER
                    )))
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                line.into()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    tokio::fs::write(&path, updated_module).await?;
    Ok(())
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
            let RequestedDays { year, days } = solve_args.into();

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
                println!(
                    "📆 {}",
                    Style::new().underline().paint(format!(
                        "{}, day {}",
                        year,
                        Style::new().bold().paint(day.to_string())
                    ))
                );

                let input = input_cache.get(&InputKey::from_yd(year, day)).await?;
                let solver: Box<dyn Solver> = solver_dispatch!(input, year, day)?;
                println!("⭐ {}", solver.solve_part_1()?);
                println!("⭐ {}", solver.solve_part_2()?);
            }
        }
        Command::Create(solve_args) => {
            static TEMPLATE: &str = include_str!("day.rs.template");

            let RequestedDays { year, days } = solve_args.into();
            let base_path = PathBuf::from(format!("src/solvers/year{}", year));
            tokio::fs::create_dir_all(&base_path)
                .await
                .with_context(|| format!("creating directories: {}", base_path.display()))?;

            for day in &days {
                let day_path = base_path.join(format!("day{}.rs", day));
                let example_path = base_path.join(format!("day{}-1.example", day));
                let source_content = TEMPLATE.replace("{{day}}", &day.to_string());
                try_join!(
                    write_if_non_existent(day_path, &source_content),
                    write_if_non_existent(example_path, ""),
                )?;
            }
            add_module_declaration("src/solvers/mod.rs", &days).await?;
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
