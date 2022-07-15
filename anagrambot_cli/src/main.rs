use anagrambot::default_wordlist;
use anagrambot::anagram;

use std::time;

const PRINT_RESULTS: bool = false;
const CASE_SENSITIVE: bool = true;

fn main() {

   let wordlist = default_wordlist::default_wordlist()
    .expect("cannot perform demo without default wordlist!");

    let target_word = "Adirondacks's";
    
    let start_time = time::Instant::now();

    let loose_anagrams = anagram::find_loose_anagrams(target_word, &wordlist, CASE_SENSITIVE);

    let mut anagram_count: usize = 0;
    for loose_anagram in loose_anagrams{
        anagram_count+=1;
        if PRINT_RESULTS{
            println!("{} is anagram of {}", target_word, loose_anagram);
        }
    }
    let run_duration = (time::Instant::now() - start_time).as_nanos();

    println!("took {}s ({}ns) to find{} {} anagrams", 
        (run_duration as f64/1e9),
        run_duration,
        if PRINT_RESULTS {" and print"} else {""},
        anagram_count
    );
}