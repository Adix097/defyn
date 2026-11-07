use std::error::Error;
use std::thread;
use std::time::Duration;

use crate::model::Entry;

const MAX_ATTEMPTS: u32 = 3;

pub fn lookup(word: &str) -> Result<Vec<Entry>, Box<dyn Error>> {
    let body = fetch(word)?;
    let entries: Vec<Entry> = serde_json::from_str(&body)?;
    Ok(entries)
}

fn fetch(word: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

    for attempt in 1..=MAX_ATTEMPTS {
        let reason = match reqwest::blocking::get(&url) {
            Ok(response) => {
                let status = response.status();

                if status == reqwest::StatusCode::NOT_FOUND {
                    return Err(format!("No definition found for '{}'", word).into());
                }

                if status.is_success() {
                    return Ok(response.text()?);
                }

                if status.is_server_error() {
                    format!("server error (HTTP {})", status.as_u16())
                } else {
                    return Err(format!("Unexpected response (HTTP {})", status.as_u16()).into());
                }
            }
            Err(e) => format!("network problem ({})", e),
        };

        if attempt < MAX_ATTEMPTS {
            eprintln!(
                "Attempt {}/{} failed: {}. Retrying in 1 second...",
                attempt, MAX_ATTEMPTS, reason
            );
            thread::sleep(Duration::from_secs(1));
        } else {
            return Err(format!(
                "Giving up after {} attempts. Last error: {}.",
                MAX_ATTEMPTS, reason
            )
            .into());
        }
    }

    unreachable!("the loop always returns before finishing")
}