use crate::gui::GUI;

mod gui;

fn main() {
    let mut gui = GUI::default();
    gui.initialize();

    gui.show();
    while gui.app.wait() {
        // do whatever
    }
}//end main fn
