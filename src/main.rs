use std::env;
use morse::{MorseApp, MorseEncoder, MorseDecoder};
use eframe::{egui};

mod types;
mod morse;
mod parse;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Morse", options, Box::new(|_cc| Box::new(MorseApp)));
}

impl eframe::App for MorseApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hellow world!");
        });
    }
}