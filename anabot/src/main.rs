use anagrambot::{wordlist::{OwnedWordList, Wordlist}, anagram, default_wordlist};
use clap::Parser;

use std::path::Path;

mod arg;
use arg::{CliArgs, AnagramType, ActionType};

const REASON_DUPLICATES: &str = "a word cannot be an anagram of itself";
const REASON_FIRST_NOT_WORD: &str = "first provided word is not a valid word";
const REASON_SECOND_NOT_WORD: &str = "second provided word is not a valid word";
const REASON_CHARS_DIFFERENT: &str = "words do not contain the same characters in the same amounts";


fn main() -> Result<(), String> {
    let args = CliArgs::parse();
    handle_args(args)
}

/// main arg handling function
/// 
/// includes full handling for standard anagrams and delegates other types of anagrams to do_action
fn handle_args(args: CliArgs) -> Result<(), String>
{
    // handle Standard first, as it requires no wordlist and thus no wordlist handling
    if &args.anagram_type == &AnagramType::Standard {
        match &args.action {
            ActionType::Find{..} => {
                return Err("No `find` method for standard anagrams (yet)!".to_string())
            },
            ActionType::Test {word_a, word_b } => {
                if anagram::are_anagrams(word_a, word_b, !args.case_insensitive){
                    if args.simple_output {
                        println!("true")
                    } else {
                        println!("\"{}\" is standard anagram of \"{}\"", word_a, word_b);
                    }
                } else {
                    if args.simple_output{
                        println!("false");
                    } else {
                        println!("\"{}\" is not standard anagram of \"{}\"", word_a, word_b);
                        if word_a == word_b {
                            println!("Reason: {}", REASON_DUPLICATES);
                        } else {
                            println!("Reason: {}", REASON_CHARS_DIFFERENT);
                        }
                    }
                }
            }
        }
    } else {
        // handle getting a wordlist
        // if this fails, return Err(message)
        // if this succeeds, call do_action to perform whatever action
        if let Some(wordlist_path) = &args.wordlist_path {
            let wordlist = match OwnedWordList::from_file(
                &Path::new(wordlist_path))
            {
                Ok(wordlist) => wordlist,
                Err(_) => {
                    return Err(format!("Failed to read word list file {}", wordlist_path));
                }
            };

            do_action(&args, &wordlist);
        } else {
            let wordlist = match default_wordlist::default_wordlist() {
                Some(wordlist) => wordlist,
                None => {
                    let errmsg = String::from("No word list was provided, ") +
                    "but no default wordlist could be found. Please provide a word list " +
                    "file (text file, one word per line) using the `-w` option";
                    return Err(errmsg);
                }
            };
            do_action(&args, &wordlist)
        }
    }

    Ok(())
}

/// used to handle actions involving a wordlist in a common manner independant of wordlist type
/// 
/// called after a wordlist is determined to be needed and has been successfully resolved. 
/// 
///# Panics
/// 
/// this function panics if args.anagram_type is `Standard`, as this is meant to be handled
/// before this function is called (due to the lack of requirement of a wordlist)
fn do_action<'a>(args: &CliArgs, wordlist: &'a impl Wordlist<'a>)
{
    const PANIC_MSG: &str = "Logic Error! Used do_action for standard anagram";
    
    let case_sensitive = !args.case_insensitive;
    match &args.action {
        ActionType::Test {word_a, word_b } => {
            let (are_anagrams, anagram_name) = match &args.anagram_type {
                AnagramType::Standard => panic!("{}", PANIC_MSG),
                AnagramType::Proper => {
                    let are_anagrams = 
                    anagram::are_proper_anagrams(&word_a, &word_b, wordlist, case_sensitive);
                    (are_anagrams, "proper")
                },
                AnagramType::Loose => {
                    let are_anagrams = 
                    anagram::are_loose_anagrams_strict(&word_a, &word_b, wordlist, case_sensitive);
                    (are_anagrams, "loose")
                }
            };

            if args.simple_output {
                if are_anagrams {
                    println!("true");
                } else {
                    println!("false");
                }
            } else {
                if are_anagrams{
                    println!("\"{}\" is {} anagram of \"{}\"",word_a, anagram_name, word_b);
                } else {
                    println!("\"{}\" is not {} anagram of \"{}\"",word_a, anagram_name, word_b);
                    if word_a == word_b {
                        println!("Reason: {}", REASON_DUPLICATES);
                    } else {
                        let word_a_real = wordlist.includes_word(&word_a);
                        let word_b_real = wordlist.includes_word(&word_b);
                        if !word_a_real {
                            println!("Reason: {}", REASON_FIRST_NOT_WORD);
                        }
                        if !word_b_real {
                            println!("Reason: {}", REASON_SECOND_NOT_WORD);
                        }
                        if word_a_real && word_b_real {
                            println!("Reason: {}", REASON_CHARS_DIFFERENT);
                        }
                    } 
                }
            }
        },
        ActionType::Find { word, limit } => {
            fn print_fn<'c>(args: &CliArgs, mut iter: impl Iterator<Item = impl std::fmt::Display>, limit: usize) {
                let mut index: usize = 0;
                while let Some(word) = iter.next(){
                    if index >= limit {
                        break;
                    }
                    println!("{}", word);

                    index += 1;
                }
                if !args.simple_output{
                    let anagram_type = match args.anagram_type{
                        AnagramType::Standard => panic!("{}", PANIC_MSG),
                        AnagramType::Proper => "proper",
                        AnagramType::Loose => "loose"
                    };
                    println!("found {} {} anagrams", index, anagram_type);
                }
            }
            match &args.anagram_type {
                AnagramType::Standard => panic!("{}", PANIC_MSG),
                AnagramType::Proper => {
                    print_fn(&args, anagram::find_proper_anagrams(&word, wordlist, case_sensitive), *limit);
                },
                AnagramType::Loose => {
                    print_fn(&args, anagram::find_loose_anagrams(&word, wordlist, case_sensitive), *limit);
                }
            }
        }
    }
}
