#![windows_subsystem = "windows"]
use morse::gui::{Gui, GuiApp};

fn main() {
    let gui = GuiApp::default();
    gui.run().unwrap()
}
