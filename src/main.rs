use crate::gui::GUI;

mod gui;

fn main() {
    let mut gui = GUI::default();
    gui.initialize();

    // placeholder values for map list
    let mut map_list: Vec<String> = Vec::new();
    map_list.push("map1".to_string());
    map_list.push("map2".to_string());
    map_list.push("obvious placeholder is obvious".to_string());
    gui.update_map_view_list(&map_list);

    // set up list to hold queue for maps to add to db
    let mut map_queue: Vec<String> = Vec::new();

    gui.show();
    while gui.app.wait() {
        if let Some(val) = gui.msg_receiver.recv() {
            // do something
            println!("{}", val);
            if val.eq("select-new-map-file") {
                println!("We should open file dialog!");
                match GUI::dialog_get_file() {
                    Some(val) => {
                        // add selected file to map queue
                        map_queue.push(val);
                        // update display of map queue
                        gui.update_map_queue(&map_queue);
                    },
                    None => {println!("file dialog cancelled or path empty");},
                }//end making sure we got something from dialog
            }//end if user wants to select a file
            else if val.eq("select-new-map-folder") {
                println!("We should open folder dialog!");
                match GUI::dialog_get_folder() {
                    Some(val) => {
                        // TODO: Add all files from folder to map queue
                        // TODO: Look inside zipped folders to find map files
                        println!("{}", val);
                        GUI::show_message("Folder selection is currently not supported. \nPlease select individual files.");
                    },
                    None => {println!("folder dialog cancelled or path empty");},
                }
            }//end if user wants to select a folder
        }//end if we got a message from gui
    }//end main app loop
}//end main fn
