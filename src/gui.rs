use fltk::{app::App, window::Window, prelude::{WidgetExt, GroupExt, WidgetBase}, enums::Color, group::{Tabs, Group}};
use fltk_theme::{WidgetScheme, SchemeType};


/// # GUI
/// This struct holds all the gui structs pulled from fltk.  
/// It also has methods to be used for managing the gui.
pub struct GUI {
	/// application that gui runs within
	pub app:App,
	/// main window of application
	pub main_window: Window,
	/// Outer vertical tab group
	pub outer_tab_group: Tabs,
	/// part of outer tab group with stuff for searching for maps
	pub outer_tab_organizer: Group,
	/// part of outer tab group with stuff for adding maps to db
	pub outer_tab_map_input: Group,

}

impl Default for GUI {
    fn default() -> Self {
        Self {
			app: Default::default(), 
			main_window: Default::default(),
			outer_tab_group: Default::default(),
			outer_tab_organizer: Default::default(),
			outer_tab_map_input: Default::default(),
		}//end Self constructor
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

		// outer tab settings
		self.outer_tab_group = Tabs::new(0, 0, get_default_win_width(), get_default_win_height(), None);
		self.outer_tab_group.auto_layout();
		self.outer_tab_group.end();
		self.main_window.add(&self.outer_tab_group);

		// outer tab organizer settings
		self.outer_tab_organizer = Group::new(0, self.outer_tab_group.y() + get_default_tab_padding(), self.outer_tab_group.width(), self.outer_tab_group.height(), "Organizer");
		self.outer_tab_organizer.end();
		self.outer_tab_group.add(&self.outer_tab_organizer);

		// outer tab map import settings
		self.outer_tab_map_input = Group::new(0, self.outer_tab_group.y() + get_default_tab_padding(), self.outer_tab_group.width(), self.outer_tab_group.height(), "Map Import");
		self.outer_tab_map_input.end();
		self.outer_tab_group.add(&self.outer_tab_map_input);
	}//end initialize(&mut self)

	pub fn show(&mut self) {
		// set theme because why not do it here?
		let widget_scheme = WidgetScheme::new(SchemeType::Fluent);
		widget_scheme.apply();
		// actually make stuff show up
		self.main_window.show();
	}//end show(&mut self)
}//end impl for GUI

pub fn get_default_win_width() -> i32 {1000}
pub fn get_default_win_height() -> i32 {700}
fn get_default_tab_padding() -> i32 {20}