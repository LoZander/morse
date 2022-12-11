#![windows_subsystem = "windows"]

use standard::gui::{GuiApp, Gui};

pub mod standard;

#[cfg(test)]
mod tests;

fn main() {
    let gui = GuiApp::default();
    gui.run();
}