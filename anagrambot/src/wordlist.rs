//! The `Wordlist` trait and some implementations

use std::{io::{self, BufReader, BufRead}, fs, path::Path};

/// A list of words
/// 
/// A `Wordlist` is a list of words (each word being a `&str`).
pub trait Wordlist<'a>
{
    /// The type of iterator that the `iter` method returns
    /// 
    /// Must be an [Iterator] yielding `&str`
    type IterType: Iterator<Item = &'a str>;

    /// Returns an an iterator that returns all words
    /// 
    /// Unlike the IntoIterator trait, does not consume the `Wordlist`
    fn iter(&'a self) -> Self::IterType;

    /// Given a word, returns true if the word is contained within this `Wordlist`
    fn includes_word(&self, word: &str) -> bool;
}

/// A [Wordlist] implementor that borrows its words
/// 
/// Useful for creating a `Wordlist` from data that already exists
/// (such as a `&'static str` or pre-existing `String`)
pub struct BorrowedWordList<'a> {
    word_vec: Vec<&'a str>
}

impl<'a> BorrowedWordList<'a> {
    /// Construct a new `BorrowedWordList` from an iterator of `&str`
    pub fn new(word_iter: impl IntoIterator<Item = &'a str>) -> Self
    {
        Self { word_vec: word_iter.into_iter().collect() }
    }
}

impl<'a> FromIterator<&'a str> for BorrowedWordList<'a>{
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Self::new(iter)
    }
}

impl<'a> Wordlist<'a> for BorrowedWordList<'a>{
    type IterType = std::iter::Copied<std::slice::Iter<'a, &'a str>>;

    fn includes_word(&self, word: &str) -> bool {
        self.word_vec.contains(&word)
    }

    fn iter(&'a self) -> Self::IterType {
        self.word_vec.iter().copied()
    }
}

#[cfg(test)]
mod borrowedwordlist_tests{
    use super::{BorrowedWordList, Wordlist};

    #[test]
    fn test_includes_word()
    {
        let list = BorrowedWordList::new(["a","b","c"]);
        assert!(list.includes_word("a"));
        assert!(!list.includes_word("not in list"))
    }

    #[test]
    fn test_to_from_iter()
    {
        const WORD_ARRAY: [&str; 3] = ["a", "b", "c"];
        use core::iter;
        let list_from_iter: BorrowedWordList = WORD_ARRAY.into_iter().collect();
        let list_from_new = BorrowedWordList::new(WORD_ARRAY);
    
        let first_iter = list_from_iter.iter();
        let second_iter = list_from_new.iter();

        for (first, second) in iter::zip(first_iter, second_iter){
            assert_eq!(first, second);
        }
    }
}

/// A [Wordlist] implementor that owns its words
/// 
/// Useful for creating a `Wordlist` from new data (such as from a file)
pub struct OwnedWordList {
    word_vec: Vec<String>
}

impl OwnedWordList{
    /// Construct a new `OwnedWordList` from an iterator of [String](std::string::String)
    pub fn new(word_iter: impl IntoIterator<Item = String>) -> Self
    {
        Self{word_vec: word_iter.into_iter().collect()}
    }

    /// Construct a new `OwnedWordList` from the contents of a text file
    ///
    /// `word_file` must be a [Path] to a text file containing words.
    ///  
    /// Each line of the text file is considered a single word.
    pub fn from_file(word_file: &Path) -> io::Result<Self>
    {
        let word_file = fs::File::open(word_file)?;

        let mut word_vec: Vec<String> = Vec::new();

        let lines_iter = BufReader::new(word_file).lines();

        for line in lines_iter {
            word_vec.push(line?);
        }

        Ok(Self::new(word_vec))
    }
}

impl FromIterator<String> for OwnedWordList{
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        OwnedWordList::new(iter)
    }
}

impl<'a> Wordlist<'a> for OwnedWordList{
    // this long type has to be written out because impl trait syntax
    // cannot be used for associated types
    type IterType = std::iter::Map<std::slice::Iter<'a, String>, fn(&String) -> &str>;

    fn includes_word(&self, word: &str) -> bool {
        let word = String::from(word);
        self.word_vec.contains(&word)
    }

    fn iter(&'a self) -> Self::IterType {
        self.word_vec.iter().map(|p|{p.as_str()})
    }
}

#[cfg(test)]
mod ownedwordlist_tests{
    use super::{OwnedWordList, Wordlist};
    use crate::default_wordlist::default_wordlist;
    use std::path::Path;

    #[test]
    fn test_includes_word()
    {
        let word_strings: [String; 3] = [String::from("a"), String::from("b"), String::from("c")];

        let list = OwnedWordList::new(word_strings);
        assert!(list.includes_word("a"));
        assert!(!list.includes_word("not in list"))
    }

    #[test]
    fn test_to_from_iter()
    {

        // two are needed because the first OwnedWordList will take ownership of the first
        let word_strings_a: [String; 3] = [String::from("a"), String::from("b"), String::from("c")];
        let word_strings_b: [String; 3] = [String::from("a"), String::from("b"), String::from("c")];


        use core::iter;
        let list_from_iter: OwnedWordList = word_strings_a.into_iter().collect();
        let list_from_new = OwnedWordList::new(word_strings_b);
    
        let first_iter = list_from_iter.iter();
        let second_iter = list_from_new.iter();

        for (first, second) in iter::zip(first_iter, second_iter){
            assert_eq!(first, second);
        }
    }

    
    #[test]
    fn test_default_vs_file(){

        let default_wordlist = match default_wordlist(){
            Some(wordlist) => wordlist,
            None => {return;} //end test if default wordlist isn't present
        };

        let wordlist_from_file = 
            OwnedWordList::from_file(Path::new("words.txt")).unwrap();

        for (defword, ownedword) in default_wordlist.iter().zip(wordlist_from_file.iter()){
            assert_eq!(defword, ownedword);
        }
    }
}