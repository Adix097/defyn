use std::env;
use std::error::Error;
use std::process;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Entry {
    word: String,
    meanings: Vec<Meaning>,
}

#[derive(Debug, Deserialize)]
struct Meaning {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    definitions: Vec<Definition>,
}

#[derive(Debug, Deserialize)]
struct Definition {
    definition: String,
    example: Option<String>,
}

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

fn lookup(word: &str) -> Result<Vec<Entry>, Box<dyn Error>> {
    let body = fetch(word)?;
    let entries: Vec<Entry> = serde_json::from_str(&body)?;
    Ok(entries)
}

fn print_entry(entry: &Entry) {
    let Some(meaning) = entry.meanings.first() else {
        eprintln!("Error: '{}' has no meanings", entry.word);
        return;
    };

    let Some(definition) = meaning.definitions.first() else {
        eprintln!("Error: '{}' has no definitions", entry.word);
        return;
    };

    // Collect up to two examples from ANY meaning of this entry
    let examples: Vec<&str> = entry
        .meanings
        .iter()
        .flat_map(|m| m.definitions.iter())
        .filter_map(|d| d.example.as_deref())
        .take(2)
        .collect();

    println!("{} ({})", entry.word, meaning.part_of_speech);
    println!("  Meaning: {}", definition.definition);

    for (i, example) in examples.iter().enumerate() {
        println!("  {}. {}", i + 1, example);
    }

    if examples.len() < 2 {
        eprintln!("  (only {} example(s) available)", examples.len());
    }
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

    match lookup(&word) {
        Ok(entries) => match entries.first() {
            Some(entry) => print_entry(entry),
            None => {
                eprintln!("Error: no entries returned for '{}'.", word);
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}