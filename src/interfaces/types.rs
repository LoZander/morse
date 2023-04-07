#[derive(Clone)]
#[derive(Hash)]
#[derive(PartialEq,Eq)]
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
        write!(f,"{self}")
    }
}

#[derive(PartialEq, Eq)]
#[derive(Hash)]
#[derive(Clone)]
#[derive(Debug)]
pub struct Char(Vec<Sym>);

impl Char {
    pub fn iter(&self) -> std::slice::Iter<Sym> {
        self.0.iter()
    }
}

impl From<Vec<Sym>> for Char {
    fn from(value: Vec<Sym>) -> Self {
        Char(value)
    }
}

impl<const N: usize> From<[Sym; N]> for Char {
    fn from(value: [Sym; N]) -> Self {
        Char(value.to_vec())
    }
}

impl IntoIterator for Char {
    type Item = Sym;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Sym> for Char {
    fn from_iter<T: IntoIterator<Item = Sym>>(iter: T) -> Self {
        Char(iter.into_iter().collect())
    }
}

impl std::fmt::Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.iter().fold(String::new(), |acc, s| acc + &s.to_string()))
    }
}

pub struct Word(Vec<Char>);

impl Word {
    pub fn iter(&self) -> std::slice::Iter<Char> {
        self.0.iter()
    }
}

impl From<Vec<Char>> for Word {
    fn from(value: Vec<Char>) -> Self {
        Word(value)
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.iter()
                            .map(Char::to_string)
                            .reduce(|acc, c| format!("{} {}", acc, c.to_string())).unwrap_or_default())
    }
}

/* impl IntoIterator for Word {
    type Item = Char;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
} */

impl FromIterator<Char> for Word {
    fn from_iter<T: IntoIterator<Item = Char>>(iter: T) -> Self {
        Word(iter.into_iter().collect())
    }
}

pub struct Sen(Vec<Word>);

impl Sen {
    pub fn iter(&self) -> std::slice::Iter<Word> {
        self.0.iter()
    }
}

impl From<Vec<Word>> for Sen {
    fn from(value: Vec<Word>) -> Self {
        Sen(value)
    }
}

/* impl IntoIterator for Sen {
    type Item = Word;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
} */

impl FromIterator<Word> for Sen {
    fn from_iter<T: IntoIterator<Item = Word>>(iter: T) -> Self {
        Sen(iter.into_iter().collect())
    }
}

impl std::fmt::Display for Sen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.iter()
                            .map(Word::to_string)
                            .reduce(|acc, w| format!("{acc} / {w}")).unwrap_or_default())
    }
}

pub struct Pos(pub usize, pub usize);
impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(word {}, char {})", self.0 + 1, self.1 + 1)
    }
}

pub type MorseResult<T> = Result<T,String>;
