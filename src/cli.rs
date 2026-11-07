pub fn parse_word(args: &[String]) -> Result<String, String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_one_word_and_lowercases_it() {
        assert_eq!(parse_word(&["Hello".to_string()]), Ok("hello".to_string()));
    }

    #[test]
    fn rejects_no_words() {
        assert!(parse_word(&[]).is_err());
    }

    #[test]
    fn rejects_two_words() {
        assert!(parse_word(&["a".to_string(), "b".to_string()]).is_err());
    }

    #[test]
    fn rejects_digits() {
        assert!(parse_word(&["abc123".to_string()]).is_err());
    }
}