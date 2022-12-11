use crate::standard::parse;
use crate::interfaces::types::{Sym::{Dash,Dot}, Sen, Word, Char, string_of_sen, string_of_char, MorseResult, Pos};

pub trait MorseEncoder {
    /**
     * Encodes a message to morse code. Any unrecognized symbols are simply thrown away.
     */
    fn encode(plaintext: String) -> String;
}

pub trait MorseDecoder {
    /**
     * Decodes a morse code sentence, where '/' is the word separator
     */
    fn decode(cipher: String) -> String;
}

pub fn decode(cipher: String) -> MorseResult<String> {
    let sen: Sen = parse::parse(cipher)?;
    sen.into_iter()
       .enumerate()
       .map(|(i,w)| decode_word(i,w))
       .map(|x| x.map(|x| x + " "))
       .collect::<MorseResult<String>>()
       .map(|s| s.trim().to_string())
}

fn decode_word(index: usize, word: Word) -> MorseResult<String> {
    word.into_iter()
        .enumerate()
        .map(|(j,c)| decode_character(Pos(index,j), c))
        .collect()
}

fn decode_character(p: Pos, char: Char) -> MorseResult<String> {
    match char[..] {
        [Dot,Dash]                      => Ok("a"),
        [Dash,Dot,Dot,Dot]              => Ok("b"),
        [Dash,Dot,Dash,Dot]             => Ok("c"),
        [Dash,Dot,Dot]                  => Ok("d"),
        [Dot]                           => Ok("e"),
        [Dot,Dot,Dash,Dot]              => Ok("f"),
        [Dash,Dash,Dot]                 => Ok("g"),
        [Dot,Dot,Dot,Dot]               => Ok("h"),
        [Dot,Dot]                       => Ok("i"),
        [Dot,Dash,Dash,Dash]            => Ok("j"),
        [Dash,Dot,Dash]                 => Ok("k"),
        [Dot,Dash,Dot,Dot]              => Ok("l"),
        [Dash,Dash]                     => Ok("m"),
        [Dash,Dot]                      => Ok("n"),
        [Dash,Dash,Dash]                => Ok("o"),
        [Dot,Dash,Dash,Dot]             => Ok("p"),
        [Dash,Dash,Dot,Dash]            => Ok("q"),
        [Dot,Dash,Dot]                  => Ok("r"),
        [Dot,Dot,Dot]                   => Ok("s"),
        [Dash]                          => Ok("t"),
        [Dot,Dot,Dash]                  => Ok("u"),
        [Dot,Dot,Dot,Dash]              => Ok("v"),
        [Dot,Dash,Dash]                 => Ok("w"),
        [Dash,Dot,Dot,Dash]             => Ok("x"),
        [Dash,Dot,Dash,Dash]            => Ok("y"),
        [Dash,Dash,Dot,Dot]             => Ok("z"),
        [Dot,Dot,Dash,Dash]             => Ok("ü"),
        [Dot,Dash,Dot,Dash]             => Ok("ö"),
        [Dot,Dash,Dash,Dash,Dash]       => Ok("1"),
        [Dot,Dot,Dash,Dash,Dash]        => Ok("2"),
        [Dot,Dot,Dot,Dash,Dash]         => Ok("3"),
        [Dot,Dot,Dot,Dot,Dash]          => Ok("4"),
        [Dot,Dot,Dot,Dot,Dot]           => Ok("5"),
        [Dash,Dot,Dot,Dot,Dot]          => Ok("6"),
        [Dash,Dash,Dot,Dot,Dot]         => Ok("7"),
        [Dash,Dash,Dash,Dot,Dot]        => Ok("8"),
        [Dash,Dash,Dash,Dash,Dot]       => Ok("9"),
        [Dash,Dash,Dash,Dash,Dash]      => Ok("0"),
        [Dot,Dot,Dash,Dot,Dot]          => Ok("é"),
        [Dot,Dash,Dot,Dot,Dash]         => Ok("è"),
        [Dot,Dash,Dash,Dot,Dash]        => Ok("à"),
        [Dot,Dash,Dot,Dash,Dot]         => Ok("+"),
        [Dash,Dot,Dot,Dot,Dash]         => Ok("="),
        [Dash,Dot,Dot,Dash,Dot]         => Ok("/"),
        [Dash,Dash,Dot,Dash,Dash]       => Ok("ñ"),
        [Dot,Dot,Dash,Dash,Dot,Dot]     => Ok("?"),
        [Dot,Dot,Dash,Dash,Dot,Dash]    => Ok("_"),
        [Dot,Dash,Dot,Dot,Dash,Dot]     => Ok("\""),
        [Dot,Dash,Dot,Dash,Dot,Dash]    => Ok("."),
        [Dot,Dash,Dash,Dot,Dash,Dot]    => Ok("@"),
        [Dot,Dash,Dash,Dash,Dash,Dot]   => Ok("\\"),
        [Dash,Dot,Dot,Dot,Dot,Dash]     => Ok("-"),
        [Dash,Dot,Dash,Dot,Dash,Dot]    => Ok(";"),
        [Dash,Dot,Dash,Dot,Dash,Dash]   => Ok("!"),
        [Dash,Dot,Dash,Dash,Dot,Dash]   => Ok("|"),
        [Dash,Dash,Dot,Dot,Dash,Dash]   => Ok(","),
        [Dash,Dash,Dash,Dot,Dot,Dot]    => Ok(":"),
        _                               => Err(format!("invalid morse sequence [{}] at position {}", string_of_char(&char), p))
    }.map(str::to_string)
}

