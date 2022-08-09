mod help_menus;

use help_menus as menu;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
use std::{io, process::exit};

trait NameAsNew{
    fn new(_: String) -> Self;
}
#[derive(Debug)]
struct Board {
    rooms: Vec<Vec<Box<Room>>>,
}
#[derive(Debug)]
struct Game {
    board: Board,
    avatar: Character,
    game_items: Vec<Item>,
    npcs: Vec<Character>,
    inventory: Option<Vec<Item>>,
    current_room: Option<Rc<Room>>,
}

impl Game {
    fn set_current_room(&mut self, current_room: Room) {
        self.current_room = Some(Rc::new(current_room));
    }
}

#[derive(Debug)]
struct Item {
    name: String,
}

impl NameAsNew for Item {
    fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
struct Character {
    name: String,
}

impl NameAsNew for Character {
    fn new(name: String) -> Self {
        Self { name }
    }
}
#[derive(Debug)]
struct Room<'a> {
    name: String,
    North: Option<Rc<&'a Room<'a>>>,
    South: Option<Rc<&'a Room<'a>>>,
    East: Option<Rc<&'a Room<'a>>>,
    West: Option<Rc<&'a Room<'a>>>,
    item_list: Option<Vec<Item>>,
    character_list: Option<Vec<Character>>,
}

#[derive(Debug)]
struct Solution {
    room: Room,
    item: Item,
    character: Character,
}

fn get_user_input(buffer: &mut String) -> &str {
    match io::stdin().read_line(buffer) {
        Ok(_) => buffer.trim(),
        Err(error) => {
            println!("error: {}", error);
            exit(1);
        }
    }
}

fn print_hex_array(vec: Vec<u8>) {
    // for char in vec.iter() {
    //     print!("{}", *char as char);
    // }
    let out = String::from_utf8(vec).unwrap();
    print!("{}", out);
}

/*
Prints a hardcoded hex array of help commands
*/
fn help() {
    // help_command_array is a array of chars in help_command_array.h
    print_hex_array(menu::get_command_array());
}

fn list() {
    print_hex_array(menu::get_list_array());
}

fn main() {
    let game = Game {
        board: Board {
            rooms: construct_board(),
        },
        avatar: Character { name: "You".to_owned() },
        game_items: create_game_items(),
        npcs: create_npcs(),
        inventory: None,
        current_room: None,
    };
    println!("{:#?}", game);
}

fn create_npcs() -> Vec<Character> {
    let npc_names = vec!["Katie".to_owned(), "Harry".to_owned(), "Peter".to_owned(), "Savanah".to_owned(), "Lexi".to_owned()];
    construct_list(npc_names)
}

fn construct_list<'a, T>(names: Vec<String>) -> Vec<T>
where
    T: NameAsNew,
{
    let mut list = Vec::new();
    for name in names {
        list.push(T::new(name));
    }
    list
}

fn create_game_items() -> Vec<Item> {
    let game_names = vec![
        "Rubber Ducky".to_owned(),
        "Hairdryer".to_owned(),
        "Knife".to_owned(),
        "Wine Bottle".to_owned(),
        "Chair".to_owned(),
        "Bedsheet".to_owned(),
    ];
    construct_list(game_names)
}

fn construct_board() -> Vec<Vec<Box<Room>>> {
    let room_names: Vec<String> = vec![
        "Drawing Room".to_owned(),
        "Backyard".to_owned(),
        "Hallway".to_owned(),
        "Attic".to_owned(),
        "Katie's Room".to_owned(),
        "Harry's Room".to_owned(),
        "Peter's Room".to_owned(),
        "Savanah's Room".to_owned(),
        "Lexi's Room".to_owned(),
    ];
    let mut board = Vec::new();
    assert!(
        room_names.len() == 9,
        "ERROR: NUMBER OF ROOMS must be a perfect square"
    );
    let rt = (room_names.len() as f64).sqrt() as usize;
    for i in 0..rt {
        let mut rooms = Vec::new();
        for j in 0..rt {
            let new_room = Box::new(Room {
                name: room_names.get(i * rt + j).unwrap().to_string(),
                North: None,
                South: None,
                East: None,
                West: None,
                item_list: None,
                character_list: None,
            });
            rooms.push(new_room);
        }
        board.push(rooms);
    }
    shuffle_rooms(&mut board);
    link_rooms(&mut board);
    
    board
}

