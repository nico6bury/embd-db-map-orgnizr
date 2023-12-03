use fltk::{prelude::{WidgetExt, GroupExt}, group::{Scroll, Pack}, widget_extends, button::Button};


pub struct ButtonList {
	/// This is the scroll group that contains everything else
	scroll: Scroll,
	/// This pack will contain all the buttons that make up 
	/// the elements in the list box.
	list_pack: Pack,
}//end struct ButtonList

#[allow(dead_code)]
impl ButtonList {
	/// # new()
	/// 
	/// Initializes a new ButtonList.
	pub fn new(width: i32, height: i32) -> ButtonList {
		// get initial settings
		let mut temp_scroll = ButtonList::init_scroll();
		let mut temp_pack = ButtonList::init_pack();

		// set the size(
		temp_scroll.set_size(width, height);
		temp_pack.set_size(width, height);

		ButtonList {
			scroll: temp_scroll,
			list_pack: temp_pack,
		}//end struct construction
	}//end new()

	/// # set_button_list()
	/// 
	/// This function replaces any buttons currently displayed with the buttons provided.  
	/// The size of each button will be determined automatically and overwritten.  
	/// button_height should not be positive.
	pub fn set_button_list(&mut self, mut button_list: Vec<Button>, button_height: i32) {
		// remove any previously added buttons
		self.list_pack.clear();
		// add all the new buttons
		for i in 0..button_list.len() {
			let mut this_btn = button_list.get_mut(i).expect("index should not be out of bounds").to_owned();
			this_btn.set_size(self.list_pack.width(), button_height);
			let button_y = i as i32 * button_height;
			this_btn.set_pos(0, button_y);
			self.list_pack.add(&this_btn);
		}//end adding each button to the pack
		// update height of the pack
		self.list_pack.set_size(self.scroll.width(), button_list.len() as i32 * button_height)
	}//end set_button_list()

	/// # set_string_list
	/// 
	/// This function simply creates default buttons with labels from 
	/// str_list and populates them into set_button_list().
	pub fn set_string_list(&mut self, str_list: &Vec<String>, button_height: i32) {
		let mut btn_lst: Vec<Button> = Vec::new();
		for str in str_list {
			let btn = Button::default()
				.with_label(str);
			btn_lst.push(btn);
		}//end adding each string as label to new button
		self.set_button_list(btn_lst, button_height);
	}//end set_string_list()

	/// # update_width()
	/// 
	/// Call this function whenever the width of this widget 
	/// is set via function.
	pub fn update_width(&mut self) {
		self.list_pack.set_size(self.scroll.width(), self.list_pack.height());
	}//end update_width(self)

	/// # init_scroll()
	/// 
	/// This function essentially just creates a Scroll object
	/// for the initial setting of self.scroll.
	fn init_scroll() -> Scroll {
		let temp_scroll = Scroll::default();
		temp_scroll.end();

		return temp_scroll;
	}//end init_scroll()

	/// # init_pack()
	/// 
	/// This function essentially just creates a pack object for the initial setting of self.scroll
	fn init_pack() -> Pack {
		let temp_pack = Pack::default();

		return temp_pack;
	}//end init_pack()
}//end impl for ButtonList

widget_extends!(ButtonList, Scroll, scroll);