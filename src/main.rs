use anagrambot::default_wordlist;
use anagrambot::anagram;
fn main() {

   let wordlist = default_wordlist::default_wordlist()
    .expect("cannot perform demo without default wordlist!");
   
    let target_word = "aster";

    let anagram_iter 
    = anagram::find_proper_anagrams(target_word, &wordlist);

    for proper_anagram in anagram_iter{
        println!("{} is anagram of {}", target_word, proper_anagram);
    }
}