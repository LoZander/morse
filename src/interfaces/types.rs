#[derive(Clone)]
pub enum Sym {Dash,Dot}
pub type Char = Vec<Sym>;
pub type Word = Vec<Char>;
pub type Sen = Vec<Word>;

pub struct Pos(pub usize, pub usize);
impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(word {}, char {})", self.0 + 1, self.1 + 1)
    }
}

pub type MorseResult<T> = Result<T,String>;

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
    let sen_string = s.iter()
                              .map(|x| format!("{} / ",string_of_word(x)))
                              .collect::<String>();
    sen_string.strip_suffix(" / ")
              .map(str::to_string)
              .unwrap_or(sen_string)
}