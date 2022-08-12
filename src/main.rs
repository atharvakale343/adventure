mod commands;
mod config;
mod help_menus;

use crate::commands::*;
use crate::config::item_names;
use crate::config::npc_names;
use crate::config::room_names;

use rand::Rng;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::{io, process::exit};

enum Direction {
    North,
    South,
    East,
    West,
    Invalid,
}

#[derive(Debug)]
struct Board {
    rooms: Vec<Vec<Rc<RefCell<Room>>>>,
}

impl Board {
    fn get_random_room(&self) -> Rc<RefCell<Room>> {
        let i: usize = rand::thread_rng().gen_range(0..self.rooms.len());
        let j: usize = rand::thread_rng().gen_range(0..self.rooms.len());
        Rc::clone(self.rooms.get(i).unwrap().get(j).unwrap())
    }
}
#[derive(Debug)]
struct Game {
    board: Board,
    avatar: Rc<RefCell<Entity>>,
    game_items: Vec<Rc<RefCell<Entity>>>,
    npcs: Vec<Rc<RefCell<Entity>>>,
    inventory: Vec<Rc<RefCell<Entity>>>,
    current_room: Option<Rc<RefCell<Room>>>,
    solution: Solution,
}

impl Game {
    fn new(
        _room_names: Vec<&str>,
        _item_names: Vec<&str>,
        _npc_names: Vec<&str>,
        avatar: Entity,
    ) -> Self {
        let _room_names = _room_names.into_iter().map(String::from).collect();
        let _item_names = _item_names.into_iter().map(String::from).collect();
        let _npc_names = _npc_names.into_iter().map(String::from).collect();
        fn construct_rooms(_room_names: Vec<String>) -> Vec<Vec<Rc<RefCell<Room>>>> {
            let mut board = Vec::new();
            assert!(
                room_names.len() == 9,
                "ERROR: NUMBER OF ROOMS must be a perfect square"
            );
            let rt = (room_names.len() as f64).sqrt() as usize;
            for i in 0..rt {
                let mut rooms = Vec::new();
                for j in 0..rt {
                    let new_room = Rc::new(RefCell::new(Room {
                        name: room_names.get(i * rt + j).unwrap().to_string(),
                        North: None,
                        South: None,
                        East: None,
                        West: None,
                        item_list: Vec::new(),
                        character_list: Vec::new(),
                    }));
                    rooms.push(new_room);
                }
                board.push(rooms);
            }
            shuffle_rooms(&mut board);
            link_rooms(&mut board);

            board
        }
        fn create_game_items(_item_names: Vec<String>) -> Vec<Rc<RefCell<Entity>>> {
            construct_list(_item_names)
        }
        fn create_npcs(_npc_names: Vec<String>) -> Vec<Rc<RefCell<Entity>>> {
            construct_list(_npc_names)
        }
        fn construct_list(names: Vec<String>) -> Vec<Rc<RefCell<Entity>>> {
            let mut list = Vec::new();
            for name in names {
                list.push(Rc::new(RefCell::new(Entity::new(name))));
            }
            list
        }
        fn shuffle_rooms(board: &mut Vec<Vec<Rc<RefCell<Room>>>>) {
            let rn: usize = rand::thread_rng().gen_range(0..100);
            for i in 0..(board.len()) {
                for j in 0..(board.len()) {
                    let index = rn % board.len();
                    board.get_mut(i).unwrap().swap(j, index);
                }
            }
        }
        fn link_rooms(board: &mut Vec<Vec<Rc<RefCell<Room>>>>) {
            let size = board.len();
            for i in 0..size {
                for j in 0..size {
                    let this_room_ptr = Rc::clone(board.get(i).unwrap().get(j).unwrap());
                    let mut this_room = this_room_ptr.borrow_mut();
                    // North
                    if i > 0 {
                        this_room.North =
                            Some(Rc::clone(board.get(i - 1).unwrap().get(j).unwrap()));
                    }
                    //South
                    if (i + 1) < size {
                        this_room.South =
                            Some(Rc::clone(board.get(i + 1).unwrap().get(j).unwrap()));
                    }
                    //East
                    if (j + 1) < size {
                        this_room.East = Some(Rc::clone(board.get(i).unwrap().get(j + 1).unwrap()));
                    }
                    //West
                    if j > 0 {
                        this_room.West = Some(Rc::clone(board.get(i).unwrap().get(j - 1).unwrap()));
                    }
                }
            }
        }
        let _board = Board {
            rooms: construct_rooms(_room_names),
        };
        let _game_items = create_game_items(_item_names);
        let _npcs = create_npcs(_npc_names);
        let _solution = Solution::new(&_board, &_game_items, &_npcs);
        let _current_room = { _board.get_random_room() };
        let avatar_ptr = Rc::new(RefCell::new(avatar));
        _current_room
            .borrow_mut()
            .push_character(avatar_ptr.clone().into());
        Game {
            board: _board,
            avatar: avatar_ptr,
            game_items: _game_items,
            npcs: _npcs,
            inventory: Vec::new(),
            current_room: Some(_current_room),
            solution: _solution,
        }
    }

