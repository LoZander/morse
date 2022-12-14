use crate::interfaces::types::{Sen, MorseResult, Word, Char, Sym, Pos};

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
     .map(|(j, s)| parse_char(Pos(i,j),s))
     .collect()
}

fn parse_char(p: Pos,s: String) -> MorseResult<Char> {
    s.trim().chars()
     .into_iter()
     .map(|c| match c {
        '.' => Ok(Sym::Dot),
        '-' => Ok(Sym::Dash),
        e => Err(format!("parsing error: invalid symbol '{}' at position {}", e, p))
     })
     .collect()
}