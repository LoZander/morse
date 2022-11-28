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
    c.into_iter().map(|x| format!("{}",x)).collect()
}

#[allow(dead_code)]
pub fn string_of_word(w: &Word) -> String {
    let string: String = w.into_iter().map(|x| format!("{} ",string_of_char(x))).collect();
    string.trim().to_string()
}

#[allow(dead_code)]
pub fn string_of_sen(s: &Sen) -> String {
    let string = s.into_iter()
                          .map(|x| format!("{} / ",string_of_word(x)))
                          .collect::<String>();
    string.as_str()[0..string.len() - 3]
          .to_string()
}