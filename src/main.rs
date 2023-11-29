use crate::gui::GUI;

mod gui;

fn main() {
    let mut gui = GUI::default();
    gui.initialize();

    let mut map_list: Vec<String> = Vec::new();
    map_list.push("map1".to_string());
    map_list.push("map2".to_string());
    map_list.push("obvious placeholder is obvious".to_string());
    gui.update_map_view_list(&map_list);

    gui.show();
    while gui.app.wait() {
        if let Some(val) = gui.msg_receiver.recv() {
            // do something
            println!("{}", val);
            if val.eq("select-new-map-file") {
                println!("We should open file dialog!");
                match GUI::dialog_get_file() {
                    Some(val) => {
                        // placeholder for debugging
                        println!("{}", val);
                    },
                    None => println!("file dialog cancelled or path empty"),
                }//end making sure we got something from dialog
            }//end if user wants to select a file
            else if val.eq("select-new-map-folder") {
                println!("We should open folder dialog!");
                match GUI::dialog_get_folder() {
                    Some(val) => {
                        // placeholder for debugging
                        println!("{}", val);
                    },
                    None => println!("folder dialog cancelled or path empty"),
                }
            }//end if user wants to select a folder
        }//end if we got a message from gui
    }//end main app loop
}//end main fn
