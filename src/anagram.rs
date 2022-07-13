//! Utilities for finding and verifying anagrams
//! 
//! For the purposes of this module, an anagram is defined as such:
//! two strings are "anagrams" of each other if they both contain the same characters,
//! but in a different order.
//! 
//! For example:
//! - "race" and "care" are anagrams, because they contain the same characters and are not identical
//! - "race" and "race" are not anagrams, because words cannot be anagrams of themselves
//! - "asdf" and "safd" are anagrams, because they contain the same characters and are not identical
//! 
//! This last point introduces the need for a more strict form of anagram, which this
//! module calls a "proper anagram": proper anagrams must be actual words. That is,
//! anagrams can only be considered proper anagrams if they appear in a list of defined words.
//! Each proper anagram related function requires a [Wordlist] to check if a given string is a word.
//! 
//! For example:
//! - "race" and "care" are proper anagrams because they are anagrams and both words
//! - "race" and "reca" are not proper anagrams because "reca" is not a word

use crate::wordlist::{Wordlist};

use std::collections::BTreeMap;
type Charmap = BTreeMap<char, u32>;

pub mod loose_anagram;
pub use loose_anagram::find_loose_anagrams;

/// Returns a [HashMap] with the number of times each character appears in `word`
/// 
/// The resulting [HashMap] has a key for each character in `word`, with the value
/// being the number of times that character appears in `word`
/// 
/// If `ignore_spaces` is true, space characters `' '` will be entirely skipped over
fn get_charcount_map(word: &str, ignore_spaces: bool) -> Charmap
{
    let mut lettercount_map = Charmap::new();

    for letter in word.chars(){
        if ignore_spaces && letter == ' '{
            continue;
        } else {
            match lettercount_map.get_mut(&letter) {
                None => {lettercount_map.insert(letter, 1);},
                Some(count) => {*count+=1}
            }
        }
    }

    lettercount_map
}

/// Returns true if `word_a` and `word_b` are anagrams
/// 
/// This tests for standard anagrams, not proper anagrams. This means
/// that non-word character sequences that nonetheless contain the same
/// characters in a different order will result in `true`
/// 
/// Note that two identical words are not considered anagrams
/// 
///# Examples
/// ```
/// use anagrambot::anagram::are_anagrams;
/// 
/// //proper anagram
/// assert!(are_anagrams("race", "care"));
/// //non-proper anagram
/// assert!(are_anagrams("aabc", "caab"));
/// 
/// //non-anagram due to different letters
/// assert!(!are_anagrams("race", "cow"));
/// //non-anagram due to being identical
/// assert!(!are_anagrams("race", "race"));
/// ```
pub fn are_anagrams(word_a: &str, word_b: &str) -> bool
{
    //words can't be anagrams if their lengths are different
    if word_a.len() != word_b.len(){
        return false;
    //two identical words are not anagrams
    } else if word_a == word_b{
        return false;
    }

    //words are anagrams if both previous conditions weren't true
    //and the counts of each of their letters are identical
    get_charcount_map(word_a, false) == get_charcount_map(word_b, false)
}

/// Similar to [are_anagrams] but checks that both words are real words
/// 
/// This function will return false if either `word_a`, `word_b`, or both
/// are not found in the specified `wordlist`.
/// 
/// `wordlist` must implement the [Wordlist] trait (for example, the 
/// [default wordlist](crate::default_wordlist::default_wordlist) if present)
/// ///# Examples
/// ```
/// use anagrambot::anagram::are_proper_anagrams;
/// use anagrambot::wordlist::BorrowedWordList;
/// 
/// // you can use anagrambot::default_wordlist::default_wordlist()
/// // to get the default Wordlist instead of generating your own,
/// // as long as the `no-default-wordlist` feature is not enabled
/// const TEST_WORD_SET: [&str; 3] = ["race", "care", "cow"];
/// let wordlist: BorrowedWordList = TEST_WORD_SET.into_iter().collect();
/// 
/// //proper anagram
/// assert!(are_proper_anagrams("race", "care", &wordlist));
/// 
/// //non-proper anagram
/// assert!(!are_proper_anagrams("aabc", "caab", &wordlist));
/// //non-anagram due to different letters
/// assert!(!are_proper_anagrams("race", "cow", &wordlist));
/// //non-anagram due to being identical
/// assert!(!are_proper_anagrams("race", "race", &wordlist));
/// ```
pub fn are_proper_anagrams<'a>(word_a: &str, word_b: &str, wordlist: &impl Wordlist<'a>) -> bool
{
    //return false if either word is not found in wordlist
    if !wordlist.includes_word(word_a){
        return false;
    } else if !wordlist.includes_word(word_b){
        return false;
    }

    //now that we ensured both words are real words, use the standard are_anagrams function
    are_anagrams(word_a, word_b)
}


// TODO: use Heap's Algorithm (https://en.wikipedia.org/wiki/Heap%27s_algorithm)
// to create a method to generate all possible anagrams of a string
// named find_anagrams


/// An iterator over all the proper anagrams of a word
/// 
/// The return value of [find_proper_anagrams]
pub struct ProperAnagramsIter<'a, T>
where T: Iterator<Item = &'a str>
{
    word: &'a str,
    wordlist_iter: T
}

impl<'a, T> Iterator for ProperAnagramsIter<'a, T>
where T: Iterator<Item = &'a str>
{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next_word) = self.wordlist_iter.next() {
            if are_anagrams(self.word, next_word){
                return Some(next_word);
            }
        }
        None
    }
}


/// Returns a [ProperAnagramsIter] over all proper anagrams of `word`
/// 
/// Note that this method does not check if `word` is present in `wordlist`;
/// this is the responsibility of the caller (if desired)
pub fn find_proper_anagrams<'a, T>(word: &'a str, wordlist: &'a T)
 -> ProperAnagramsIter<'a, impl Iterator<Item = &'a str>>
where T: Wordlist<'a>
{
    ProperAnagramsIter { word, wordlist_iter: wordlist.iter()}
}