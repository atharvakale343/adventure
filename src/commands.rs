use std::{cell::RefCell, rc::Rc};

use crate::{config::PRINT_WIDTH, help_menus as menu, Room};

fn print_hex_array(vec: Vec<u8>) {
    // for char in vec.iter() {
    //     print!("{}", *char as char);
    // }
    let out = String::from_utf8(vec).unwrap();
    print!("{}", out);
}

fn print_demarkcation_line() {
    print!("{}\n", "═".repeat(PRINT_WIDTH));
}
pub(crate) fn print_center(text: &str) {
    let padlen = PRINT_WIDTH.checked_sub(text.len()).unwrap_or(0) / 2;
    println!("{:indent$}{}", "", text, indent = padlen);
}

fn print_room(room: Rc<RefCell<Room>>) {
    let room = room.borrow();
    print_demarkcation_line();
    print_center(&room.name);
    print_demarkcation_line();
    print_center("Rooms around you:");
    print_center(format!("{}", room.around()).as_str());
    print_demarkcation_line();
    print_center(format!("Items in {}:", room.name.as_str()).as_str());
    print_center(room.item_list_as_string().as_str());
    print_demarkcation_line();
    print_center(format!("Characters in {}:", room.name.as_str()).as_str());
    print_center(room.character_list_as_string().as_str());
    print_demarkcation_line();
}

/*
Prints a hardcoded hex array of help commands
*/
pub(crate) fn help() {
    // help_command_array is a array of chars in help_command_array.h
    print_hex_array(menu::get_command_array());
}

pub(crate) fn list() {
    print_hex_array(menu::get_list_array());
}

pub(crate) fn look(room: Rc<RefCell<Room>>) {
    print_room(room);
}
