pub mod cli;
pub mod core;

use crate::core::action::{get, init_db, set};

fn main() {
    let mut db = init_db();
    set(&mut db, "Test".to_string(), "Test Data".to_string());
    let data = get(&db, "Test".to_string());
    println!("{}", data);
}
