mod sudoku;
mod server;

use std::path::Path;
use anyhow;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author = "Sam Uherek <samuherekbiz@gmail.com>")]
#[command(about = "Sudoku stuff", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Gen { value: String }
}

fn main() -> anyhow::Result<()>{
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Gen{value}) => sudoku::generate(Path::new(value), 100),
        None => server::server::run(),
    }

    return Ok(());
}
