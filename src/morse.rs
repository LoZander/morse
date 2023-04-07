use std::collections::hash_set::HashSet;

use crate::{types::{MorseResult, Pos, Word, Char, Sym::{Dash, Dot}, Sen}, parse};

pub fn encode<T: ToString>(plaintext: T, map: &SymbolMap) -> MorseResult<String> {
    let sen: Sen = plaintext.to_string()
                    .to_lowercase()
                    .split_whitespace()
                    .enumerate()
                    .map(|(i,s)| (i,s.to_string()))
                    .map(|(i,s)| encode_word(i,s, map))
                    .collect::<MorseResult<_>>()?;
    Ok(sen.to_string())
}

fn encode_word(word_number: usize, plaintext: String, map: &SymbolMap) -> MorseResult<Word> {
    plaintext.char_indices()
     .map(|(i,c)| encode_char(Pos(word_number, i),c, map))
     .collect()
}

fn encode_char(p: Pos, c: char, map: &SymbolMap) -> MorseResult<Char> {
    map.get_morse(c).map_err(|_| format!("invalid character '{c}' at position {p}"))
}

pub fn decode<T: ToString>(cipher: T, map: &SymbolMap) -> MorseResult<String> {
    let sen: Sen = parse::parse(cipher)?;
    sen.iter()
   .enumerate()
   .map(|(i,w)| decode_word(i,w, map))
   .map(|x| x.map(|x| x + " "))
   .collect::<MorseResult<String>>()
   .map(|s| s.trim().to_string())
}

fn decode_word(index: usize, word: &Word, map: &SymbolMap) -> MorseResult<String> {
    word.iter()
        .enumerate()
        .map(|(j,c)| decode_character(Pos(index,j), c, map))
        .collect()
}

fn decode_character(p: Pos, char: &Char, map: &SymbolMap) -> MorseResult<String> {
    map.get_plain(char.clone())
       .map_err(|_| format!("invalid morse sequence [{}] at position {}", char, p))
       .map(Into::into)
}

pub struct SymbolMap {
    pairs: HashSet<(char, Char)>
}

pub struct SymbolMapBuilder {
    map: HashSet<(char, Char)>
}

