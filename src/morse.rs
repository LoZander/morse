use crate::{types::{Sym::{Dash,Dot}, Sen, Word, Char, self, string_of_sen}, parse};

/**
 * Decodes a morse code sentence, where '/' is the word separator
 */
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
        []                          => String::from(""),
        _                           => format!("invalid morse sequence [{}]",types::string_of_char(&char))
    }
}

/**
 * Encodes a message to morse code. Any unrecognized symbols are simply thrown away.
 */
pub fn encode(plaintext: String) -> String {
    let sen: Sen = plaintext.to_lowercase()
                    .split(" ")
                    .into_iter()
                    .map(str::to_string)
                    .map(encode_word)
                    .collect();
    format!("{}", string_of_sen(&sen))
}

fn encode_word(plaintext: String) -> Word {
    plaintext.chars()
     .into_iter()
     .map(encode_char)
     .collect()
}

fn encode_char(c: char) -> Char {
    match c {
        'a' => vec![Dot,Dash],
        'b' => vec![Dash,Dot,Dot,Dot],
        'c' => vec![Dash,Dot,Dash,Dot],
        'd' => vec![Dash,Dot,Dot],
        'e' => vec![Dot],
        'f' => vec![Dot,Dot,Dash,Dot],
        'g' => vec![Dash,Dash,Dot],
        'h' => vec![Dot,Dot,Dot,Dot],
        'i' => vec![Dot,Dot],
        'j' => vec![Dot,Dash,Dash,Dash],
        'k' => vec![Dash,Dot,Dash],
        'l' => vec![Dot,Dash,Dot,Dot],
        'm' => vec![Dash,Dash],
        'n' => vec![Dash,Dot],
        'o' => vec![Dash,Dash,Dash],
        'p' => vec![Dot,Dash,Dash,Dot],
        'q' => vec![Dash,Dash,Dot,Dash],
        'r' => vec![Dot,Dash,Dot],
        's' => vec![Dot,Dot,Dot],
        't' => vec![Dash],
        'u' => vec![Dot,Dot,Dash],
        'v' => vec![Dot,Dot,Dot,Dash],
        'w' => vec![Dot,Dash,Dash],
        'x' => vec![Dash,Dot,Dot,Dash],
        'y' => vec![Dash,Dot,Dash,Dash],
        'z' => vec![Dash,Dash,Dot,Dot],
        '1' => vec![Dot,Dash,Dash,Dash,Dash],
        '2' => vec![Dot,Dot,Dash,Dash,Dash],
        '3' => vec![Dot,Dot,Dot,Dash,Dash],
        '4' => vec![Dot,Dot,Dot,Dot,Dash],
        '5' => vec![Dot,Dot,Dot,Dot,Dot],
        '6' => vec![Dash,Dot,Dot,Dot,Dot],
        '7' => vec![Dash,Dash,Dot,Dot,Dot],
        '8' => vec![Dash,Dash,Dash,Dot,Dot],
        '9' => vec![Dash,Dash,Dash,Dash,Dot],
        '0' => vec![Dash,Dash,Dash,Dash,Dash],
        _ => vec![]
    }
}