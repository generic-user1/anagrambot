# anagrambot

A library and CLI tool to find anagrams of words. Written in Rust.


## Cargo Features

- `no-default-wordlist`: Do not include the default wordlist when building the project.

## CLI Tool Dependencies

- [clap](https://github.com/clap-rs/clap) for command line argument parsing

## Library Dependencies

None

## License Note

This project ships with an in-built list of words (henceforth referred to as 'the wordlist'). 
The wordlist is used as a default list of words when no external list is used, and is stored in the `words.txt` file.
The wordlist was extracted from the Ubuntu [wamerican](https://packages.ubuntu.com/jammy/wamerican) package and is redistibuted as a component of this project for maximum compatibility.
The copyright document associated with the wordlist is included verbatim in the `WORDLIST-LICENSE` file, and has been appended to the `LICENSE` file.

This project can function with a different default wordlist. To achieve this, replace the existing `words.txt` with a
new file named `words.txt` before building the project. Wordlist files should be UTF-8 encoded, contain one word per line, and ideally should have no duplicates.

This project can also function with no default wordlist. To achieve this, build the project with the `no-default-wordlist` feature.
- e.g. `cargo build --features no-default-wordlist` 
