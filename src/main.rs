use anagrambot::default_wordlist;
fn main() {

    if let Some(content) = default_wordlist::default_wordlist_content()
    {
        print!("{}", content);
    } else {
        println!("No default wordlist!");
    }
}