    fn set_current_room(&mut self, current_room: &Rc<RefCell<Room>>) {
        self.current_room = Some(Rc::clone(current_room));
        // TODO: Move Avatar into new room
    }
}

#[derive(Debug)]
struct Entity {
    name: String,
}

impl Entity {
    fn new(name: String) -> Self {
        Self { name }
    }
}

struct Room {
    name: String,
    North: Option<Rc<RefCell<Room>>>,
    South: Option<Rc<RefCell<Room>>>,
    East: Option<Rc<RefCell<Room>>>,
    West: Option<Rc<RefCell<Room>>>,
    item_list: Vec<Rc<RefCell<Entity>>>,
    character_list: Vec<Rc<RefCell<Entity>>>,
}

impl Room {
    fn around(&self) -> String {
        let mut around: String = String::new();
        fn form(str1: &str, str2: &str) -> String {
            format!(" {} ({}) |", str1, str2)
        }
        if let Some(room) = self.North.as_ref() {
            around.push_str(&form(&room.borrow().name, "North"));
        }
        if let Some(room) = self.South.as_ref() {
            around.push_str(&form(&room.borrow().name, "South"));
        }
        if let Some(room) = self.East.as_ref() {
            around.push_str(&form(&room.borrow().name, "East"));
        }
        if let Some(room) = self.West.as_ref() {
            around.push_str(&form(&room.borrow().name, "West"));
        }
        around
    }
    fn item_list_as_string(&self) -> String {
        let mut string: String = String::new();
        fn form(str1: &str) -> String {
            format!(" {} |", str1)
        }
        if self.item_list.is_empty() {
            string.push_str("No items.");
        }
        for item in self.item_list.iter() {
            string.push_str(&form(&item.borrow().name));
        }
        string
    }
    fn character_list_as_string(&self) -> String {
        let mut string: String = String::new();
        fn form(str1: &str) -> String {
            format!(" {} |", str1)
        }
        if self.character_list.is_empty() {
            string.push_str("No characters.");
        }
        for character in self.character_list.iter() {
            string.push_str(&form(&character.borrow().name));
        }
        string
    }

    fn push_item(&mut self, item: Rc<RefCell<Entity>>) {
        self.item_list.push(item);
    }
    fn push_character(&mut self, entity: Rc<RefCell<Entity>>) {
        self.character_list.push(entity);
    }
}

impl fmt::Debug for Room {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let north = match self.North.as_ref() {
            Some(r) => (r.borrow().name.clone()),
            None => "None".to_owned(),
        };
        let south = match self.South.as_ref() {
            Some(r) => (r.borrow().name.clone()),
            None => "None".to_owned(),
        };
        let east = match self.East.as_ref() {
            Some(r) => (r.borrow().name.clone()),
            None => "None".to_owned(),
        };
        let west = match self.West.as_ref() {
            Some(r) => (r.borrow().name.clone()),
            None => "None".to_owned(),
        };
        write!(
            f,
            "Name: {} | North: {} | South: {} | East: {} | West: {} |",
            self.name, north, south, east, west
        )
    }
}

