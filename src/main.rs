use std::env;

use morse::{decode, encode};

mod types;
mod morse;
mod parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return
    }
    match args[1].as_str() {
        "encode" => println!("{}",encode(args[2].clone())),
        "decode" => println!("{}",decode(args[2].clone())),
        s => println!("{} is not a valid command. Try \"encode <plaintext>\" or \"decode <ciphertext>\"", s)
    }
}
