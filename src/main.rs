mod commands;
mod config;
mod help_menus;

use crate::commands::*;
use crate::config::ITEM_NAMES;
use crate::config::MAX_CLUES;
use crate::config::NPC_NAMES;
use crate::config::ROOM_NAMES;

use rand::Rng;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::{io, process::exit};

macro_rules! room {
    ($expression:expr) => {
        $expression.as_ref().unwrap().as_ref()
    };
}

fn main() {
    println!("\n");
    print_center("Welcome to Clue!");
    println!();
    print_center("You are currently in:");
    println!();

    let mut n_clue: usize = 0;

    let mut game = Game::new(
        ROOM_NAMES.to_vec(),
        ITEM_NAMES.to_vec(),
        NPC_NAMES.to_vec(),
        Entity::new("You".to_owned()),
    );

    look(room!(game.current_room));

    loop {
        println!("Enter a command or type help:");
        let mut buffer = get_user_input();

        println!();

        if buffer.eq("help") {
            help();
        } else if buffer.eq("list") {
            list();
        } else if buffer.eq("game_solution") {
            println!("{}", game.solution);
        } else if buffer.eq("look") {
            look(room!(game.current_room));
        } else if buffer.eq("go") {
            // inner loop for accepting direction
            go(&mut game);
            look(room!(game.current_room));
        } else if buffer.eq("take") {
            let mut curr_room_ref = game.current_room.as_mut().unwrap().borrow_mut();
            let item_list = &mut curr_room_ref.item_list;
            if item_list.is_empty() {
                println!("No items to take in the room!");
            } else {
                loop {
                    println!("Items in {}:", curr_room_ref.name);
                    println!(
                        "{}\n",
                        Entity::entity_list_as_string(
                            &curr_room_ref.item_list,
                            "ERROR: Unreachable"
                        )
                    );
                    println!("Which item would you like to take?");
                    buffer = get_user_input();

                    let result = Entity::move_entity_by_name(
                        &buffer,
                        &mut curr_room_ref.item_list,
                        &mut game.inventory,
                        "Item does not exist",
                    );
                    match result {
                        Ok(_) => (),
                        Err(message) => {
                            println!("{}", message);
                            continue;
                        }
                    }
                    println!("Item Taken!");
                    break;
                }
            }
        } else if buffer.eq("drop") {
            if game.inventory.is_empty() {
                println!("No items to take in the room!");
            } else {
                let mut curr_room_ref = game.current_room.as_mut().unwrap().borrow_mut();
                loop {
                    println!(" Items in inventory:");
                    println!(
                        "{}\n",
                        Entity::entity_list_as_string(&game.inventory, "ERROR: Unreachable")
                    );
                    println!("Which item would you like to take?");
                    buffer = get_user_input();

                    let result = Entity::move_entity_by_name(
                        &buffer,
                        &mut game.inventory,
                        &mut curr_room_ref.item_list,
                        "Item does not exist",
                    );
                    match result {
                        Ok(_) => (),
                        Err(message) => {
                            println!("{}", message);
                            continue;
                        }
                    }
                    println!("Item Dropped!");
                    break;
                }
            }
        } else if buffer.eq("inventory") {
            println!(
                "Items in inventory: {}",
                Entity::entity_list_as_string(&game.inventory, "No items in inventory!")
            );
        } else if buffer.eq("clue") {
            // inner loop for clue command sequence
            loop {
                print!("Call a character to the room: ");
                println!(
                    "{}",
                    Entity::entity_list_as_string(&game.npcs, "ERROR: No characters.")
                );

                buffer = get_user_input();

                let room = game.board.find_room_for_character_by_name(&buffer);

                if room.is_none() {
                    println!("Specified character does not exist!");
                    continue;
                }

                let room = room.unwrap();

                if !Rc::ptr_eq(&room, game.current_room.as_ref().unwrap()) {
                    Entity::move_entity_by_name(
                        &buffer,
                        &mut room.borrow_mut().character_list,
                        &mut game
                            .current_room
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .character_list,
                        "ERROR: Failed to move",
                    )
                    .ok()
                    .unwrap();
                }

                let win_state: WinningState = game.get_win_state();

                n_clue += 1;

                println!("{}", win_state);

                if matches!(win_state.room_match, State::Match)
                    && matches!(win_state.item_match, State::Match)
                    && matches!(win_state.character_match, State::Match)
                {
                    println!("\nCONGRATULATIONS! You've found the right game_solution!\n");
                    println!("GAME OVER!");
                    exit(0);
                }

                if n_clue == MAX_CLUES {
                    println!("SORRY, YOU COULDN'T FINISH THE GAME IN {} ATTEMPTS", n_clue);
                    println!("GAME OVER!");
                    exit(0);
                }

                println!("{} ATTEMPT(S) REMAINING\n", MAX_CLUES - n_clue);
                break;
            }
        } else {
            println!("Invalid command! Use `help` to display available commands.\n");
        }
    }
}
struct WinningState {
    room_match: State,
    item_match: State,
    character_match: State,
}

