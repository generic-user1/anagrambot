//! Utilities for dealing with the default wordlist
//!
//! The default wordlist is a list of words built into the anagrambot project,
//! used when a wordlist is needed but no external wordlist file is provided. 
//! It may be excluded by using the `no-default-wordlist` feature.
//! 
//! The default wordlist was extracted from the Ubuntu 
//! [wamerican](https://packages.ubuntu.com/jammy/wamerican) package.
//! A copy of the copyright document distributed with the wamerican package is included 
//! in source distributions of the anagrambot project as `WORDLIST-LICENSE` or can be viewed
//! [online](http://changelogs.ubuntu.com/changelogs/pool/main/s/scowl/scowl_2020.12.07-2/copyright). 

use crate::wordlist::BorrowedWordList;

/// Returns the default wordlist content as a string literal, if present
/// 
/// If the project was built normally (i.e. without the `no-default-wordlist` feature),
/// this function will return `Some` containing the wordlist content. 
/// 
/// If the project was built with the `no-default-wordlist` feature,
/// this function will return `None`.
pub const fn default_wordlist_content() -> Option<&'static str>
{
    #[cfg(feature = "no-default-wordlist")]
    return None;

    #[cfg(not(feature = "no-default-wordlist"))]
    return Some(include_str!("../words.txt"));
}

/// Returns the default wordlist as a [BorrowedWordList], if present.
/// 
/// If the project was built normally (i.e. without the `no-default-wordlist` feature),
/// this function will return `Some` containing the wordlist. 
/// 
/// If the project was built with the `no-default-wordlist` feature,
/// this function will return `None`.
pub fn default_wordlist() -> Option<BorrowedWordList<'static>>
{
    default_wordlist_content().map(|wordlist_content|{wordlist_content.lines().collect()})
}