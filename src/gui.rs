use eframe::{egui, epaint::vec2, HardwareAcceleration, Renderer, Theme};
use thiserror::Error;
use crate::morse::{SymbolMap, self};

#[derive(Error, Debug)]
pub enum GuiError {
    #[error(transparent)]
    Eframe(#[from] eframe::Error)
}

pub type GuiResult = Result<(), GuiError>;

pub trait Gui {
    fn run(self) -> GuiResult;
}
#[derive(Default)]
pub struct GuiApp {
    enc_string: String,
    dec_string: String,
}

impl Gui for GuiApp {
    fn run(self) -> GuiResult {
        let viewport = egui::ViewportBuilder::default()
            .with_title("Morse")
            .with_inner_size(vec2(300.,300.))
        ;

        let options = eframe::NativeOptions{
            viewport,
            vsync: true,
            multisampling: 0,
            depth_buffer: 0,
            stencil_buffer: 0,
            hardware_acceleration: HardwareAcceleration::Preferred,
            renderer: Renderer::default(),
            follow_system_theme: cfg!(target_os = "macos") || cfg!(target_os = "windows"),
            default_theme: Theme::Dark,
            run_and_return: true,
            event_loop_builder: None,
            window_builder: None,
            shader_version: None,
            centered: true,
            persist_window: false
        };
        eframe::run_native("Morse", options, Box::new(|_cc| Box::new(self)))?;
        Ok(())
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
