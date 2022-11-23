use Sym::*;
pub enum Sym {Dash,Dot}
impl std::fmt::Display for Sym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dash => write!(f,"-"),
            Self::Dot => write!(f,".")
        }
    }
}

impl std::fmt::Debug for Sym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self)
    }
}

pub type Char = Vec<Sym>;

pub fn string_of_char(c: &Char) -> String {
    c.into_iter().map(|x| format!("{}",x)).collect()
}

#[allow(dead_code)]
pub fn string_of_word(w: &Word) -> String {
    let string: String = w.into_iter().map(|x| format!("{} ",string_of_char(x))).collect();
    string.trim().to_string()
}

#[allow(dead_code)]
pub fn string_of_sen(s: &Sen) -> String {
    s.into_iter().map(|x| format!("{} / ",string_of_word(x))).collect()
}

pub type Word = Vec<Char>;
pub type Sen = Vec<Word>;


pub fn decode(s: Sen) -> Result<String,String> {
    s.into_iter()
     .try_fold(String::new(),|acc, x| Ok(acc + decode_word(x)?.as_str() + " "))
     .map(|x| x.trim_end().to_string())
}

fn decode_word(w: Word) -> Result<String,String> {
    w.into_iter()
     .try_fold(String::new(),|acc,x| Ok(acc + decode_character(x)?))
}

fn decode_character(c: Char) -> Result<&'static str,String> {
    match c[..] {
        [Dot,Dash]                  => Ok("a"),
        [Dash,Dot,Dot,Dot]          => Ok("b"),
        [Dash,Dot,Dash,Dot]         => Ok("c"),
        [Dash,Dot,Dot]              => Ok("d"),
        [Dot]                       => Ok("e"),
        [Dot,Dot,Dash,Dot]          => Ok("f"),
        [Dash,Dash,Dot]             => Ok("g"),
        [Dot,Dot,Dot,Dot]           => Ok("h"),
        [Dot,Dot]                   => Ok("i"),
        [Dot,Dash,Dash,Dash]        => Ok("j"),
        [Dash,Dot,Dash]             => Ok("k"),
        [Dot,Dash,Dot,Dot]          => Ok("l"),
        [Dash,Dash]                 => Ok("m"),
        [Dash,Dot]                  => Ok("n"),
        [Dash,Dash,Dash]            => Ok("o"),
        [Dot,Dash,Dash,Dot]         => Ok("p"),
        [Dash,Dash,Dot,Dash]        => Ok("q"),
        [Dot,Dash,Dot]              => Ok("r"),
        [Dot,Dot,Dot]               => Ok("s"),
        [Dash]                      => Ok("t"),
        [Dot,Dot,Dash]              => Ok("u"),
        [Dot,Dot,Dot,Dash]          => Ok("v"),
        [Dot,Dash,Dash]             => Ok("w"),
        [Dash,Dot,Dot,Dash]         => Ok("x"),
        [Dash,Dot,Dash,Dash]        => Ok("y"),
        [Dash,Dash,Dot,Dot]         => Ok("z"),
        [Dot,Dash,Dash,Dash,Dash]   => Ok("1"),
        [Dot,Dot,Dash,Dash,Dash]    => Ok("2"),
        [Dot,Dot,Dot,Dash,Dash]     => Ok("3"),
        [Dot,Dot,Dot,Dot,Dash]      => Ok("4"),
        [Dot,Dot,Dot,Dot,Dot]       => Ok("5"),
        [Dash,Dot,Dot,Dot,Dot]      => Ok("6"),
        [Dash,Dash,Dot,Dot,Dot]     => Ok("7"),
        [Dash,Dash,Dash,Dot,Dot]    => Ok("8"),
        [Dash,Dash,Dash,Dash,Dot]   => Ok("9"),
        [Dash,Dash,Dash,Dash,Dash]  => Ok("0"),
        _ => Err(format!("invalid morse sequence [{}]",string_of_char(&c)))
    }
}