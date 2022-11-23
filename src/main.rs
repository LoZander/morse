use morse::Sen;
use morse::decode;
use parse::parse;




mod morse;
mod parse;

fn main() {
    let cipher: Sen = parse(String::from("... --- ... / ... --- ...")).unwrap();
    let plain = decode(cipher).unwrap();
    println!("Plaintext: {}",plain)
}
