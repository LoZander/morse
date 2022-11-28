use morse::{decode, encode};
use parse::parse;

mod types;
mod morse;
mod parse;

fn main() {
    let plain = String::from("The quick brown fox jumps over the lazy dog, 1234567890. Certain character are not recognized and are simply thrown away, such as some of \"\'()[]{}.,:;/!?+-");
    println!("Plaintext: {}",plain);
    let encoded = encode(plain);
    println!("Encoded: {}", encoded);
    let decoded = decode(encoded);
    println!("Decoded: {}",decoded)
}
