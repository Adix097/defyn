# defyn

A tiny command-line dictionary written in Rust. Give it one word, and it prints the meaning and up to two example sentences.

```
$ defyn serendipity
serendipity (noun)
  Meaning: the occurrence of events by chance in a happy way.
  1. A fortunate stroke of serendipity brought them together.
  2. Finding that book was pure serendipity.
```

## Install

You need [Rust](https://rustup.rs) installed.

```bash
git clone <repo-url>
cd defyn
cargo install --path .
```

## Usage

```bash
defyn <word>
```

- Exactly one word is accepted.
- Anything else prints an error and exits with code 1.
- If the API is down, it retries 3 times before giving up.

## How it works

`defyn` fetches data from the free [dictionaryapi.dev](https://dictionaryapi.dev) API, parses the JSON with `serde`, and prints the first meaning with its examples. Some words have fewer than two examples in this API.

## Project structure

```
src/
├── main.rs      # entry point
├── cli.rs       # input validation (with unit tests)
├── model.rs     # data structs
├── client.rs    # HTTP request, retries, JSON parsing
└── display.rs   # output formatting
```

## Run the tests

```bash
cargo test
```