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

use crate::wordlist::Wordlist;

use std::collections::BTreeMap;

/// Type representing the set of characters a word contains
/// 
/// Each key is a character that appears in the word. Each value
/// is the number of times that character appears
type Charmap = BTreeMap<char, u32>;

pub mod loose_anagram;
pub use loose_anagram::{are_loose_anagrams, are_loose_anagrams_strict, find_loose_anagrams};

/// Returns a [Charmap] with the number of times each character appears in `word`
/// 
/// The resulting [Charmap] has a key for each character in `word`, with the value
/// being the number of times that character appears in `word`
/// 
/// If `ignore_spaces` is true, space characters `' '` will be entirely skipped over
/// 
/// If `case_sensitive` is true, characters of different case will be treated as different.
/// If `case_sensitive` is false, characters of different case will be treated as the same.
fn get_charcount_map(word: &str, ignore_spaces: bool, case_sensitive: bool) -> Charmap
{
    let mut lettercount_map = Charmap::new();

    let mut insert_closure = |letter|{
        match lettercount_map.get_mut(&letter) {
        None => {lettercount_map.insert(letter, 1);},
        Some(count) => {*count+=1}
        }
    };

    for letter in word.chars(){
        if ignore_spaces && letter == ' '{
            continue;
        } else {
            if case_sensitive{
                insert_closure(letter);
            } else {
                for lower_letter in letter.to_lowercase(){
                    insert_closure(lower_letter);
                }
            }
        }
    }

    lettercount_map
}

/// Returns true if two words are anagrams
/// 
/// `word_a` and `word_b` are the two words to check
/// 
/// If `case_sensitive` is `true`, uppercase and lowercase forms of the
/// same letter will be considered different. If `case_sensitive` is `false`,
/// uppercase and lowercase forms of th same letter will be considered the same.
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
/// const CASE_SENSITIVE: bool = true;
/// 
/// //proper anagram
/// assert!(are_anagrams("race", "care", CASE_SENSITIVE));
/// //non-proper anagram
/// assert!(are_anagrams("aabc", "caab", CASE_SENSITIVE));
/// 
/// //non-anagram due to different letters
/// assert!(!are_anagrams("race", "cow", CASE_SENSITIVE));
/// //non-anagram due to being identical
/// assert!(!are_anagrams("race", "race", CASE_SENSITIVE));
/// ```
pub fn are_anagrams(word_a: &str, word_b: &str, case_sensitive: bool) -> bool
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
    get_charcount_map(word_a, false, case_sensitive) 
        == get_charcount_map(word_b, false, case_sensitive)
}

/// Similar to [are_anagrams] but checks that both words are real words
/// 
/// This function will return false if either `word_a`, `word_b`, or both
/// are not found in the specified `wordlist`.
/// 
/// `wordlist` must implement the [Wordlist] trait (for example, the 
/// [default wordlist](crate::default_wordlist::default_wordlist) if present)
///# Examples
/// ```
/// use anagrambot::anagram::are_proper_anagrams;
/// use anagrambot::wordlist::BorrowedWordList;
/// 
/// const CASE_SENSITIVE: bool = true;
/// 
/// // you can use anagrambot::default_wordlist::default_wordlist()
/// // to get the default Wordlist instead of generating your own,
/// // as long as the `no-default-wordlist` feature is not enabled
/// const TEST_WORD_SET: [&str; 3] = ["race", "care", "cow"];
/// let wordlist: BorrowedWordList = TEST_WORD_SET.into_iter().collect();
/// 
/// //proper anagram
/// assert!(are_proper_anagrams("race", "care", &wordlist, CASE_SENSITIVE));
/// 
/// //non-proper anagram
/// assert!(!are_proper_anagrams("aabc", "caab", &wordlist, CASE_SENSITIVE));
/// //non-anagram due to different letters
/// assert!(!are_proper_anagrams("race", "cow", &wordlist, CASE_SENSITIVE));
/// //non-anagram due to being identical
/// assert!(!are_proper_anagrams("race", "race", &wordlist, CASE_SENSITIVE));
/// ```
pub fn are_proper_anagrams<'a>(word_a: &str, word_b: &str, wordlist: &impl Wordlist<'a>, 
    case_sensitive: bool) -> bool
{
    //return false if either word is not found in wordlist
    if !wordlist.includes_word(word_a){
        return false;
    } else if !wordlist.includes_word(word_b){
        return false;
    }

    //now that we ensured both words are real words, use the standard are_anagrams function
    are_anagrams(word_a, word_b, case_sensitive)
}