pub fn encode(plaintext: String) -> MorseResult<String> {
        let sen: Sen = plaintext.to_lowercase()
                        .split_whitespace()
                        .enumerate()
                        .map(|(i,s)| (i,s.to_string()))
                        .map(|(i,s)| encode_word(i,s))
                        .collect::<MorseResult<_>>()?;
        Ok(string_of_sen(&sen))
}

fn encode_word(word_number: usize, plaintext: String) -> MorseResult<Word> {
    plaintext.char_indices()
     .into_iter()
     .map(|(i,c)| encode_char(Pos(word_number, i),c))
     .collect()
}

fn encode_char(p: Pos, c: char) -> MorseResult<Char> {
    match c {
        'a' => Ok(vec![Dot,Dash]),
        'b' => Ok(vec![Dash,Dot,Dot,Dot]),
        'c' => Ok(vec![Dash,Dot,Dash,Dot]),
        'd' => Ok(vec![Dash,Dot,Dot]),
        'e' => Ok(vec![Dot]),
        'f' => Ok(vec![Dot,Dot,Dash,Dot]),
        'g' => Ok(vec![Dash,Dash,Dot]),
        'h' => Ok(vec![Dot,Dot,Dot,Dot]),
        'i' => Ok(vec![Dot,Dot]),
        'j' => Ok(vec![Dot,Dash,Dash,Dash]),
        'k' => Ok(vec![Dash,Dot,Dash]),
        'l' => Ok(vec![Dot,Dash,Dot,Dot]),
        'm' => Ok(vec![Dash,Dash]),
        'n' => Ok(vec![Dash,Dot]),
        'o' => Ok(vec![Dash,Dash,Dash]),
        'p' => Ok(vec![Dot,Dash,Dash,Dot]),
        'q' => Ok(vec![Dash,Dash,Dot,Dash]),
        'r' => Ok(vec![Dot,Dash,Dot]),
        's' => Ok(vec![Dot,Dot,Dot]),
        't' => Ok(vec![Dash]),
        'u' => Ok(vec![Dot,Dot,Dash]),
        'v' => Ok(vec![Dot,Dot,Dot,Dash]),
        'w' => Ok(vec![Dot,Dash,Dash]),
        'x' => Ok(vec![Dash,Dot,Dot,Dash]),
        'y' => Ok(vec![Dash,Dot,Dash,Dash]),
        'z' => Ok(vec![Dash,Dash,Dot,Dot]),
        'ü' => Ok(vec![Dot,Dot,Dash,Dash]),
        'ä' => Ok(vec![Dot,Dash,Dot,Dash]),
        'ö' => Ok(vec![Dash,Dash,Dash,Dot]),
        '1' => Ok(vec![Dot,Dash,Dash,Dash,Dash]),
        '2' => Ok(vec![Dot,Dot,Dash,Dash,Dash]),
        '3' => Ok(vec![Dot,Dot,Dot,Dash,Dash]),
        '4' => Ok(vec![Dot,Dot,Dot,Dot,Dash]),
        '5' => Ok(vec![Dot,Dot,Dot,Dot,Dot]),
        '6' => Ok(vec![Dash,Dot,Dot,Dot,Dot]),
        '7' => Ok(vec![Dash,Dash,Dot,Dot,Dot]),
        '8' => Ok(vec![Dash,Dash,Dash,Dot,Dot]),
        '9' => Ok(vec![Dash,Dash,Dash,Dash,Dot]),
        '0' => Ok(vec![Dash,Dash,Dash,Dash,Dash]),
        'é' => Ok(vec![Dot,Dot,Dash,Dot,Dot]),
        'è' => Ok(vec![Dot,Dash,Dot,Dot,Dash]),
        'à' => Ok(vec![Dot,Dash,Dash,Dot,Dash]),
        '+' => Ok(vec![Dot,Dash,Dot,Dash,Dot]),
        '=' => Ok(vec![Dash,Dot,Dot,Dot,Dash]),
        '/' => Ok(vec![Dash,Dot,Dot,Dash,Dot]),
        'ñ' => Ok(vec![Dash,Dash,Dot,Dash,Dash]),
        '?' => Ok(vec![Dot,Dot,Dash,Dash,Dot,Dot]),
        '_' => Ok(vec![Dot,Dot,Dash,Dash,Dot,Dash]),
        '"' => Ok(vec![Dot,Dash,Dot,Dot,Dash,Dot]),
        '.' => Ok(vec![Dot,Dash,Dot,Dash,Dot,Dash]),
        '@' => Ok(vec![Dot,Dash,Dash,Dot,Dash,Dot]),
        '\'' => Ok(vec![Dot,Dash,Dash,Dash,Dash,Dot]),
        '-' => Ok(vec![Dash,Dot,Dot,Dot,Dot,Dash]),
        ';' => Ok(vec![Dash,Dot,Dash,Dot,Dash,Dot]),
        '!' => Ok(vec![Dash,Dot,Dash,Dot,Dash,Dash]),
        '('|')' => Ok(vec![Dash,Dot,Dash,Dash,Dot,Dash]),
        ',' => Ok(vec![Dash,Dash,Dot,Dot,Dash,Dash]),
        ':' => Ok(vec![Dash,Dash,Dash,Dot,Dot,Dot]),
        c => Err(format!("invalid character '{}' at position {}", c, p))
    }
}