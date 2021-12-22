pub mod actions;

use crate::repl::actions::get as get_action;
use crate::repl::actions::get_hash as get_hash_action;
use crate::repl::actions::keys as keys_action;
use crate::repl::actions::set as set_action;
use rustyline::Editor;
use skydbcore::dbcore::model::DB;
use skydbcore::init_db;
use std::process::exit;

pub fn start_repl() {
    let mut rl = Editor::<()>::new();
    let mut line = String::new();
    let mut db = init_db();
    loop {
        line.clear();
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                operations(line, &mut db);
            }
            Err(error) => {
                println!("Error: {:?}", error);
                break;
            }
        }
    }
}

fn operations(input: String, db: &mut DB) {
    let split_string = input.split_whitespace().collect::<Vec<&str>>();
    let command = split_string[0];
    match command {
        "set" => {
            set_action(split_string, db);
        }
        "get" => {
            get_action(split_string, db);
        }
        "get_hash" => {
            get_hash_action(split_string, db);
        }
        "keys" => {
            keys_action(split_string, db);
        }
        "exit" => {
            println!("Bye!!!");
            exit(0);
        }
        _ => {
            println!("Unknown command");
        }
    }
}
