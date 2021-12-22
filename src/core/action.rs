use crate::core::model::*;

use super::model;

pub fn init_db() -> DB {
    let db = DB::init();
    check_init(db)
}

fn check_init(db: DB) -> DB {
    let state = &db.state;
    let model = &state.state_model;
    let hash = model.hash;
    if hash == 0 {
        println!("Database initialized!")
    }
    db
}

pub fn set(db: &mut model::DB, key: String, data: String) -> bool {
    db.set(key, data)
}

pub fn get(db: &model::DB, key: String) -> String {
    db.get(key)
}
