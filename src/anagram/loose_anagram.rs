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
pub fn find_loose_anagrams<'a, T>(word: &str, wordlist: &'a T) -> Vec<String>
where T: Wordlist<'a>
{

    // define the recursive function used to find loose anagrams
    fn recursive_find_loose_anagrams<'a>(target_map: &Charmap, partial_word: &str,
        partial_map: &Charmap, wordlist: &Vec<(&'a str, Charmap)>) -> Vec<String>
    {
        if *target_map == *partial_map {
            return vec![String::from(partial_word)];
        }

        let mut returnable_words: Vec<String> = Vec::new();

        for (word, charmap) in wordlist{
            let summed_word = if partial_word == "" {String::from(*word)} else {
                String::from(partial_word) + " " + word
            };
            let summed_map = add_charmaps(partial_map, charmap);
            if summed_map == *target_map{
                returnable_words.push(summed_word);
            }
            else if word_fits(target_map, &summed_map){
                returnable_words.append(
                    &mut recursive_find_loose_anagrams(target_map, summed_word.as_str(),
                        &summed_map, wordlist)
                );
            }
        }
        returnable_words
    }

    // get the charcount map of word (ignoring spaces)
    let word_charcount_map = get_charcount_map(word, true);
    
    // find every word in the wordlist that can fit into the base word
    // and store the word and hashmap
    let full_candidate_set: Vec<(&str, Charmap)> = 
        wordlist.iter().filter_map(|word_b|{
            let charcount_map = get_charcount_map(word_b, true);
            if word_fits(&word_charcount_map, &charcount_map){
                //dont include word if it's the same word
                if word == word_b{
                    None
                } else {
                    Some((word_b, charcount_map))
                }
            } else {
                None
            }
        }
    ).collect();
    
    recursive_find_loose_anagrams(&word_charcount_map, 
        "", 
        &Charmap::new(), 
        &full_candidate_set
    )
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