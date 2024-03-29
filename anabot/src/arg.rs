use clap::{clap_derive::ArgEnum, Parser, Subcommand};

#[derive(Debug, Copy, Clone, PartialEq, Eq, ArgEnum)]
pub enum AnagramType {
    Standard,
    Proper,
    Loose
}

#[derive(Debug, PartialEq, Eq, Subcommand)]
pub enum ActionType {
    /// Test if two words are anagrams
    Test { word_a: String, word_b: String },
    /// Find and print anagrams for a word, up to a given limit
    Find {
        word: String,
        /// The maximum number of anagrams to find
        ///
        /// The actual number of anagrams found may be under this limit, but never above.
        #[clap(short, long, default_value_t = 100)]
        limit: usize,
        /// The minimum length of each sub-word (only used with loose anagrams)
        ///
        /// For example, with this set to 3, no 1 or 2 letter words will appear in the results.
        #[clap(short, long, default_value_t = 1)]
        min_word_length: usize
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[clap(short = 'i', long)]
    /// Ignore case when testing or finding anagrams
    pub case_insensitive: bool,

    /// Type of anagrams to search for
    ///
    /// `standard`: every letter in word A appears in word B the same number of times.
    ///
    /// `proper`: word A and word B both appear in the word list and are
    /// standard anagrams of each other (requires a word list)
    ///
    /// `loose`: word A and word B are proper anagrams but may have a different number of
    /// spaces. For example, "racecar" and "arc care" are loose anagrams but not proper anagrams
    /// (requires a word list)
    #[clap(long, short = 't', arg_enum, value_parser, default_value_t = AnagramType::Proper)]
    pub anagram_type: AnagramType,

    /// Path to a word list file
    ///
    /// This file should be a text file with one word per line.
    /// If not provided, a default wordlist will be used as needed (if available)
    #[clap(long, short)]
    pub wordlist_path: Option<String>,

    /// Use simplified (machine readable)
    #[clap(long, short)]
    pub simple_output: bool,

    #[clap(subcommand)]
    pub action: ActionType
}
