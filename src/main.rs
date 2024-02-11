use sudokurs::{server, sudoku};
use std::path::Path;
use anyhow;
use clap::{Parser, Subcommand};
use actix_web;
use std::net::TcpListener;

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

#[actix_web::main]
async fn serve() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address)?;
    server::run(listener)?.await?;
    Ok(())
}

fn main() -> anyhow::Result<()>{
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Gen{value}) => sudoku::generate(Path::new(value), 100),
        None => serve()?,
    }

    return Ok(());
}
