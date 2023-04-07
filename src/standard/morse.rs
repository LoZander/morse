use crate::standard::parse;
use crate::interfaces::types::{Sym::{Dash,Dot}, Sen, Word, Char, MorseResult, Pos};
use std::collections::hash_set::HashSet;

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
        Ok(sen.to_string())
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
        sen.iter()
       .enumerate()
       .map(|(i,w)| self.decode_word(i,w))
       .map(|x| x.map(|x| x + " "))
       .collect::<MorseResult<String>>()
       .map(|s| s.trim().to_string())
    }
}

impl MorseTranslater {
    fn decode_word(&self, index: usize, word: &Word) -> MorseResult<String> {
        word.iter()
            .enumerate()
            .map(|(j,c)| self.decode_character(Pos(index,j), c))
            .collect()
    }
    
    fn decode_character(&self, p: Pos, char: &Char) -> MorseResult<String> {
        self.symbol_map.get_plain(char.clone())
            .map_err(|_| format!("invalid morse sequence [{}] at position {}", char, p))
            .map(Into::into)
    }
}

pub struct SymbolMap {
    pairs: HashSet<(char, Char)>
}

impl Default for SymbolMap {
    fn default() -> Self {
        let mut set: HashSet<(char,Char)> = HashSet::new();

        set.insert(('a', [Dot,Dash].into()));
        set.insert(('b', [Dash,Dot,Dot,Dot].into()));
        set.insert(('c', [Dash,Dot,Dash,Dot].into()));
        set.insert(('d', [Dash,Dot,Dot].into()));
        set.insert(('e', [Dot].into()));
        set.insert(('f', [Dot,Dot,Dash,Dot].into()));
        set.insert(('g', [Dash,Dash,Dot].into()));
        set.insert(('h', [Dot,Dot,Dot,Dot].into()));
        set.insert(('i', [Dot,Dot].into()));
        set.insert(('j', [Dot,Dash,Dash,Dash].into()));
        set.insert(('k', [Dash,Dot,Dash].into()));
        set.insert(('l', [Dot,Dash,Dot,Dot].into()));
        set.insert(('m', [Dash,Dash].into()));
        set.insert(('n', [Dash,Dot].into()));
        set.insert(('o', [Dash,Dash,Dash].into()));
        set.insert(('p', [Dot,Dash,Dash,Dot].into()));
        set.insert(('q', [Dash,Dash,Dot,Dash].into()));
        set.insert(('r', [Dot,Dash,Dot].into()));
        set.insert(('s', [Dot,Dot,Dot].into()));
        set.insert(('t', [Dash].into()));
        set.insert(('u', [Dot,Dot,Dash].into()));
        set.insert(('v', [Dot,Dot,Dot,Dash].into()));
        set.insert(('w', [Dot,Dash,Dash].into()));
        set.insert(('x', [Dash,Dot,Dot,Dash].into()));
        set.insert(('y', [Dash,Dot,Dash,Dash].into()));
        set.insert(('z', [Dash,Dash,Dot,Dot].into()));
        set.insert(('ü', [Dot,Dot,Dash,Dash].into()));
        set.insert(('ä', [Dot,Dash,Dot,Dash].into()));
        set.insert(('ö', [Dash,Dash,Dash,Dot].into()));
        set.insert(('1', [Dot,Dash,Dash,Dash,Dash].into()));
        set.insert(('2', [Dot,Dot,Dash,Dash,Dash].into()));
        set.insert(('3', [Dot,Dot,Dot,Dash,Dash].into()));
        set.insert(('4', [Dot,Dot,Dot,Dot,Dash].into()));
        set.insert(('5', [Dot,Dot,Dot,Dot,Dot].into()));
        set.insert(('6', [Dash,Dot,Dot,Dot,Dot].into()));
        set.insert(('7', [Dash,Dash,Dot,Dot,Dot].into()));
        set.insert(('8', [Dash,Dash,Dash,Dot,Dot].into()));
        set.insert(('9', [Dash,Dash,Dash,Dash,Dot].into()));
        set.insert(('0', [Dash,Dash,Dash,Dash,Dash].into()));
        set.insert(('é', [Dot,Dot,Dash,Dot,Dot].into()));
        set.insert(('è', [Dot,Dash,Dot,Dot,Dash].into()));
        set.insert(('à', [Dot,Dash,Dash,Dot,Dash].into()));
        set.insert(('+', [Dot,Dash,Dot,Dash,Dot].into()));
        set.insert(('=', [Dash,Dot,Dot,Dot,Dash].into()));
        set.insert(('/', [Dash,Dot,Dot,Dash,Dot].into()));
        set.insert(('ñ', [Dash,Dash,Dot,Dash,Dash].into()));
        set.insert(('?', [Dot,Dot,Dash,Dash,Dot,Dot].into()));
        set.insert(('_', [Dot,Dot,Dash,Dash,Dot,Dash].into()));
        set.insert(('"', [Dot,Dash,Dot,Dot,Dash,Dot].into()));
        set.insert(('.', [Dot,Dash,Dot,Dash,Dot,Dash].into()));
        set.insert(('@', [Dot,Dash,Dash,Dot,Dash,Dot].into()));
        set.insert(('\'', [Dot,Dash,Dash,Dash,Dash,Dot].into()));
        set.insert(('-', [Dash,Dot,Dot,Dot,Dot,Dash].into()));
        set.insert((';', [Dash,Dot,Dash,Dot,Dash,Dot].into()));
        set.insert(('!', [Dash,Dot,Dash,Dot,Dash,Dash].into()));
        set.insert(('(', [Dash,Dot,Dash,Dash,Dot,Dash].into()));
        set.insert((')', [Dash,Dot,Dash,Dash,Dot,Dash].into()));
        set.insert((',', [Dash,Dash,Dot,Dot,Dash,Dash].into()));
        set.insert((':', [Dash,Dash,Dash,Dot,Dot,Dot].into()));

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