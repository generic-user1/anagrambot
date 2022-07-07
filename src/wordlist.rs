//! The `Wordlist` struct and associated utilities

use std::{io::{self, BufRead}, fs, path};


/// an enum to allow either a Vec<String> or Vec<&str>
enum WordlistStorage<'a> 
{
    Owned(Vec<String>),
    Borrowed(Vec<&'a str>)
}

/// A list of words
pub struct Wordlist<'a> {
    word_set: WordlistStorage<'a>
}

impl<'a> FromIterator<&'a str> for Wordlist<'a> {
    
    /// Constructs a `Wordlist` from an iterator over `&str`s
    /// 
    /// Each value returned by the iterator should be one and only one word.
    /// If a value contains two words, they will be interpreted as one word with a
    /// space character as one of the letters
    /// 
    /// This is the underlying method that all other `Wordlist` constructors use
    fn from_iter<T: IntoIterator<Item = &'a str >>(iter: T) -> Self {
        Self { word_set: WordlistStorage::Borrowed(iter.into_iter().collect())}
    }
}

impl<'a> IntoIterator for &'a Wordlist<'a>{
    type Item = &'a str;
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;
    
    /// Returns an iterator over 
    fn into_iter(self) -> Self::IntoIter {
        match &self.word_set {
            WordlistStorage::Borrowed(vec) => {
                Box::new(vec.iter().map(|p|{*p}))
            },
            WordlistStorage::Owned(vec) => {
                Box::new(vec.iter().map(|p|{p.as_str()}))
            }
        }
    }
}

impl<'a> Wordlist<'a> {

    /// Similar to [Vec]'s `iter` function
    /// 
    /// Returns the result of `IntoIterator::into_iter(&WordList)`
    pub fn iter(&'a self) -> Box<dyn Iterator<Item = &'a str> + 'a>
    {
        self.into_iter()
    }

    /// Constructs a `Wordlist` from a `&str`
    ///
    /// `content` should be a `&str` containing a newline-delimited list of words.
    /// Each line should contain one word. If a line contains two words, they will be 
    /// treated as a single word with the space character interpreted as a letter.
    /// 
    /// If `content` doesn't contain any newlines, it is assumed to be a single word.
    pub fn from_str(content: &'a str) -> Self
    {
        // because Wordlist implements the FromIterator trait,
        // we can just call collect on the lines iterator        
        content.lines().collect()
    }

    /// Constructs a `Wordlist` from an open text file
    /// 
    /// `wordlist_file` should be a [Path](std::path::Path) to an existing text (UTF-8) file.
    /// Each line of the file should contain one word. 
    /// If a line contains two words, they will be treated as a
    /// single word with the space character interpreted as a letter.
    /// 
    /// If the content of `wordlist_file` doesn't contain any newlines,
    /// it is assumed to be a single word.
    pub fn from_file(wordlist_file_path: &path::Path) -> io::Result<Self>
    {
        let lines_iter = io::BufReader::new(fs::File::open(wordlist_file_path)?).lines();
        let mut word_vec = Vec::new();
        for line in lines_iter{
            word_vec.push(line?);
        }

        Ok(Self{word_set: WordlistStorage::Owned(word_vec)})
    }
}

#[cfg(test)]
mod tests {
    use super::Wordlist;

    #[test]
    fn test_iteration()
    {
        fn assertion<'a>(mut word_iterator: Box<dyn Iterator<Item = &'a str> + 'a>)
        {
            assert_eq!(word_iterator.next(), Some("a"));
            assert_eq!(word_iterator.next(), Some("aa"));
            assert_eq!(word_iterator.next(), Some("aaa"));
            assert_eq!(word_iterator.next(), Some("aaaa"));
        }

        let wordstring = "a\naa\naaa\naaaa";
        let wordlist = Wordlist::from_str(wordstring);
        let iter_one = wordlist.iter();
        
        assertion(iter_one);

        let iter_two = wordlist.iter();
        assertion(iter_two);


    }
}
