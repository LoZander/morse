use eframe::{egui::{self, Ui}, epaint::vec2, NativeOptions};
use crate::{standard::morse, interfaces::types::{MorseResult, Log}};

pub trait Gui {
    fn run(self);
}
#[derive(Default)]
pub struct GuiApp {
    enc_string: String,
    dec_string: String,
}

impl Gui for GuiApp {
    fn run(self) {
        let options = NativeOptions {
            initial_window_size: Some(vec2(300.,400.)),
            ..NativeOptions::default()
        };
        eframe::run_native("Morse", options, Box::new(|_cc| Box::new(self)));
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            add_encode_block(&mut self.enc_string, ui);
            add_decode_block(&mut self.dec_string, ui);
        });
    }
}

fn add_encode_block(data: &mut String, ui: &mut Ui) {
    add_block("Encoding", data, ui, morse::encode)
}

fn add_decode_block(data: &mut String, ui: &mut Ui) {
    add_block("Decoding", data, ui, morse::decode)
}

fn add_block<F>(title: &str, data: &mut String, ui: &mut Ui, f: F) 
where 
    F: Fn(String) -> Log<String,String>,
{
    ui.heading(title);
    ui.vertical(|ui| {
        ui.text_edit_multiline(data);
        ui.label("Output:");
        let res = match f(data.to_owned()) {
            Log{value, errors} => format!("{}\n{}",value,errors.join("\n"))
        };
        ui.text_edit_multiline(&mut res.to_owned());
    });
}