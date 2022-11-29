use crate::{types::{Sen,Word,Char,Sym::{Dot,Dash}}};

pub fn parse(s: String) -> Sen {
    s.trim()
     .split('/')
     .into_iter()
     .map(|x| parse_words(String::from(x)))
     .collect()
}

fn parse_words(s: String) -> Word {
    s.trim()
     .split(' ')
     .map(String::from)
     .into_iter()
     .map(parse_char)
     .collect()
}

fn parse_char(s: String) -> Char {
    s.trim().chars()
     .into_iter()
     .map(|c| match c {
        '.' => Ok(Dot),
        '-' => Ok(Dash),
        e => Err(format!("Parsing error: {} is not a valid morse symbol", e))
     })
     .collect::<Result<Char,String>>()
     .unwrap_or_default()
}