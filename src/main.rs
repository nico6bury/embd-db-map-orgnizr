use crate::gui::GUI;

mod gui;

fn main() {
    let mut gui = GUI::default();
    gui.initialize();

    gui.show();
}//end main fn