/// An iterator over all standard anagrams of a word
/// 
/// The return value of [find_anagrams]
pub struct AnagramsIter {
    chars: Vec<char>,
    stack_state: Vec<usize>,
    i: usize
}

impl AnagramsIter {
    pub fn new(word: &str) -> Self
    {
        let chars: Vec<char> = word.chars().collect();
        let stack_state = vec![0; chars.len()];

        Self { chars, stack_state, i: 1 }
    }
}

impl Iterator for AnagramsIter {
    type Item = String;

    // heaps algorithm graciously taken from wikipedia
    // and modified to function as a rust iterator
    // https://en.wikipedia.org/wiki/Heap's_algorithm
    fn next(&mut self) -> Option<Self::Item> {
        let seq_len = self.chars.len();
        if seq_len <= 1{
            return None;
        }
        while self.i < seq_len {
            let k = self.stack_state.get_mut(self.i).unwrap();
            if *k < self.i {
                if (self.i & 1) == 0 {
                    self.chars.swap(0, self.i);
                    
                } else {
                    self.chars.swap(*k, self.i);
                }
                // Swap has occurred ending the for-loop. Simulate the increment of the for-loop counter
                *k += 1;
                // Simulate recursive call reaching the base case by bringing the pointer to the base case analog in the array
                self.i = 1;

                return Some(self.chars.iter().collect());
            } else {
                // Calling generate(i+1, A) has ended as the for-loop terminated. Reset the state and simulate popping the stack by incrementing the pointer.
                *k = 0;
                self.i += 1;
            }
        }
        None
    }
}

/// Returns an [AnagramsIter] over all the standard anagrams of a word
/// 
/// Effectively returns an iterator over all permutations of word's characters,
/// except the original permutation (which is skipped because a word cannot be an anagram
/// of itself)
/// 
///# Notes
/// 
/// For a word of length `n`, there are `n! - 1` standard anagrams (`n!` meaning `factorial(n)`).
/// Factorials get up to extremely high output values for relatively low input values.
/// Be mindful of this if you plan to fill a vector with standard anagrams: 
/// storing ***all*** standard anagrams of a word may require multiple gigabytes of memory.
pub fn find_anagrams(word: &str) -> impl Iterator<Item = String>
{
    AnagramsIter::new(word)
}

/// An iterator over all the proper anagrams of a word
/// 
/// The return value of [find_proper_anagrams]
pub struct ProperAnagramsIter<'a, 'b, T>
where T: Iterator<Item = &'a str>
{
    word: &'b str,
    wordlist_iter: T,
    case_sensitive: bool
}

impl<'a, 'b, T> Iterator for ProperAnagramsIter<'a, 'b, T>
where T: Iterator<Item = &'a str>
{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next_word) = self.wordlist_iter.next() {
            if are_anagrams(self.word, next_word, self.case_sensitive){
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
/// 
///# Examples
/// ```
/// use anagrambot::anagram::find_proper_anagrams;
/// use anagrambot::wordlist::BorrowedWordList; 
/// 
/// const CASE_SENSITIVE: bool = true;
/// 
/// // you can use anagrambot::default_wordlist::default_wordlist()
/// // to get the default Wordlist instead of generating your own,
/// // as long as the `no-default-wordlist` feature is not enabled
/// const TEST_WORD_SET: [&str; 5] = ["aster", "taser", "tears", "race", "cow"];
/// let wordlist: BorrowedWordList = TEST_WORD_SET.into_iter().collect();
/// 
/// let mut proper_anagrams = find_proper_anagrams("tears", &wordlist, CASE_SENSITIVE);
/// 
/// assert_eq!(proper_anagrams.next(), Some("aster"));
/// assert_eq!(proper_anagrams.next(), Some("taser"));
/// assert_eq!(proper_anagrams.next(), None);
/// 
/// // note that the original word "tears" is not included because
/// // two identical words are not considered anagrams
/// ```
pub fn find_proper_anagrams<'a, 'b, T>(word: &'b str, wordlist: &'a T, case_sensitive: bool)
 -> ProperAnagramsIter<'a, 'b, impl Iterator<Item = &'a str>>
where T: Wordlist<'a>
{
    ProperAnagramsIter { word, wordlist_iter: wordlist.iter(), case_sensitive}
}