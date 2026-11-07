mod cli;
mod client;
mod display;
mod model;

use std::env;
use std::error::Error;
use std::process;

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    let word = cli::parse_word(&args)?;
    let entries = client::lookup(&word)?;

    let entry = entries
        .first()
        .ok_or_else(|| format!("no entries returned for '{}'.", word))?;

    display::print_entry(entry);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}