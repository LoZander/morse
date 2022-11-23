use morse::Sen;
use morse::translate;
use parse::parse;




mod morse;
mod parse;

fn main() {
    let cipher: Sen = parse(String::from("... --- ... / ... --- ...")).unwrap();
    let plain = translate(cipher).unwrap();
    println!("Plaintext: {}",plain)
}