impl WinningState {
    fn new() -> Self {
        Self { room_match: State::Miss, item_match: State::Miss, character_match: State::Miss }
    }
}

enum State {
    Match,
    Miss,
}

struct Board {
    rooms: Vec<Vec<Rc<RefCell<Room>>>>,
}

impl Board {
    fn get_random_room(&self) -> Rc<RefCell<Room>> {
        let i: usize = rand::thread_rng().gen_range(0..self.rooms.len());
        let j: usize = rand::thread_rng().gen_range(0..self.rooms.len());
        Rc::clone(self.rooms.get(i).unwrap().get(j).unwrap())
    }

    fn find_room_for_character_by_name(&self, name: &str) -> Option<Rc<RefCell<Room>>> {
        for room_row in &self.rooms {
            for room in room_row {
                if Entity::find_entity_by_name(name, &room.borrow().character_list).is_some() {
                    return Some(Rc::clone(room));
                } else {
                    continue;
                }
            }
        }
        None
    }
}
#[allow(dead_code)]
struct Game {
    board: Board,
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
                ROOM_NAMES.len() == 9,
                "ERROR: NUMBER OF ROOMS must be a perfect square"
            );
            let rt = (ROOM_NAMES.len() as f64).sqrt() as usize;
            for i in 0..rt {
                let mut rooms = Vec::new();
                for j in 0..rt {
                    let new_room = Rc::new(RefCell::new(Room {
                        name: ROOM_NAMES.get(i * rt + j).unwrap().to_string(),
                        north: None,
                        south: None,
                        east: None,
                        west: None,
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
                        this_room.north =
                            Some(Rc::clone(board.get(i - 1).unwrap().get(j).unwrap()));
                    }
                    //South
                    if (i + 1) < size {
                        this_room.south =
                            Some(Rc::clone(board.get(i + 1).unwrap().get(j).unwrap()));
                    }
                    //East
                    if (j + 1) < size {
                        this_room.east = Some(Rc::clone(board.get(i).unwrap().get(j + 1).unwrap()));
                    }
                    //West
                    if j > 0 {
                        this_room.west = Some(Rc::clone(board.get(i).unwrap().get(j - 1).unwrap()));
                    }
                }
            }
        }
        fn distribute_characters(board: &mut Board, entities: &mut Vec<Rc<RefCell<Entity>>>) {
            for entity in entities {
                let random_room = board.get_random_room();
                Entity::push_entity(
                    &mut random_room.borrow_mut().character_list,
                    Rc::clone(entity),
                );
            }
        }
        fn distribute_items(board: &mut Board, entities: &mut Vec<Rc<RefCell<Entity>>>) {
            for entity in entities {
                let random_room = board.get_random_room();
                Entity::push_entity(&mut random_room.borrow_mut().item_list, Rc::clone(entity));
            }
        }
        let mut _board = Board {
            rooms: construct_rooms(_room_names),
        };
        let mut _game_items = create_game_items(_item_names);
        let mut _npcs = create_npcs(_npc_names);
        let _solution = Solution::new(&_board, &_game_items, &_npcs);
        let _current_room = { _board.get_random_room() };
        Entity::push_entity(
            &mut _current_room.borrow_mut().character_list,
            Rc::new(RefCell::new(avatar)),
        );
        distribute_characters(&mut _board, &mut _npcs);
        distribute_items(&mut _board, &mut _game_items);
        Game {
            board: _board,
            game_items: _game_items,
            npcs: _npcs,
            inventory: Vec::new(),
            current_room: Some(_current_room),
            solution: _solution,
        }
    }

    fn set_current_room(&mut self, new_room: &Rc<RefCell<Room>>) {
        Entity::move_entity_by_name(
            "You",
            &mut self
                .current_room
                .as_mut()
                .unwrap()
                .borrow_mut()
                .character_list,
            &mut new_room.borrow_mut().character_list,
            "ERROR: Unreachable",
        )
        .ok()
        .unwrap();
        self.current_room = Some(Rc::clone(new_room));
    }

    pub(crate) fn get_win_state(&self) -> WinningState {
        let mut win_state = WinningState::new();
        if Rc::ptr_eq(self.current_room.as_ref().unwrap(), &self.solution.room) {
            win_state.room_match = State::Match;
        }
        let correct_item = &self.solution.item.borrow().name;
        if Entity::find_entity_by_name(
            correct_item,
            &self.current_room.as_ref().unwrap().borrow().item_list,
        )
        .is_some()
            || Entity::find_entity_by_name(correct_item, &self.inventory).is_some()
        {
            win_state.item_match = State::Match;
        }
        let correct_character = &self.solution.character.borrow().name;
        if Entity::find_entity_by_name(
            correct_character,
            &self.current_room.as_ref().unwrap().borrow().character_list,
        )
        .is_some()
        {
            win_state.character_match = State::Match;
        }
        win_state
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

    fn entity_list_as_string(list: &Vec<Rc<RefCell<Entity>>>, default: &str) -> String {
        let mut string: String = String::new();
        fn form(str1: &str) -> String {
            format!("{} | ", str1)
        }
        if list.is_empty() {
            string.push_str(default);
        }
        for entity in list.iter() {
            string.push_str(&form(&entity.borrow().name));
        }
        string
    }

    fn find_entity_by_name(name: &str, list: &[Rc<RefCell<Entity>>]) -> Option<usize> {
        list.iter()
            .position(|x| x.borrow().name.eq_ignore_ascii_case(name))
    }
    fn push_entity(list: &mut Vec<Rc<RefCell<Entity>>>, item: Rc<RefCell<Entity>>) {
        list.push(item);
    }
    fn remove_entity(
        list: &mut Vec<Rc<RefCell<Entity>>>,
        name: &str,
    ) -> Option<Rc<RefCell<Entity>>> {
        let position = Entity::find_entity_by_name(name, list);
        position.map(|position| list.remove(position))
    }
    fn move_entity_by_name(
        name: &str,
        source: &mut Vec<Rc<RefCell<Entity>>>,
        dest: &mut Vec<Rc<RefCell<Entity>>>,
        default: &str,
    ) -> Result<(), String> {
        let item = Entity::remove_entity(source, name);
        match item {
            Some(item) => {
                dest.push(item);
                Ok(())
            }
            None => Err(default.to_owned()),
        }
    }
}

