use std::env;
use std::process;

fn parse_word(args: &[String]) -> Result<String, String> {
    if args.len() != 1 {
        return Err("defyn accepts exactly one word.\nUsage: defyn <word>".to_string());
    }

    let word = &args[0];

    if word.is_empty() {
        return Err("No word was passed!".to_string());
    }

    if !word.chars().all(|c| c.is_alphabetic() || c == '-') {
        return Err(format!("'{}' is not a valid word", word));
    }

    Ok(word.to_lowercase())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match parse_word(&args) {
        Ok(word) => println!("Looking up: {}", word),
        Err(message) => {
            eprintln!("Error: {}", message);
            process::exit(1);
        }
    }
}