use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("Error: defyn accepts exactly one word.");
        eprintln!("Usage: defyn <word>");
        process::exit(1);
    }

    let word = &args[0];

    if !word.chars().all(|c| c.is_alphabetic() || c == '-') {
        eprintln!("Error: '{}' is not a valid word (letters and hyphens only).", word);
        process::exit(1);
    }

    println!("Looking up: {}", word.to_lowercase());
}