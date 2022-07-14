use anagrambot::default_wordlist;
use anagrambot::anagram;
//use anagrambot::wordlist;

use std::time;

const PRINT_RESULTS: bool = false;
const CASE_SENSITIVE: bool = true;

fn main() {

   let wordlist = default_wordlist::default_wordlist()
    .expect("cannot perform demo without default wordlist!");

    /*
    let (largest_word, count) = most_loose_anagrams(&wordlist);
    println!("largest word is \"{}\", with {} loose anagrams", largest_word, count);
    */

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