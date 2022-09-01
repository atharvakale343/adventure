use std::{cell::RefCell, rc::Rc};

use crate::{
    config::PRINT_WIDTH, get_user_input, help_menus as menu, Game, Room,
};

enum Direction {
    North,
    South,
    East,
    West,
    Invalid,
}

impl Direction {
    fn get_direction(buffer: &str) -> Direction {
        if buffer.eq("north") {
            return Direction::North;
        }
        if buffer.eq("south") {
            return Direction::South;
        }
        if buffer.eq("east") {
            return Direction::East;
        }
        if buffer.eq("west") {
            return Direction::West;
        }
        Direction::Invalid
    }
}

fn print_hex_array(vec: Vec<u8>) {
    // for char in vec.iter() {
    //     print!("{}", *char as char);
    // }
    let out = String::from_utf8(vec).unwrap();
    print!("{}", out);
}

fn print_demarkcation_line() {
    println!("{}", "═".repeat(PRINT_WIDTH));
}
pub(crate) fn print_center(text: &str) {
    let padlen = PRINT_WIDTH.saturating_sub(text.len()) / 2;
    println!("{:indent$}{}", "", text, indent = padlen);
}

fn print_room(room: &RefCell<Room>) {
    let room = room.borrow();
    print_demarkcation_line();
    print_center(&room.name);
    print_demarkcation_line();
    print_center("Rooms around you:");
    print_center(room.around().as_str());
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

pub(crate) fn look(room: &RefCell<Room>) {
    print_room(room);
}
pub(crate) fn go(game: &mut Game) {
    loop {
        println!("Enter north, south, east, or west:");
        let buffer = get_user_input();

        let direction: Direction = Direction::get_direction(&buffer);
        let new_room: Option<Rc<RefCell<Room>>>;

        {
            let this_room = game.current_room.as_ref().unwrap().borrow();

            let new_room_ref = match direction {
                Direction::North => this_room.north.as_ref(),
                Direction::South => this_room.south.as_ref(),
                Direction::East => this_room.east.as_ref(),
                Direction::West => this_room.west.as_ref(),
                Direction::Invalid => {
                    println!("\nRe-enter direction!\n");
                    continue;
                }
            };

            new_room = new_room_ref.map(Rc::clone);
        }

        match new_room {
            Some(room_ref) => {
                game.set_current_room(&room_ref);
            }
            None => {
                println!("\nCannot go that way!\n");
            }
        }
        break;
    }
}
