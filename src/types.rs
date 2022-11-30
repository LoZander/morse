#[derive(Clone)]
pub enum Sym {Dash,Dot}
pub type Char = Vec<Sym>;
pub type Word = Vec<Char>;
pub type Sen = Vec<Word>;

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


pub fn string_of_char(c: &Char) -> String {
    c.iter()
     .map(|x| format!("{}",x))
     .collect::<String>()
}

pub fn string_of_word(w: &Word) -> String {
    w.iter()
     .map(|x: &Char| format!("{} ",string_of_char(x)))
     .collect::<String>()
     .trim()
     .to_string()
}

pub fn string_of_sen(s: &Sen) -> String {
    let string = s.iter()
                          .map(|x| format!("{} / ",string_of_word(x)))
                          .collect::<String>();
    string[0..string.len() - 3].to_string()
}