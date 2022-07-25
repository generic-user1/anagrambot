//! A simple benchmarking utility for testing anagrambot performance
//! 
//! Edit the `const`s in `main` to change how the benchmark functions.
//! Note that the benchmark requires the presence of `default_wordlist`.

use anagrambot::{default_wordlist, anagram::{find_proper_anagrams, find_loose_anagrams}};
use std::time::{Instant, Duration};

use std::collections::{HashMap, hash_map::Entry};

const NANOS_PER_SEC: f64 = 1e9;

fn main() {

    /// the word to generate anagrams of
    const TARGET_WORD: &str = "aster";

    /// the number of iterations in each test batch
    const LOWER_ITERATIONS: u32 = 10;

    /// the number of test batches to run
    const HIGHER_ITERATIONS: u32 = 10;

    /// the anagram types to test
    const BENCH_ANAGRAM_TYPES: [AnagramType; 2] = [AnagramType::Proper, AnagramType::Loose];

    /// case sensitivity
    const CASE_SENSITIVE: bool = true;

    let mut higher_dur_map: HashMap<AnagramType, Duration> = HashMap::with_capacity(BENCH_ANAGRAM_TYPES.len());

    for _ in 0..HIGHER_ITERATIONS{

        for anagram_type in BENCH_ANAGRAM_TYPES {
            let total_duration = run_bench(TARGET_WORD, 
                anagram_type, 
                LOWER_ITERATIONS, 
                CASE_SENSITIVE
            );

            dur_print(total_duration, Durtype::Total, LOWER_ITERATIONS);
            dur_print(total_duration, Durtype::Avg, LOWER_ITERATIONS);
        
            match higher_dur_map.entry(anagram_type){
                Entry::Occupied(mut entry) => {*entry.get_mut() += total_duration},
                Entry::Vacant(entry) => {entry.insert(total_duration);}
            }
        }
    }
    
    const TOTAL_ITERATIONS: u32 = HIGHER_ITERATIONS * LOWER_ITERATIONS;
    for anagram_type in BENCH_ANAGRAM_TYPES {
        println!("final results for {}:", anagram_type.name());
        let total_duration =  *higher_dur_map.get(&anagram_type).unwrap();
        dur_print(total_duration, Durtype::Total, TOTAL_ITERATIONS);
        dur_print(total_duration, Durtype::Avg, TOTAL_ITERATIONS);
    }

}

/// finds `anagram_type` anagrams of `target_word` `iterations` times
///
/// prints results of each iteration and returns total duration (each duration summed)
/// 
/// to find average from this, divide by `iterations` 
fn run_bench(target_word: &str, 
    anagram_type: AnagramType, 
    iterations: u32, 
    case_sensitive: bool) -> Duration
{
    
    const PRINT_INDIVIDUAL: bool = false;

    if !PRINT_INDIVIDUAL{
        println!("finding {} anagrams of {}", anagram_type.name(), target_word);
    }

    let default_wordlist = default_wordlist::default_wordlist().expect("failed to get default wordlist");

    let mut durations: Vec<Duration> = Vec::with_capacity(iterations as usize);

    for _ in 0..iterations {

        let (count, duration) = match anagram_type {
            AnagramType::Proper => {
                let start_time = Instant::now();
                let iter = find_proper_anagrams(
                    target_word, 
                    &default_wordlist, 
                    case_sensitive);
                let count = iter.count();
                let duration = start_time.elapsed();
                (count, duration)
            }
            AnagramType::Loose => {
                let start_time = Instant::now();
                let iter = find_loose_anagrams(
                    target_word, 
                    &default_wordlist, 
                    1, 
                    case_sensitive);
                let count = iter.count();
                let duration = start_time.elapsed();
                (count, duration)
            }
        };

        if PRINT_INDIVIDUAL {
            let dur_nanos = duration.as_nanos();
            let dur_secs = dur_nanos as f64 / NANOS_PER_SEC;
            println!("{} {} anagrams of {} found in {} s ({} ns)",
                count, anagram_type.name(), target_word, dur_secs, dur_nanos);
        }
        durations.push(duration);
    }

    durations.into_iter().sum::<Duration>() / iterations

}

/// helper function for printing durations in a human readable format
fn dur_print(total_duration: Duration, durtype: Durtype, iterations: u32) {
    let duration = match durtype {
        Durtype::Total => total_duration,
        Durtype::Avg => total_duration / iterations
    };
    let dur_nanos = duration.as_nanos();
    let dur_secs = dur_nanos as f64 / NANOS_PER_SEC;
    print!("{} duration:\t{} s ({} ns)",
        durtype.name(), dur_secs, dur_nanos);
    match durtype {
        Durtype::Avg => print!("\n"),
        Durtype::Total => print!(" over {} iterations\n", iterations)
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum AnagramType {
    Loose,
    Proper
}

impl AnagramType {
    pub fn name(&self) -> &'static str
    {
        match self {
            &AnagramType::Loose => "loose",
            &AnagramType::Proper => "proper"
        }
    }
}

enum Durtype{
    Total,
    Avg
}

impl Durtype{
    pub fn name(&self) -> &'static str
    {
        match self {
            &Durtype::Avg => "avg",
            &Durtype::Total => "total"
        }
    }
}