fn shuffle_rooms(board: &mut Vec<Vec<Box<Room>>>) {
    let rn: usize = rand::thread_rng().gen_range(0..100);
    for i in 0..(board.len()) {
        for j in 0..(board.len()) {
            let index = rn % board.len();
            board.get_mut(i).unwrap().swap(j, index);
        }
    }
}

fn link_rooms(board: &mut Vec<Vec<Box<Room>>>) {
    let size = board.len();
    let mut rooms_iter = board.iter_mut();
    for i in 0..size {
        let rooms = rooms_iter.nth(i).unwrap();
        for j in 0..size {
            let mut this_room = rooms.get_mut(j).unwrap();
            // North
            if (i - 1) as i32 >= 0 {
                let r = rooms_iter.nth(i-1).unwrap();
                let c = r.get(j).unwrap();
                let d = c.as_ref();
                let e = Rc::new(d);
                this_room.North = Some(e);
            }
            // //South
            // if ( (i+1) < size) {
            //     this_room.South = board.get(i+1).unwrap().get_mut(j);
            // }
            // //East
            // if ( (j+1) < size) {
            //     this_room.East = board.get(i).unwrap().get_mut(j+1);
            // }
            // //West
            // if ( (j-1) >= 0) {
            //     this_room.West = board.get(i).unwrap().get_mut(j-1);
            // }
        }
    }
}

