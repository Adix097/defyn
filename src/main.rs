use std::env;
use std::error::Error;
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

fn fetch(word: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

    let response = reqwest::blocking::get(&url)?;
    let status = response.status();

    if status == reqwest::StatusCode::NOT_FOUND {
        return Err(format!("No definition found for '{}'", word).into());
    }

    if status.is_server_error() {
        return Err(format!(
            "The dictionary service is having problems (HTTP {}). Try again in a moment",
            status.as_u16()
        )
        .into());
    }

    let body = response.error_for_status()?.text()?;
    Ok(body)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let word = match parse_word(&args) {
        Ok(word) => word,
        Err(message) => {
            eprintln!("Error: {}", message);
            process::exit(1);
        }
    };

    match fetch(&word) {
        Ok(body) => println!("{}", body),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}