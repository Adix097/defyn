use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub word: String,
    pub meanings: Vec<Meaning>,
}

#[derive(Debug, Deserialize)]
pub struct Meaning {
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Deserialize)]
pub struct Definition {
    pub definition: String,
    pub example: Option<String>,
}