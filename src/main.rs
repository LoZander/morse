use gui::{GuiApp, Gui};

mod types;
mod morse;
mod parse;
mod gui;

fn main() {
    let gui = GuiApp::default();
    gui.run();
}