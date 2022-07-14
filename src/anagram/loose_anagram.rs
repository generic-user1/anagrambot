//! Utilities for loose_anagrams
//! 
//! A loose anagram of a word is a proper anagram that can have a different
/// number of spaces (i.e. a different number of words).

use super::{Charmap, Wordlist, get_charcount_map};
use std::collections::HashMap;

/// Returns an Iterator over all loose anagrams of `target_word`
/// 
/// A loose anagram of a word is a proper anagram that can have a different
/// number of spaces (i.e. a different number of words).
/// 
/// `target_word` may or may not contain spaces; either is permitted. The resulting
/// loose anagrams may contain the same amount of spaces (i.e. proper anagrams),
/// fewer spaces, or more spaces.
/// 
///# Technical notes
/// 
/// Loose anagrams take significantly more computational effort to find than proper anagrams.
/// For this reason, the [LooseAnagramsIterator] caches partial results to decrease time spent waiting
/// on the next anagram to be generated. This caching behavior results in massive speed gains, but means 
/// that [LooseAnagramsIterator] instances may take more memory than you might think, especially for larger words.
/// 
/// Loose anagrams are also significantly more numerous than proper anagrams. Be mindful of this if you plan to fill
/// a vector with loose anagrams: storing ***all*** loose anagrams of a word may require multiple gigabytes of memory.
/// 
pub fn find_loose_anagrams<'a, T>(target_word: &str, wordlist: &'a T, case_sensitive:bool) 
-> LooseAnagramsIterator<'a> where T: Wordlist<'a>
{

    // get the charcount map of word (ignoring spaces)
    let target_charmap = get_charcount_map(target_word, true, case_sensitive);

    // find every word in the wordlist that can fit into the base word
    // and store them in full_candidate_set
    let full_candidate_set: HashMap<&str, Charmap> = wordlist.iter().filter_map(|word_b|{
            let charcount_map = get_charcount_map(word_b, true, case_sensitive);
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

    // hashmap containing the wordset that will fit into the specified charmap
    let candidate_map: HashMap<Charmap, Vec<(&str, Charmap)>> = HashMap::with_capacity(full_candidate_set.len());

    // vector containing the words to test fit into target word
    // this is where created words will be stored before verification
    // once verified, they are moved to result_vec
    let words_to_try: Vec<(Vec<&str>, Charmap)>;
    //tuple member 1 is the words that combine to make this word
    //tuple member 2 is the charmap of this word
    //tuple member 3 is the reduced charmap of this word's parent,
    //which was used to find this word

    // initially fill words_to_try with the candidate set
    words_to_try = full_candidate_set.iter().map(|item|{
        (vec![*item.0], item.1.clone())
    }).collect();

    // create a LooseAnagramsIterator from this data
    LooseAnagramsIterator{
        target_charmap,
        full_candidate_set,
        candidate_map,
        words_to_try
    }
}

/// An iterator over all the loose anagrams of a word
/// 
/// The return value of [find_loose_anagrams]
/// 
///# Technical Notes
/// 
/// See the Tecnical Notes section of [find_loose_anagrams]
pub struct LooseAnagramsIterator<'a> {
    target_charmap: Charmap,
    full_candidate_set: HashMap<&'a str, Charmap>,
    candidate_map: HashMap<Charmap, Vec<(&'a str, Charmap)>>,
    words_to_try: Vec<(Vec<&'a str>, Charmap)>
}

impl<'a> Iterator for LooseAnagramsIterator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((word_vec, word_charmap)) 
        = self.words_to_try.pop() {
            if word_charmap == self.target_charmap{
                return Some(word_vec.join(" "));
            } else {
                let allowed_words = match self.candidate_map.get(&word_charmap){
                    Some(map) => map,
                    None => {
                        // this word hasn't had allowed words generated yet
                        // create allowed words as a subset of parent's allowed words

                        let last_word = word_vec.last().unwrap();
                        let last_word_charmap = self.full_candidate_set.get(last_word).unwrap();
                        let parent_charmap = 
                            unsafe{
                                sub_charmaps(&word_charmap, last_word_charmap)
                            };
                        
                        let parent_words = 
                            match self.candidate_map.get(&parent_charmap){
                                Some(val) => val,
                                None => {
                                    let reduced_map =
                                    // it is safe to use sub_charmaps here because the word charmap will always fit
                                    // within the target charmap; if it didn't, it wouldn't be in words_to_try
                                    unsafe {sub_charmaps(&self.target_charmap, &parent_charmap)};


                                    let allowed_words = self.full_candidate_set.iter()
                                                .filter_map(|item|{
                                                    if word_fits(&reduced_map, &item.1){
                                                        Some((*item.0, item.1.clone()))
                                                    } else {
                                                        None
                                                    }
                                                }).collect();
                                    self.candidate_map.entry(parent_charmap).or_insert(allowed_words)
                                }
                            };
                        
                        //find reduced map; the map that words must fit into to still fit into
                        //the target word after 'word' has been included
                        let reduced_map = 
                        // it is safe to use sub_charmaps here because the word charmap will always fit
                        // within the target charmap; if it didn't, it wouldn't be in words_to_try
                        unsafe {sub_charmaps(&self.target_charmap, &word_charmap)};

                        let allowed_words = parent_words.iter()
                        .filter_map(|item|{
                            if word_fits(&reduced_map, &item.1){
                                Some(item.clone())
                            } else {
                                None
                            }
                        }).collect();
                        //store allowed words in candidate_map and return ref to newly stored words
                        self.candidate_map.entry(word_charmap.clone()).or_insert(allowed_words)
                    }
                };

                for allowed_word in allowed_words.iter() 
                {
                    let (subword, submap) = allowed_word;
                    
                    let mut subword_vec:Vec<&str> = Vec::with_capacity(word_vec.len() + 1);
                    subword_vec.clone_from(&word_vec);
                    subword_vec.push(subword);

                    let summed_map = 
                        add_charmaps(&word_charmap, &submap);
                    self.words_to_try.push((subword_vec, summed_map));
                }
            }
        }
        None
    }
}

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

/// Subtracts small_charmap from big_charmap and returns the result
/// 
/// return value contains all keys of big_charmap, except those
/// whose values are exactly matched within small_charmap (which are removed)
/// 
///# Unsafety
/// 
/// If small_charmap does not fit within big_charmap, incorrect behavior may result,
/// but this function does not check if small_charmap fits within big_charmap
unsafe fn sub_charmaps(big_charmap: &Charmap, small_charmap: &Charmap) -> Charmap
{
    let mut new_charmap = Charmap::new();
    for (key, bigvalue) in big_charmap{
        match small_charmap.get(key){
            None => {new_charmap.insert(*key, *bigvalue);},
            Some(smallvalue) => {
                //using word_fits earlier already ensured smallvalue is
                //less than or equal to bigvalue, so if they are not equal
                //then smallvalue must be less than bigvalue
                //if they are equal, the result of the subtraction would be zero
                //and we don't need to insert anything
                if smallvalue != bigvalue{
                    new_charmap.insert(*key, *bigvalue - *smallvalue);
                }
            }
        }
    }

    new_charmap
}