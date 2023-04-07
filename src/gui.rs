use eframe::{egui, epaint::vec2, HardwareAcceleration, Renderer, Theme};
use crate::morse::{SymbolMap, self};

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
            initial_window_size: Some(vec2(300.,300.)),
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
        let symbols = SymbolMap::default();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Encoding");
            ui.vertical(|ui| {
                ui.text_edit_multiline(&mut self.enc_string);
                ui.label("Output:");
                let x = match morse::encode(self.enc_string.clone(), &symbols) {
                    Ok(str) => str,
                    Err(str) => str
                };
                ui.label(x);
            });
            ui.heading("Decoding");
            ui.vertical(|ui| {
                ui.text_edit_multiline(&mut self.dec_string);
                ui.label("Output:");
                let x = match morse::decode(self.dec_string.clone(), &symbols) {
                    Ok(str) => str,
                    Err(str) => str
                };
                ui.label(x);
            });
        });
    }
}