fn main2() {
    print!("\n\n");
    println!("Welcome to Clue!");
    print!("\n");
    println!("You are currently in:");
    print!("\n");

    let n_clue: usize = 0;

    let mut termbuf = String::new();

    // look(currentRoom);

    loop {
        print!("Enter a command or type help: \n");
        let buffer = get_user_input(&mut termbuf);

        print!("{}", buffer);

        print!("\n");

        if buffer.eq("help") {
            help();
        } else if buffer.eq("list") {
            list();
        }
        // else if buffer.eq("game_solution") {
        //     print!("ANSWER\n");
        //     print!("ROOM: {}\n", game_solution.room->name);
        //     print!("ITEM: {}\n", game_solution.item->name);
        //     print!("CHARACTER: {}\n", game_solution.character->name);
        //     print!("\n");
        // }
        //         else if(compare_strings(buffer, "look")) {
        //             look(currentRoom);
        //         }
        //         else if(compare_strings(buffer, "go")) {
        //             // inner loop for accepting direction
        //             while(1) {
        //                 print!("Enter north, south, east, or west: \n");
        //                 get_user_input(&buffer, &bufsize);
        //                 int direction;
        //                 // direction returns -1 if invalid direction was specified
        //                 if ( (direction = get_direction(buffer)) != -1 ) {
        //                     // returns the same room if impossible to go in specified direction
        //                     currentRoom = go(direction, currentRoom, avatar.person);
        //                     break;
        //                 }
        //                 else {
        //                     print!("Re-enter direction!\n\n");
        //                 }
        //             }
        //             look(currentRoom);
        //         }
        //         else if(compare_strings(buffer, "take")) {
        //             if (currentRoom->item_list == NULL ) {
        //                 print!("No items to take in the room!\n");
        //             }
        //             else {
        //                 // inner loop to specify which item to take
        //                 while(1) {
        //                     print!("Which item would you like to take?\n");
        //                     print!("Items in %s: ", currentRoom->name);
        //                     print_item_list(currentRoom->item_list);
        //                     print!("\n");
        //                     get_user_input(&buffer, &bufsize);

        //                     Item * taken_item;
        //                     // take() returns NULL if string in buffer does not match any item in the room
        //                     if ( (taken_item = take(currentRoom, buffer)) != NULL) {
        //                         drop_item_into_inventory(taken_item);
        //                         print!("\nItem Taken!\n");
        //                         break;
        //                     }
        //                     else {
        //                         print!("\nItem does not exist!\n");
        //                     }
        //                 }
        //             }
        //         }
        //         else if(compare_strings(buffer, "drop")) {
        //             if (avatar.inventory == NULL ) {
        //                 print!("No items in the inventory!\n");
        //             }
        //             else {
        //                 // inner loop to specify which item to drop
        //                 while(1) {
        //                     print!("Which item would you like to drop?\n");
        //                     print_inventory(avatar.inventory);
        //                     get_user_input(&buffer, &bufsize);

        //                     Item * dropped_item;
        //                     // drop() returns NULL if string in buffer does not match any item in the inventory
        //                     if ( (dropped_item = remove_item_from_inventory(buffer) ) != NULL) {
        //                         drop(currentRoom, dropped_item);
        //                         print!("\nItem Dropped!\n");
        //                         break;
        //                     }
        //                     else {
        //                         print!("\nItem does not exist in your inventory!\n\n");
        //                     }
        //                 }
        //             }
        //         }
        //         else if(compare_strings(buffer, "inventory")) {
        //             print_inventory(avatar.inventory);
        //         }
        //         else if(compare_strings(buffer, "clue")) {
        //             // inner loop for clue command sequence
        //             while(1) {
        //                 print!("Call a character to the room: ");
        //                 print_characters_array();
        //                 get_user_input(&buffer, &bufsize);

        //                 Room * char_room;

        //                 // find_room_for_character() returns NULL if no character with the name in the buffer exists
        //                 if ( (char_room = find_room_for_character(buffer)) != NULL) {
        //                     Character * called_character = remove_character_from_room(char_room, buffer);
        //                     add_character_to_room(currentRoom, called_character);

        //                     ++n_clue;

        //                     print!("\n");

        //                     int has_won_game = check_winning_state(currentRoom, avatar.inventory);
        //                     if (has_won_game) {
        //                         print!("\nCONGRATULATIONS! You've found the right game_solution!\n\n");
        //                         print!("GAME OVER!\n");
        //                         free_memory();
        //                         exit(0);
        //                     }
        //                     else {
        //                         if (n_clue==MAX_CLUES) {
        //                             print!("\nSORRY! You've couldn't finish the game in %d clues!\n\n", MAX_CLUES);
        //                             print!("GAME OVER!\n");
        //                             free_memory();
        //                             exit(0);
        //                         }
        //                     }
        //                     break;
        //                 }
        //                 else {
        //                     print!("\nSpecified character does not exist!\n\n");
        //                 }
        //             }
        //         }
        //         else {
        //             print!("Invalid command! Use help to display available commands.\n");
        //         }
        //         print!("\n");
        //     }

        // }

        // /*
        // Accepts a pointer to a char * buffer and a pointer the the size of the buffer
        // User input is accepted and put into the buffer; '\n' at the end of the input is removed
        // */
        // void get_user_input(char ** buffer, size_t * bufsize) {
        //     getline(buffer, bufsize, stdin);
        //     (*buffer)[strcspn(*buffer, "\n")] = (char)'\0';   // removes extra \n from user input
        // }

        // /*
        // Accepts a pointer to a struct Room, and a pointer to a struct Item
        // Checks if the fields in game_solution match the associated fields in the inputs
        // Return 0 for losing state and 1 for winning state
        // */
        // int check_winning_state(Room * currentRoom, Item * inventory) {
        //     int win_flag = 0;

        //     if (currentRoom == game_solution.room) {
        //         win_flag++;
        //         print!("ROOM MATCH!\n");
        //     }
        //     else {
        //         print!("INCORRECT ROOM!\n");
        //     }

        //     // checks if winning item is in the room OR in the inventory
        //     if (check_item_in_item_list(inventory, game_solution.item->name) ||
        //         check_item_in_item_list(currentRoom->item_list, game_solution.item->name)) {
        //             win_flag++;
        //             print!("ITEM MATCH!\n");
        //     }
        //     else {
        //         print!("WRONG ITEM!\n");
        //     }

        //     if (is_character_in_room(currentRoom->character_list, game_solution.character->name)) {
        //         win_flag++;
        //         print!("CHARACTER MATCH!\n");
        //     }
        //     else {
        //         print!("WRONG CHARACTER!\n");
        //     }

        //     if (win_flag==3)
        //         return 1;
        //     if (win_flag==0) {
        //         print!("NOT A VALID GUESS!\n");
    }
}
