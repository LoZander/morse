#![windows_subsystem = "windows"]
use morse::graphics::gui::{Gui, GuiApp};

fn main() {
    let gui = GuiApp::default();
    gui.run()
}