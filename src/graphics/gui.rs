use eframe::{egui::{self, Ui}, epaint::vec2, HardwareAcceleration, Renderer, Theme};
use crate::{standard::morse, interfaces::types::MorseResult};

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
        let options = eframe::NativeOptions{
            always_on_top: false,
            maximized: false,
            decorated: true,
            fullscreen: false,
            drag_and_drop_support: true,
            icon_data: None,
            initial_window_pos: None,
            initial_window_size: Some(vec2(300.,400.)),
            min_window_size: None,
            max_window_size: None,
            resizable: true,
            transparent: false,
            vsync: true,
            multisampling: 0,
            depth_buffer: 0,
            stencil_buffer: 0,
            hardware_acceleration: HardwareAcceleration::Preferred,
            renderer: Renderer::default(),
            follow_system_theme: cfg!(target_os = "macos") || cfg!(target_os = "windows"),
            default_theme: Theme::Dark,
            run_and_return: true,
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
    F: Fn(String) -> MorseResult<String>,
{
    ui.heading(title);
    ui.vertical(|ui| {
        ui.text_edit_multiline(data);
        ui.label("Output:");
        let res = match f(data.to_owned()) {
            Ok(str) => str,
            Err(str) => str
        };
        ui.text_edit_multiline(&mut res.to_owned());
    });
}