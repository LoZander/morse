use crate::interfaces::types::Log;
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

pub fn decode(cipher: String) -> Log<String,String> {
    let sen: Sen = parse::parse(cipher);
    sen.into_iter()
       .enumerate()
       .map(|(i,w)| decode_word(i,w))
       .fold(Log::ret(String::new()),
            |acc,element|
                acc.bind(|x|
                element.bind(|y|
                Log::ret(format!("{} {}", x, y)))))
       .map(|x| x.trim().to_string())
       /* .map(|x| x.map(|x| x + " "))
       .collect::<MorseResult<String>>()
       .map(|s| s.trim().to_string()) */
}

fn decode_word(index: usize, word: Word) -> Log<String,String> {
    word.into_iter()
        .map(|x|Log::from_result(x, Vec::new()))
        .enumerate()
        .map(|(j,c)| 
            c.bind(|x| decode_character(Pos(index,j), x)))
        .fold(Log::ret(String::new()),
            |acc,element| 
                acc.bind(|x|
                element.bind(|y|
                Log::ret(x + y.as_str()))))
}

fn decode_character(p: Pos, char: Char) -> Log<String,String> {
    match char[..] {
        [Dot,Dash]                      => Log::ret("a"),
        [Dash,Dot,Dot,Dot]              => Log::ret("b"),
        [Dash,Dot,Dash,Dot]             => Log::ret("c"),
        [Dash,Dot,Dot]                  => Log::ret("d"),
        [Dot]                           => Log::ret("e"),
        [Dot,Dot,Dash,Dot]              => Log::ret("f"),
        [Dash,Dash,Dot]                 => Log::ret("g"),
        [Dot,Dot,Dot,Dot]               => Log::ret("h"),
        [Dot,Dot]                       => Log::ret("i"),
        [Dot,Dash,Dash,Dash]            => Log::ret("j"),
        [Dash,Dot,Dash]                 => Log::ret("k"),
        [Dot,Dash,Dot,Dot]              => Log::ret("l"),
        [Dash,Dash]                     => Log::ret("m"),
        [Dash,Dot]                      => Log::ret("n"),
        [Dash,Dash,Dash]                => Log::ret("o"),
        [Dot,Dash,Dash,Dot]             => Log::ret("p"),
        [Dash,Dash,Dot,Dash]            => Log::ret("q"),
        [Dot,Dash,Dot]                  => Log::ret("r"),
        [Dot,Dot,Dot]                   => Log::ret("s"),
        [Dash]                          => Log::ret("t"),
        [Dot,Dot,Dash]                  => Log::ret("u"),
        [Dot,Dot,Dot,Dash]              => Log::ret("v"),
        [Dot,Dash,Dash]                 => Log::ret("w"),
        [Dash,Dot,Dot,Dash]             => Log::ret("x"),
        [Dash,Dot,Dash,Dash]            => Log::ret("y"),
        [Dash,Dash,Dot,Dot]             => Log::ret("z"),
        [Dot,Dot,Dash,Dash]             => Log::ret("ü"),
        [Dot,Dash,Dot,Dash]             => Log::ret("ö"),
        [Dot,Dash,Dash,Dash,Dash]       => Log::ret("1"),
        [Dot,Dot,Dash,Dash,Dash]        => Log::ret("2"),
        [Dot,Dot,Dot,Dash,Dash]         => Log::ret("3"),
        [Dot,Dot,Dot,Dot,Dash]          => Log::ret("4"),
        [Dot,Dot,Dot,Dot,Dot]           => Log::ret("5"),
        [Dash,Dot,Dot,Dot,Dot]          => Log::ret("6"),
        [Dash,Dash,Dot,Dot,Dot]         => Log::ret("7"),
        [Dash,Dash,Dash,Dot,Dot]        => Log::ret("8"),
        [Dash,Dash,Dash,Dash,Dot]       => Log::ret("9"),
        [Dash,Dash,Dash,Dash,Dash]      => Log::ret("0"),
        [Dot,Dot,Dash,Dot,Dot]          => Log::ret("é"),
        [Dot,Dash,Dot,Dot,Dash]         => Log::ret("è"),
        [Dot,Dash,Dash,Dot,Dash]        => Log::ret("à"),
        [Dot,Dash,Dot,Dash,Dot]         => Log::ret("+"),
        [Dash,Dot,Dot,Dot,Dash]         => Log::ret("="),
        [Dash,Dot,Dot,Dash,Dot]         => Log::ret("/"),
        [Dash,Dash,Dot,Dash,Dash]       => Log::ret("ñ"),
        [Dot,Dot,Dash,Dash,Dot,Dot]     => Log::ret("?"),
        [Dot,Dot,Dash,Dash,Dot,Dash]    => Log::ret("_"),
        [Dot,Dash,Dot,Dot,Dash,Dot]     => Log::ret("\""),
        [Dot,Dash,Dot,Dash,Dot,Dash]    => Log::ret("."),
        [Dot,Dash,Dash,Dot,Dash,Dot]    => Log::ret("@"),
        [Dot,Dash,Dash,Dash,Dash,Dot]   => Log::ret("\\"),
        [Dash,Dot,Dot,Dot,Dot,Dash]     => Log::ret("-"),
        [Dash,Dot,Dash,Dot,Dash,Dot]    => Log::ret(";"),
        [Dash,Dot,Dash,Dot,Dash,Dash]   => Log::ret("!"),
        [Dash,Dot,Dash,Dash,Dot,Dash]   => Log::ret("|"),
        [Dash,Dash,Dot,Dot,Dash,Dash]   => Log::ret(","),
        [Dash,Dash,Dash,Dot,Dot,Dot]    => Log::ret(":"),
        []                              => Log::ret(""),
        _                               => Log{
                                            value: "#",
                                            errors: vec![format!("invalid morse sequence [{}] at position {}", string_of_char(&Ok(char) ).value, p)]
                                        }
    }.map(str::to_string)
}

pub fn encode(plaintext: String) -> Log<String,String> {
        let sen: Sen = plaintext.to_lowercase()
                        .split_whitespace()
                        .enumerate()
                        .map(|(i,s)| (i,s.to_string()))
                        .map(|(i,s)| encode_word(i,s))
                        .collect();
        string_of_sen(&sen)
}

fn encode_word(word_number: usize, plaintext: String) -> Word {
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