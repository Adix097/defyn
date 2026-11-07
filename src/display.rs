use crate::model::Entry;

pub fn print_entry(entry: &Entry) {
    let Some(meaning) = entry.meanings.first() else {
        eprintln!("Error: '{}' has no meanings", entry.word);
        return;
    };

    let Some(definition) = meaning.definitions.first() else {
        eprintln!("Error: '{}' has no definitions", entry.word);
        return;
    };

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