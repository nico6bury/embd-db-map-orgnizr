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
	outer_tab_group: Tabs,
	/// part of outer tab group with stuff for searching for maps
	outer_tab_organizer: Group,
	/// part of outer tab group with stuff for adding maps to db
	outer_tab_map_input: Group,

	/// tab group containing tabs for manipulating maps in the database
	organizer_group: Tabs,
	/// tab for viewing statistics about the maps stored in the database
	organizer_view: Group,
	/// tab for filtering maps in the database to find specific maps
	organizer_filter: Group,

	/// tab group containing tabs for adding maps to database
	map_input_group: Tabs,
	/// tab for assembling files of maps to add to database
	map_input_files: Group,
	/// tab for looking through assembled maps and assigning metadata
	map_input_tagger: Group,
}//end struct GUI

impl Default for GUI {
    fn default() -> Self {
        Self {
			app: Default::default(), 
			main_window: Default::default(),
			outer_tab_group: Default::default(),
			outer_tab_organizer: Default::default(),
			outer_tab_map_input: Default::default(),
            map_input_group: Default::default(),
            map_input_files: Default::default(),
            map_input_tagger: Default::default(),
            organizer_group: Default::default(),
            organizer_view: Default::default(),
            organizer_filter: Default::default(),
		}//end Self constructor
    }//end default()
}//end impl Default for GUI

impl GUI {
	/// # initialize(self)
	/// 
	/// This function sets up the default properties all all
	/// the GUI elements. Call this and show() to set up 
	/// the GUI and make it visible.
	pub fn initialize(&mut self) {
		// window settings
		self.main_window.set_size(get_default_win_width(),get_default_win_height());
		self.main_window.make_resizable(true);
		self.main_window.set_label("Embedded DB Map Organizer");
		self.main_window.set_label_size(32);
		self.main_window.set_label_color(Color::Green);

		// set up organization of the tabs organization
		self.initialize_tabs();
		// set up tab for viewing maps in database
		self.initialize_map_view();
	}//end initialize(&mut self)

	/// # initialize_tabs(self)
	/// 
	/// This function is a little helper method that sets up
	/// default properties for the tabs and groups that contain
	/// all the other parts of the GUI.
	fn initialize_tabs(&mut self) {
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

		// db organizer tab group settings
		self.organizer_group = Tabs::new(0, self.outer_tab_organizer.y(), get_default_win_width(), get_default_win_height() - get_default_tab_padding(), None);
		self.organizer_group.auto_layout();
		self.organizer_group.end();
		self.outer_tab_organizer.add(&self.organizer_group);

		// db organizer view settings
		self.organizer_view = Group::new(0, self.organizer_group.y() + get_default_tab_padding(), self.organizer_group.width(), self.organizer_group.height(), "View");
		self.organizer_view.end();
		self.organizer_group.add(&self.organizer_view);

		// db organizer filter settings
		self.organizer_filter = Group::new(0, self.organizer_group.y() + get_default_tab_padding(), self.organizer_group.width(), self.organizer_group.height(), "Filter");
		self.organizer_filter.end();
		self.organizer_group.add(&self.organizer_filter);

		// db import tab group settings
		self.map_input_group = Tabs::new(0, self.outer_tab_map_input.y(), get_default_win_width(), get_default_win_height() - get_default_tab_padding(), None);
		self.map_input_group.auto_layout();
		self.map_input_group.end();
		self.outer_tab_map_input.add(&self.map_input_group);

		// db import files settings
		self.map_input_files = Group::new(0, self.map_input_group.y() + get_default_tab_padding(), self.map_input_group.width(), self.map_input_group.height(), "Files");
		self.map_input_files.end();
		self.map_input_group.add(&self.map_input_files);

		// db import tagger settings
		self.map_input_tagger = Group::new(0, self.map_input_group.y() + get_default_tab_padding(), self.map_input_group.width(), self.map_input_group.height(), "Tagger");
		self.map_input_tagger.end();
		self.map_input_group.add(&self.map_input_tagger);
	}

	fn initialize_map_view(&mut self) {

	}//end initialize_map_view(self)

	/// # show(self)
	/// 
	/// This function makes the GUI visible.
	/// Note that the initialize() function should be called first.
	pub fn show(&mut self) {
		// set theme because why not do it here?
		let widget_scheme = WidgetScheme::new(SchemeType::Fluent);
		widget_scheme.apply();
		// actually make stuff show up
		self.main_window.show();
		self.main_window.resize(self.main_window.x(), self.main_window.y(), self.main_window.width() + 1, self.main_window.height());
	}//end show(&mut self)
}//end impl for GUI

pub fn get_default_win_width() -> i32 {1000}
pub fn get_default_win_height() -> i32 {700}
fn get_default_tab_padding() -> i32 {20}