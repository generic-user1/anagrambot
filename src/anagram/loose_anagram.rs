//! Utilities for loose_anagrams
//! 
//! A loose anagram of a word is a proper anagram that can have a different
/// number of spaces (i.e. a different number of words).

use super::{Charmap, Wordlist, get_charcount_map};

/// Given the charcount map of word a and the charcount map of word b,
/// checks if word b would fit into word_a (i.e. that map b only has keys
/// that map a also has, and that the quantities of each key in word b are
/// less than or equal to the quantities in word a)
/// 
/// returns true if word_b would fit into word_a
fn word_fits(word_map_a: &Charmap, word_map_b: &Charmap) -> bool
{
    // if word map b has more keys than word map a, it cannot fit within word a
    if word_map_b.keys().len() > word_map_a.keys().len(){
        return false;
    }

    // iterate through map b's keys
    for map_b_key in word_map_b.keys() {
        // try to get this key in map a
        match word_map_a.get(map_b_key){
            // return false if this key does not exist in map a
            None => return false,
            Some(word_a_value) => {
                // check that word b's value for this key
                // is less than or equal to word a's value for the key
                // we can safely unwrap this because the key was retrived from word map b,
                // so it definitely exists
                let word_b_value = word_map_b.get(map_b_key).unwrap();
                if word_b_value > word_a_value {
                    return false;
                }
            }

        }
    }
    // if all keys in word b exist in word a,
    // and the word a amount for each key meets or exceeds
    // the word b amount, word b must fit into word a
    true
}

/// Returns a `Vec<String>` of all loose anagrams of `word`
/// 
/// A loose anagram of a word is a proper anagram that can have a different
/// number of spaces (i.e. a different number of words).
/// 
/// `word` may or may not contain spaces; either is permitted. The resulting
/// loose anagrams may contain the same amount of spaces (i.e. proper anagrams),
/// fewer spaces, or more spaces.
/// 
///# Technical Notes
/// Loose anagrams are more difficult to find than proper anagrams;
/// finding them requires a pre-analysis of all words in the wordlist.
/// Any returned iterator would need to do a significant amount of 
/// pre-computation before it could return its first value.
/// Thus, loose anagrams are returned as a `Vec` rather than an iterator
/// (unlike [find_proper_anagrams])
pub fn find_loose_anagrams<'a, T>(target_word: &str, wordlist: &'a T) -> Vec<String>
where T: Wordlist<'a>
{

    // get the charcount map of word (ignoring spaces)
    let target_charmap = get_charcount_map(target_word, true);
    
    let mut result_vec: Vec<String> = Vec::new();

    // vector containing the words to test fit into target word
    // this is where created words will be stored before verification
    // once verified, they are moved to result_vec
    let mut words_to_try: Vec<(String, Charmap)>;

    // find every word in the wordlist that can fit into the base word
    // and store them in full_candidate_set
    let full_candidate_set: Vec<(&str, Charmap)> = wordlist.iter().filter_map(|word_b|{
            let charcount_map = get_charcount_map(word_b, true);
            if word_fits(&target_charmap, &charcount_map){
                //dont include word if it's the same word
                if target_word == word_b{
                    None
                } else {
                    Some((word_b, charcount_map))
                }
            } else {
                None
            }
        }
    ).collect();

    // initially fill words_to_try with the candidate set
    words_to_try = full_candidate_set.iter().map(|item|{
        (item.0.to_string(), item.1.clone())
    }).collect();

    // iterate through words_to_try until it is empty
    // we can't use iterator because we need to pop each value off individually
    while let Some((word, word_charmap)) = words_to_try.pop() {

        if word_charmap == target_charmap{
            result_vec.push(word);
        } else {
            for (subword, submap) in full_candidate_set.iter() {
                let summed_map = 
                    add_charmaps(&word_charmap, &submap);
                if word_fits(&target_charmap, &summed_map){
                    let summed_word = if word == "" {
                        String::from(*subword)
                    } else {
                        word.clone() + " " + subword
                    };
                    words_to_try.push((summed_word, summed_map));
                }
            }
        }
    }

    result_vec
}

/// Adds charmap_a to charmap_b and returns the result
/// 
/// return value contains all keys of both charmap a and charmap b;
/// if both charmaps have a particular key, their values are summed
fn add_charmaps(charmap_a: &Charmap, charmap_b: &Charmap) -> Charmap
{
    let mut new_charmap = charmap_a.clone();
    for (key, value) in charmap_b{
        match new_charmap.get_mut(key) {
            Some(existing_value) => *existing_value += value,
            None => {new_charmap.insert(*key, *value);}
        }
    }
    new_charmap
}