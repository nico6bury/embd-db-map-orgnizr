use fltk::{app::App, window::Window, prelude::{WidgetExt, GroupExt}, enums::Color};


/// # GUI
/// This struct holds all the gui structs pulled from fltk.  
/// It also has methods to be used for managing the gui.
pub struct GUI {
	/// application that gui runs within
	pub app:App,
	/// main window of application
	pub main_window: Window,
}

impl Default for GUI {
    fn default() -> Self {
        Self { app: Default::default(), main_window: Default::default() }
    }//end default()
}//end impl Default for GUI

impl GUI {
	pub fn initialize(&mut self) {
		// window settings
		self.main_window.set_size(get_default_win_width(),get_default_win_height());
		self.main_window.make_resizable(true);
		self.main_window.set_label("Embedded DB Map Organizer");
		self.main_window.set_label_size(32);
		self.main_window.set_label_color(Color::Green);
		self.main_window.set_color(Color::from_rgb(101, 68, 24));
	}//end initialize(&mut self)

	pub fn show(&mut self) {
		self.main_window.show();
	}//end show(&mut self)
}//end impl for GUI

pub fn get_default_win_width() -> i32 {1000}
pub fn get_default_win_height() -> i32 {700}