#[derive(Debug)]
struct Solution {
    room: Rc<RefCell<Room>>,
    item: Rc<RefCell<Entity>>,
    character: Rc<RefCell<Entity>>,
}

impl Solution {
    fn new(
        board: &Board,
        items: &Vec<Rc<RefCell<Entity>>>,
        characters: &Vec<Rc<RefCell<Entity>>>,
    ) -> Self {
        let room = { board.get_random_room() };
        let item = {
            let rn: usize = rand::thread_rng().gen_range(0..items.len());
            Rc::clone(items.get(rn).unwrap())
        };
        let character = {
            let rn: usize = rand::thread_rng().gen_range(0..characters.len());
            Rc::clone(characters.get(rn).unwrap())
        };

        Self {
            room,
            item,
            character,
        }
    }
}

impl fmt::Display for Solution {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "ANSWER\n\
                 ROOM: {} \n\
                 ITEM: {} \n\
            CHARACTER: {} \n\
            ",
            self.room.borrow().name,
            self.item.borrow().name,
            self.character.borrow().name,
        )
    }
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

fn main2() {
    let game = Game::new(
        room_names.to_vec(),
        item_names.to_vec(),
        npc_names.to_vec(),
        Entity::new("You".to_owned()),
    );
    println!("{:#?}", game.solution);
}

macro_rules! rc {
    ($expression:expr) => {
        Rc::clone(&$expression.as_ref().unwrap())
    };
}

fn main() {
    print!("\n\n");
    print_center("Welcome to Clue!");
    print!("\n");
    print_center("You are currently in:");
    print!("\n");

    let n_clue: usize = 0;

    let mut termbuf = String::new();

    let mut game = Game::new(
        room_names.to_vec(),
        item_names.to_vec(),
        npc_names.to_vec(),
        Entity::new("You".to_owned()),
    );

    game.current_room
        .as_mut()
        .unwrap()
        .borrow_mut()
        .push_item(game.game_items.remove(0));

    look(rc!(game.current_room));

    loop {
        print!("Enter a command or type help: \n");
        termbuf.clear();
        let mut buffer = get_user_input(&mut termbuf);

        print!("\n");

        if buffer.eq("help") {
            help();
        } else if buffer.eq("list") {
            list();
        } else if buffer.eq("game_solution") {
            println!("{}", game.solution);
        } else if buffer.eq("look") {
            look(rc!(game.current_room));
        } else if buffer.eq("go") {
            // inner loop for accepting direction
            loop {
                print!("Enter north, south, east, or west: \n");
                termbuf.clear();
                println!("termbuf: {}", termbuf);
                buffer = get_user_input(&mut termbuf);

                let direction: Direction = get_direction(buffer);
                let mut new_room: Option<Rc<RefCell<Room>>>;

                {
                    let this_room = game.current_room.as_ref().unwrap().borrow();

                    let new_room_ref = match direction {
                        Direction::North => this_room.North.as_ref(),
                        Direction::South => this_room.South.as_ref(),
                        Direction::East => this_room.East.as_ref(),
                        Direction::West => this_room.West.as_ref(),
                        Direction::Invalid => {
                            println!("\nRe-enter direction!\n");
                            continue;
                        },
                    };

                    new_room = match new_room_ref {
                        Some(reference) => Some(Rc::clone(&reference)),
                        None => None,
                    };
                }

                match new_room {
                    Some(room_ref) => {
                        game.set_current_room(&room_ref);
                    },
                    None => {
                        println!("\nCannot go that way!\n");
                    },
                }
                break;
            }
            look(rc!(game.current_room));
        }
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
