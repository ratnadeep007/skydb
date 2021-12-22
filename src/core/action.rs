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

#[cfg(test)]
mod test {
    #[test]
    fn db_initializing_correctly() {
        let db = super::init_db();
        let state = &db.state;
        let model = &state.state_model;
        let hash = model.hash;
        if hash != 0 {
            panic!("Database not initialized!")
        }
    }

    #[test]
    fn set_and_get_correct_value() {
        let mut db = super::init_db();
        super::set(&mut db, "Test".to_string(), "Test Data".to_string());
        let data = super::get(&db, "Test".to_string());
        assert_eq!(data, "Test Data".to_string());
    }
}
