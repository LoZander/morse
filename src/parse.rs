use crate::morse::{Sen, Word};
use crate::morse::Sym::*;
use crate::morse::Char;

pub fn parse(s: String) -> Result<Sen,String> {
    s.trim().split('/').into_iter().map(|x| parse_words(String::from(x))).collect()
}

fn parse_words(s: String) -> Result<Word,String> {
    s.trim().split(" ").into_iter().map(|x| parse_char(String::from(x))).collect()
}

fn parse_char(s: String) -> Result<Char,String> {
    s.trim().chars()
     .into_iter()
     .map(|c| match c {
        '.' => Ok(Dot),
        '-' => Ok(Dash),
        e => Err(format!("Parsing error: {} is not a valid morse symbol", e))
     }).collect()
}
