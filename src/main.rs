pub mod cli;
pub mod core;

use crate::core::action::{get, init_db, set};
use crate::core::extras::calculate_hash;

fn main() {
    // let mut db = init_db();
    // set(&mut db, "Test".to_string(), "Test Data".to_string());
    // let data = get(&db, "Test".to_string());
    // println!("{}", data);

    let hash = calculate_hash(&"Test String".to_string());
    println!("{}", hash);
}
