use crate::standard::parse;
use crate::interfaces::types::{Sym::{Dash,Dot}, Sen, Word, Char, string_of_sen, string_of_char, MorseResult, Pos};
use std::collections::hash_set::{HashSet};
use std::hash::Hash;

pub trait MorseEncoder {
    fn encode<T: ToString>(&self, plaintext: T) -> MorseResult<String>;
}

pub trait MorseDecoder {
    fn decode<T: ToString>(&self, cipher: T) -> MorseResult<String>;
}

pub struct MorseTranslater {
    symbol_map: SymbolMap
}

impl MorseTranslater {
    pub fn new(symbol_map: SymbolMap) -> Self {
        MorseTranslater { symbol_map }
    }
}

impl Default for MorseTranslater {
    fn default() -> Self {
        Self { symbol_map: Default::default() }
    }
}

impl MorseEncoder for MorseTranslater {
    fn encode<T: ToString>(&self, plaintext: T) -> MorseResult<String> {
        let sen: Sen = plaintext.to_string()
                        .to_lowercase()
                        .split_whitespace()
                        .enumerate()
                        .map(|(i,s)| (i,s.to_string()))
                        .map(|(i,s)| self.encode_word(i,s))
                        .collect::<MorseResult<_>>()?;
        Ok(string_of_sen(&sen))
    }
}

impl MorseTranslater {
    fn encode_word(&self, word_number: usize, plaintext: String) -> MorseResult<Word> {
        plaintext.char_indices()
         .map(|(i,c)| self.encode_char(Pos(word_number, i),c))
         .collect()
    }
    
    fn encode_char(&self, p: Pos, c: char) -> MorseResult<Char> {
        self.symbol_map.get_morse(c).map_err(|_| format!("invalid character '{c}' at position {p}"))
    }
}

impl MorseDecoder for MorseTranslater {
    fn decode<T: ToString>(&self,cipher: T) -> MorseResult<String> {
        let sen: Sen = parse::parse(cipher)?;
        sen.into_iter()
       .enumerate()
       .map(|(i,w)| self.decode_word(i,w))
       .map(|x| x.map(|x| x + " "))
       .collect::<MorseResult<String>>()
       .map(|s| s.trim().to_string())
    }
}

impl MorseTranslater {
    fn decode_word(&self, index: usize, word: Word) -> MorseResult<String> {
        word.into_iter()
            .enumerate()
            .map(|(j,c)| self.decode_character(Pos(index,j), c))
            .collect()
    }
    
    fn decode_character(&self, p: Pos, char: Char) -> MorseResult<String> {
        self.symbol_map.get_plain(char.clone())
            .map_err(|_| format!("invalid morse sequence [{}] at position {}", string_of_char(&char), p))
            .map(Into::into)
    }
}

pub struct SymbolMap {
    pairs: HashSet<(char, Char)>
}

impl Default for SymbolMap {
    fn default() -> Self {
        let mut set: HashSet<(char,Char)> = HashSet::new();

        set.insert(('a', vec![Dot,Dash]));
        set.insert(('b', vec![Dash,Dot,Dot,Dot]));
        set.insert(('c', vec![Dash,Dot,Dash,Dot]));
        set.insert(('d', vec![Dash,Dot,Dot]));
        set.insert(('e', vec![Dot]));
        set.insert(('f', vec![Dot,Dot,Dash,Dot]));
        set.insert(('g', vec![Dash,Dash,Dot]));
        set.insert(('h', vec![Dot,Dot,Dot,Dot]));
        set.insert(('i', vec![Dot,Dot]));
        set.insert(('j', vec![Dot,Dash,Dash,Dash]));
        set.insert(('k', vec![Dash,Dot,Dash]));
        set.insert(('l', vec![Dot,Dash,Dot,Dot]));
        set.insert(('m', vec![Dash,Dash]));
        set.insert(('n', vec![Dash,Dot]));
        set.insert(('o', vec![Dash,Dash,Dash]));
        set.insert(('p', vec![Dot,Dash,Dash,Dot]));
        set.insert(('q', vec![Dash,Dash,Dot,Dash]));
        set.insert(('r', vec![Dot,Dash,Dot]));
        set.insert(('s', vec![Dot,Dot,Dot]));
        set.insert(('t', vec![Dash]));
        set.insert(('u', vec![Dot,Dot,Dash]));
        set.insert(('v', vec![Dot,Dot,Dot,Dash]));
        set.insert(('w', vec![Dot,Dash,Dash]));
        set.insert(('x', vec![Dash,Dot,Dot,Dash]));
        set.insert(('y', vec![Dash,Dot,Dash,Dash]));
        set.insert(('z', vec![Dash,Dash,Dot,Dot]));
        set.insert(('ü', vec![Dot,Dot,Dash,Dash]));
        set.insert(('ä', vec![Dot,Dash,Dot,Dash]));
        set.insert(('ö', vec![Dash,Dash,Dash,Dot]));
        set.insert(('1', vec![Dot,Dash,Dash,Dash,Dash]));
        set.insert(('2', vec![Dot,Dot,Dash,Dash,Dash]));
        set.insert(('3', vec![Dot,Dot,Dot,Dash,Dash]));
        set.insert(('4', vec![Dot,Dot,Dot,Dot,Dash]));
        set.insert(('5', vec![Dot,Dot,Dot,Dot,Dot]));
        set.insert(('6', vec![Dash,Dot,Dot,Dot,Dot]));
        set.insert(('7', vec![Dash,Dash,Dot,Dot,Dot]));
        set.insert(('8', vec![Dash,Dash,Dash,Dot,Dot]));
        set.insert(('9', vec![Dash,Dash,Dash,Dash,Dot]));
        set.insert(('0', vec![Dash,Dash,Dash,Dash,Dash]));
        set.insert(('é', vec![Dot,Dot,Dash,Dot,Dot]));
        set.insert(('è', vec![Dot,Dash,Dot,Dot,Dash]));
        set.insert(('à', vec![Dot,Dash,Dash,Dot,Dash]));
        set.insert(('+', vec![Dot,Dash,Dot,Dash,Dot]));
        set.insert(('=', vec![Dash,Dot,Dot,Dot,Dash]));
        set.insert(('/', vec![Dash,Dot,Dot,Dash,Dot]));
        set.insert(('ñ', vec![Dash,Dash,Dot,Dash,Dash]));
        set.insert(('?', vec![Dot,Dot,Dash,Dash,Dot,Dot]));
        set.insert(('_', vec![Dot,Dot,Dash,Dash,Dot,Dash]));
        set.insert(('"', vec![Dot,Dash,Dot,Dot,Dash,Dot]));
        set.insert(('.', vec![Dot,Dash,Dot,Dash,Dot,Dash]));
        set.insert(('@', vec![Dot,Dash,Dash,Dot,Dash,Dot]));
        set.insert(('\'', vec![Dot,Dash,Dash,Dash,Dash,Dot]));
        set.insert(('-', vec![Dash,Dot,Dot,Dot,Dot,Dash]));
        set.insert((';', vec![Dash,Dot,Dash,Dot,Dash,Dot]));
        set.insert(('!', vec![Dash,Dot,Dash,Dot,Dash,Dash]));
        set.insert(('(', vec![Dash,Dot,Dash,Dash,Dot,Dash]));
        set.insert((')', vec![Dash,Dot,Dash,Dash,Dot,Dash]));
        set.insert((',', vec![Dash,Dash,Dot,Dot,Dash,Dash]));
        set.insert((':', vec![Dash,Dash,Dash,Dot,Dot,Dot]));

        Self { pairs: set }
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