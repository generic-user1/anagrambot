use anagrambot::default_wordlist;
use anagrambot::anagram;
//use anagrambot::wordlist;

use std::time;

const PRINT_RESULTS: bool = false;

fn main() {

   let wordlist = default_wordlist::default_wordlist()
    .expect("cannot perform demo without default wordlist!");

    /*
    let (largest_word, count) = most_loose_anagrams(&wordlist);
    println!("largest word is \"{}\", with {} loose anagrams", largest_word, count);
    */

    let target_word = "Adirondacks's";
    
    let start_time = time::Instant::now();

    let loose_anagrams = anagram::find_loose_anagrams(target_word, &wordlist);

    let run_duration = (time::Instant::now() - start_time).as_nanos();
    let anagram_count = loose_anagrams.len();

    if PRINT_RESULTS{
        for loose_anagram in loose_anagrams{
            println!("{} is anagram of {}", target_word, loose_anagram);
        }
    }

    println!("took {}s ({}ns) to find {} anagrams", 
        (run_duration as f64/1e9),
        run_duration,
        anagram_count
    );
}

/*
fn most_loose_anagrams<'a>(wordlist: &'a impl wordlist::Wordlist<'a>) -> (&'a str, usize)
{
    let mut largest_word: &'a str = "";
    let mut largest_count: usize = 0;

    for word in wordlist.iter(){
        println!("testing {}", word);
        let word_count = anagram::find_loose_anagrams(word, wordlist).len();
        if word_count > largest_count {
            largest_count = word_count;
            largest_word = word;
        }
    }

    (largest_word, largest_count)
}
*/