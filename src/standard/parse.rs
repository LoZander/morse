use crate::interfaces::types::{Sen, MorseResult, Word, Char, Sym, Pos};

pub fn parse<T: ToString>(s: T) -> MorseResult<Sen> {
    s.to_string()
     .trim()
     .split('/')
     .enumerate()
     .map(|(i,x)| parse_words(i,String::from(x)))
     .collect()
}

fn parse_words(i: usize, s: String) -> MorseResult<Word> {
    s.split_whitespace()
     .map(String::from)
     .enumerate()
     .map(|(j, s)| parse_char(Pos(i,j),s))
     .collect()
}

fn parse_char(p: Pos,s: String) -> MorseResult<Char> {
    s.trim().chars()
     .map(|c| match c {
        '.' => Ok(Sym::Dot),
        '-' => Ok(Sym::Dash),
        e => Err(format!("parsing error: invalid symbol '{e}' at position {p}"))
     })
     .collect()
}