struct Room {
    name: String,
    north: Option<Rc<RefCell<Room>>>,
    south: Option<Rc<RefCell<Room>>>,
    east: Option<Rc<RefCell<Room>>>,
    west: Option<Rc<RefCell<Room>>>,
    item_list: Vec<Rc<RefCell<Entity>>>,
    character_list: Vec<Rc<RefCell<Entity>>>,
}

impl Room {
    fn around(&self) -> String {
        let mut around: String = String::new();
        fn form(str1: &str, str2: &str) -> String {
            format!(" {} ({}) |", str1, str2)
        }
        if let Some(room) = self.north.as_ref() {
            around.push_str(&form(&room.borrow().name, "North"));
        }
        if let Some(room) = self.south.as_ref() {
            around.push_str(&form(&room.borrow().name, "South"));
        }
        if let Some(room) = self.east.as_ref() {
            around.push_str(&form(&room.borrow().name, "East"));
        }
        if let Some(room) = self.west.as_ref() {
            around.push_str(&form(&room.borrow().name, "West"));
        }
        around
    }
    fn item_list_as_string(&self) -> String {
        Entity::entity_list_as_string(&self.item_list, "No items.")
    }
    fn character_list_as_string(&self) -> String {
        Entity::entity_list_as_string(&self.character_list, "No characters.")
    }
}

impl fmt::Debug for Room {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let north = match self.north.as_ref() {
            Some(r) => (r.borrow().name.clone()),
            None => "None".to_owned(),
        };
        let south = match self.south.as_ref() {
            Some(r) => (r.borrow().name.clone()),
            None => "None".to_owned(),
        };
        let east = match self.east.as_ref() {
            Some(r) => (r.borrow().name.clone()),
            None => "None".to_owned(),
        };
        let west = match self.west.as_ref() {
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

impl fmt::Display for State {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let display = match *self {
            State::Match => "MATCH",
            State::Miss => "MISS",
        };
        write!(f, "{}", display)
    }
}
impl fmt::Display for WinningState {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "\nANSWER\n\
                 ROOM: {} \n\
                 ITEM: {} \n\
            CHARACTER: {} \n\
            ",
            self.room_match, self.item_match, self.character_match,
        )
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

fn get_user_input() -> String {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim().to_owned(),
        Err(error) => {
            println!("error: {}", error);
            exit(1);
        }
    }
}