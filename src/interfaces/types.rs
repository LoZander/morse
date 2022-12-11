#[derive(Clone)]
pub enum Sym {Dash,Dot}
pub type Char = Vec<Sym>;
pub type Word = Vec<MorseResult<Char>>;
pub type Sen = Vec<Word>;

pub struct Log<T,E: Clone> 
{
    pub value: T,
    pub errors: Vec<E>
}
impl<A, E: Clone> Log<A,E> {
    pub fn bind<B,F>(self, f: F) -> Log<B,E>
    where F: FnOnce(A) -> Log<B,E> 
    {
        let log = f(self.value);

        let mut errs = self.errors;
        errs.append(&mut log.errors.to_owned());
        Log{
            errors: errs,
            ..log
        }
    }

    pub fn map<F,B>(self, f: F) -> Log<B,E>
    where F: FnOnce(A) -> B
    {
        self.bind(|x| Log::ret(f(x)))
    }
    pub fn ret(value: A) -> Log<A,E> {
        Log{value, errors: Vec::new()}
    }
    pub fn from_result(result: Result<A,E>, default: A) -> Self {
        match result {
            Err(e) => Log{value: default, errors: vec![e]},
            Ok(value) => Log{value, errors: Vec::new()}
        }
    }

    pub fn is_error(&self) -> bool {
        !self.errors.is_empty()
    }

}

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


pub fn string_of_char(c: &MorseResult<Char>) -> Log<String,String> {
    match c {
        Err(e) => Log{
            value: String::from("#"),
            errors: vec![e.to_owned()]},
        Ok(v) => Log::ret(v.iter().map(|x| format!("{}", x)).collect())
    }
    /* c.iter()
     .map(|x| format!("{}",x))
     .collect::<String>() */
}

pub fn string_of_word(w: &Word) -> Log<String,String> {
    w.iter()
     .map(|x| string_of_char(x).map(|y| y + " "))
     .fold(
        Log::ret(String::new()), 
        |acc,x|     
            x.bind(|y|
            acc.bind(|z|
            Log::ret(z + y.as_str())))
        )
     .map(|x| x.trim().to_string())
}

pub fn string_of_sen(s: &Sen) -> Log<String,String> {
    let sen_string = s.iter()
                              .map(|x| string_of_word(x).map(|y| format!("{} / ", y)))
                              .fold(
                                Log::ret(String::new()),
                                |acc, element|
                                acc.bind(|x|
                                element.bind(|y|
                                Log::ret(x + y.as_str())))
                              );
    sen_string.map(|x| x.strip_suffix(" / ")
                                .map(str::to_string)
                                .unwrap_or(x)
    )
}