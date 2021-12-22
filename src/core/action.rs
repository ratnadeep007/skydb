use crate::core::model::*;

use super::model;

pub fn init_db() -> DB {
    let db = DB::init();
    check_init(db)
}

fn check_init(db: DB) -> DB {
    let state = &db.state;
    let model = &state.state_model;
    let hash = model[0].hash;
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

pub fn get_hash(db: &model::DB, key: String) -> u64 {
    db.get_hash(key)
}

#[cfg(test)]
mod test {
    use crate::{calculate_hash, core::model::Model};

    #[test]
    fn db_initializing_correctly() {
        let db = super::init_db();
        let state = &db.state;
        let model = &state.state_model;
        let hash = model[0].hash;
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

    #[test]
    fn get_returns_empty_string_if_key_not_found() {
        let db = super::init_db();
        let data = super::get(&db, "Test".to_string());
        assert_eq!(data, "".to_string());
    }

    #[test]
    fn set_adds_correct_hash() {
        let mut db = super::init_db();
        super::set(&mut db, "Test".to_string(), "Test Data".to_string());
        super::set(
            &mut db,
            "Test Hash".to_string(),
            "Test Hash Data".to_string(),
        );
        let model = Model::new("Test Hash".to_string(), "Test Hash Data".to_string());
        let hash = super::get_hash(&db, "Test Hash".to_string());
        assert_eq!(hash, calculate_hash(&model));
    }
}
