use std::env;
use eframe::egui;
use morse::{MorseApp, MorseEncoder, MorseDecoder};

mod types;
mod morse;
mod parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return
    }
    match args[1].as_str() {
        "encode" => println!("{}",MorseApp::encode(args[2].clone())),
        "decode" => println!("{}",MorseApp::decode(args[2].clone())),
        s => println!("{} is not a valid command. Try \"encode <plaintext>\" or \"decode <ciphertext>\"", s)
    }
}

impl eframe::App for MorseApp {
    
}