impl Default for SymbolMapBuilder {
    fn default() -> Self {
        Self { map: Default::default() }
        .with('a',  [Dot,Dash])                     .unwrap()
        .with('b',  [Dash,Dot,Dot,Dot])             .unwrap()
        .with('c',  [Dash,Dot,Dash,Dot])            .unwrap()
        .with('d',  [Dash,Dot,Dot])                 .unwrap()
        .with('e',  [Dot])                          .unwrap()
        .with('f',  [Dot,Dot,Dash,Dot])             .unwrap()
        .with('g',  [Dash,Dash,Dot])                .unwrap()
        .with('h',  [Dot,Dot,Dot,Dot])              .unwrap()
        .with('i',  [Dot,Dot])                      .unwrap()
        .with('j',  [Dot,Dash,Dash,Dash])           .unwrap()
        .with('k',  [Dash,Dot,Dash])                .unwrap()
        .with('l',  [Dot,Dash,Dot,Dot])             .unwrap()
        .with('m',  [Dash,Dash])                    .unwrap()
        .with('n',  [Dash,Dot])                     .unwrap()
        .with('o',  [Dash,Dash,Dash])               .unwrap()
        .with('p',  [Dot,Dash,Dash,Dot])            .unwrap()
        .with('q',  [Dash,Dash,Dot,Dash])           .unwrap()
        .with('r',  [Dot,Dash,Dot])                 .unwrap()
        .with('s',  [Dot,Dot,Dot])                  .unwrap()
        .with('t',  [Dash])                         .unwrap()
        .with('u',  [Dot,Dot,Dash])                 .unwrap()
        .with('v',  [Dot,Dot,Dot,Dash])             .unwrap()
        .with('w',  [Dot,Dash,Dash])                .unwrap()
        .with('x',  [Dash,Dot,Dot,Dash])            .unwrap()
        .with('y',  [Dash,Dot,Dash,Dash])           .unwrap()
        .with('z',  [Dash,Dash,Dot,Dot])            .unwrap()
        .with('ü',  [Dot,Dot,Dash,Dash])            .unwrap()
        .with('ä',  [Dot,Dash,Dot,Dash])            .unwrap()
        .with('ö',  [Dash,Dash,Dash,Dot])           .unwrap()
        .with('1',  [Dot,Dash,Dash,Dash,Dash])      .unwrap()
        .with('2',  [Dot,Dot,Dash,Dash,Dash])       .unwrap()
        .with('3',  [Dot,Dot,Dot,Dash,Dash])        .unwrap()
        .with('4',  [Dot,Dot,Dot,Dot,Dash])         .unwrap()
        .with('5',  [Dot,Dot,Dot,Dot,Dot])          .unwrap()
        .with('6',  [Dash,Dot,Dot,Dot,Dot])         .unwrap()
        .with('7',  [Dash,Dash,Dot,Dot,Dot])        .unwrap()
        .with('8',  [Dash,Dash,Dash,Dot,Dot])       .unwrap()
        .with('9',  [Dash,Dash,Dash,Dash,Dot])      .unwrap()
        .with('0',  [Dash,Dash,Dash,Dash,Dash])     .unwrap()
        .with('é',  [Dot,Dot,Dash,Dot,Dot])         .unwrap()
        .with('è',  [Dot,Dash,Dot,Dot,Dash])        .unwrap()
        .with('à',  [Dot,Dash,Dash,Dot,Dash])       .unwrap()
        .with('+',  [Dot,Dash,Dot,Dash,Dot])        .unwrap()
        .with('=',  [Dash,Dot,Dot,Dot,Dash])        .unwrap()
        .with('/',  [Dash,Dot,Dot,Dash,Dot])        .unwrap()
        .with('ñ',  [Dash,Dash,Dot,Dash,Dash])      .unwrap()
        .with('?',  [Dot,Dot,Dash,Dash,Dot,Dot])    .unwrap()
        .with('_',  [Dot,Dot,Dash,Dash,Dot,Dash])   .unwrap()
        .with('"',  [Dot,Dash,Dot,Dot,Dash,Dot])    .unwrap()
        .with('.',  [Dot,Dash,Dot,Dash,Dot,Dash])   .unwrap()
        .with('@',  [Dot,Dash,Dash,Dot,Dash,Dot])   .unwrap()
        .with('\'', [Dot,Dash,Dash,Dash,Dash,Dot])  .unwrap()
        .with('-',  [Dash,Dot,Dot,Dot,Dot,Dash])    .unwrap()
        .with(';',  [Dash,Dot,Dash,Dot,Dash,Dot])   .unwrap()
        .with('!',  [Dash,Dot,Dash,Dot,Dash,Dash])  .unwrap()
        .with('(',  [Dash,Dot,Dash,Dash,Dot,Dash])  .unwrap()
        .with(')',  [Dash,Dot,Dash,Dash,Dot,Dash])  .unwrap()
        .with(',',  [Dash,Dash,Dot,Dot,Dash,Dash])  .unwrap()
        .with(':',  [Dash,Dash,Dash,Dot,Dot,Dot])   .unwrap()
    }
}

impl SymbolMapBuilder {
    pub fn new() -> Self {
        Self { map: HashSet::new() }
    }

    pub fn with<T: Into<Char>>(mut self, plain: char, cipher: T) -> Result<Self, String> {
        let cipher: Char = cipher.into();
        let added = self.map.insert((plain, cipher.clone()));
        if !added {
            return Err(format!("duplicate error: failed to add ({}, {}) as it's a duplicate entry", plain, cipher))
        }

        Ok(self)
    }

    pub fn build(self) -> SymbolMap {
        SymbolMap { pairs: self.map }
    }
}

impl Default for SymbolMap {
    fn default() -> Self {
        SymbolMapBuilder::default().build()
    }
}

impl SymbolMap {
    pub fn new(pairs: HashSet<(char, Char)>) -> Self {
        SymbolMap{pairs}
    }

    pub fn get_morse(&self, plaintext: char) -> MorseResult<Char> {
        self.pairs.clone().into_iter()
            .find(|(plain,_)| *plain == plaintext)
            .ok_or(format!("{plaintext} is not a valid symbol"))
            .map(|(_,morse)| morse)
    }
    pub fn get_plain(&self, morse: Char) -> MorseResult<char> {
        self.pairs.clone().into_iter()
            .find(|(_, m)| m.clone() == morse)
            .ok_or(format!("{morse:?} is not a valid morse sequence"))
            .map(|(plain,_)| plain)
    }
}