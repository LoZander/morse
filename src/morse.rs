use crate::{types::{Sym::{Dash,Dot}, Sen, Word, Char, string_of_sen, string_of_char, MorseResult}, parse};

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

pub fn decode(cipher: String) -> String {
    let sen: Sen = parse::parse(cipher);
    sen.into_iter()
       .map(decode_word)
       .map(|x| x + " ")
       .collect()
}

fn decode_word(word: Word) -> String {
    word.into_iter()
        .map(decode_character)
        .collect()
}

fn decode_character(char: Char) -> String {
    match char[..] {
        [Dot,Dash]                  => String::from("a"),
        [Dash,Dot,Dot,Dot]          => String::from("b"),
        [Dash,Dot,Dash,Dot]         => String::from("c"),
        [Dash,Dot,Dot]              => String::from("d"),
        [Dot]                       => String::from("e"),
        [Dot,Dot,Dash,Dot]          => String::from("f"),
        [Dash,Dash,Dot]             => String::from("g"),
        [Dot,Dot,Dot,Dot]           => String::from("h"),
        [Dot,Dot]                   => String::from("i"),
        [Dot,Dash,Dash,Dash]        => String::from("j"),
        [Dash,Dot,Dash]             => String::from("k"),
        [Dot,Dash,Dot,Dot]          => String::from("l"),
        [Dash,Dash]                 => String::from("m"),
        [Dash,Dot]                  => String::from("n"),
        [Dash,Dash,Dash]            => String::from("o"),
        [Dot,Dash,Dash,Dot]         => String::from("p"),
        [Dash,Dash,Dot,Dash]        => String::from("q"),
        [Dot,Dash,Dot]              => String::from("r"),
        [Dot,Dot,Dot]               => String::from("s"),
        [Dash]                      => String::from("t"),
        [Dot,Dot,Dash]              => String::from("u"),
        [Dot,Dot,Dot,Dash]          => String::from("v"),
        [Dot,Dash,Dash]             => String::from("w"),
        [Dash,Dot,Dot,Dash]         => String::from("x"),
        [Dash,Dot,Dash,Dash]        => String::from("y"),
        [Dash,Dash,Dot,Dot]         => String::from("z"),
        [Dot,Dot,Dash,Dash]         => String::from("ü"),
        [Dot,Dash,Dot,Dash]         => String::from("ö"),
        [Dot,Dash,Dash,Dash,Dash]   => String::from("1"),
        [Dot,Dot,Dash,Dash,Dash]    => String::from("2"),
        [Dot,Dot,Dot,Dash,Dash]     => String::from("3"),
        [Dot,Dot,Dot,Dot,Dash]      => String::from("4"),
        [Dot,Dot,Dot,Dot,Dot]       => String::from("5"),
        [Dash,Dot,Dot,Dot,Dot]      => String::from("6"),
        [Dash,Dash,Dot,Dot,Dot]     => String::from("7"),
        [Dash,Dash,Dash,Dot,Dot]    => String::from("8"),
        [Dash,Dash,Dash,Dash,Dot]   => String::from("9"),
        [Dash,Dash,Dash,Dash,Dash]  => String::from("0"),
        [Dot,Dot,Dash,Dot,Dot]      => String::from("é"),
        [Dot,Dash,Dot,Dot,Dash]     => String::from("è"),
        [Dot,Dash,Dash,Dot,Dash]    => String::from("à"),
        [Dot,Dash,Dot,Dash,Dot]     => String::from("+"),
        [Dash,Dot,Dot,Dot,Dash]     => String::from("="),
        [Dash,Dot,Dot,Dash,Dot]     => String::from("/"),
        [Dash,Dash,Dot,Dash,Dash]   => String::from("ñ"),
        [Dot,Dot,Dash,Dash,Dot,Dot] => String::from("?"),
        [Dot,Dot,Dash,Dash,Dot,Dash]=> String::from("_"),
        [Dot,Dash,Dot,Dot,Dash,Dot] => String::from("\""),
        [Dot,Dash,Dot,Dash,Dot,Dash] => String::from("."),
        [Dot,Dash,Dash,Dot,Dash,Dot]=> String::from("@"),
        [Dot,Dash,Dash,Dash,Dash,Dot] => String::from("\\"),
        [Dash,Dot,Dot,Dot,Dot,Dash] => String::from("-"),
        [Dash,Dot,Dash,Dot,Dash,Dot]=> String::from(";"),
        [Dash,Dot,Dash,Dot,Dash,Dash] => String::from("!"),
        [Dash,Dot,Dash,Dash,Dot,Dash] => String::from("|"),
        [Dash,Dash,Dot,Dot,Dash,Dash] => String::from(","),
        [Dash,Dash,Dash,Dot,Dot,Dot] => String::from(":"),
        _                           => format!("invalid morse sequence [{}]",string_of_char(&char))
    }
}

pub fn encode(plaintext: String) -> MorseResult<String> {
        let sen: Sen = plaintext.to_lowercase()
                        .split_whitespace()
                        .into_iter()
                        .map(str::to_string)
                        .map(encode_word)
                        .collect::<MorseResult<_>>()?;
        Ok(string_of_sen(&sen))
}

fn encode_word(plaintext: String) -> MorseResult<Word> {
    plaintext.char_indices()
     .into_iter()
     .map(encode_char)
     .collect()
}

fn encode_char((i,c): (usize,char)) -> MorseResult<Char> {
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
        c => Err(format!("{} at position {} is not a valid character", c, i))
    }
}