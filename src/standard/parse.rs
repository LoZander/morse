use super::{types::{Sen,Word,Char,Sym::{Dot,Dash}, MorseResult, EncodePos}};

pub fn parse(s: String) -> MorseResult<Sen> {
    s.trim()
     .split('/')
     .enumerate()
     .map(|(i,x)| parse_words(i,String::from(x)))
     .collect()
}

fn parse_words(i: usize, s: String) -> MorseResult<Word> {
    s.trim()
     .split_whitespace()
     .map(String::from)
     .enumerate()
     .map(|(j, s)| parse_char(EncodePos(i,j),s))
     .collect()
}

fn parse_char(p: EncodePos,s: String) -> MorseResult<Char> {
    s.trim().chars()
     .into_iter()
     .map(|c| match c {
        '.' => Ok(Dot),
        '-' => Ok(Dash),
        e => Err(format!("parsing error: '{}' at position {} is not a valid morse symbol", e, p))
     })
     .collect()
}