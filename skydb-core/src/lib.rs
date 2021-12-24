pub mod dbcore;

use std::{fs::File, io::Write, ops::Add};

use crate::dbcore::model::DB;

pub fn init_db() -> DB {
    let db = DB::init();
    check_init(db)
}

fn check_init(db: DB) -> DB {
    let state = &db.state;
    let model = &state[0].state_model;
    let hash = model[0].hash;
    if hash == 0 {
        println!("Database initialized!")
    }
    db
}

pub fn set(db: &mut DB, key: String, data: String) -> bool {
    db.set(key, data)
}

pub fn get(db: &DB, key: String) -> String {
    db.get(key)
}

pub fn get_hash(db: &DB, key: String) -> u64 {
    db.get_hash(key)
}

pub fn keys(db: &DB) -> Vec<String> {
    db.keys()
}

pub fn store(db: &DB) {
    let data = db.get_all();
    let name = db.get_state_name();
    let mut file = File::create(name.add(".sky")).unwrap();
    for i in &data {
        let data = i.to_string();
        let _ = writeln!(file, "{}", data);
    }
}

pub fn read(db: &mut DB) {
    db.bulk_set();
}

pub fn clear(db: &mut DB) {
    db.clear();
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::path::Path;

    use crate::dbcore::extras::calculate_hash;
    use crate::dbcore::model::Model;

    #[test]
    fn db_initializing_correctly() {
        let db = super::init_db();
        let state = &db.state;
        let model = &state[0].state_model;
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

    #[test]
    fn write_to_file_working() {
        let mut db = super::init_db();
        super::set(&mut db, "Test".to_string(), "Test Data".to_string());
        super::store(&db);
        assert!(Path::new("default.sky").exists());
    }

    #[test]
    fn read_from_file_working() {
        let mut db = super::init_db();
        super::set(&mut db, "Test".to_string(), "Test Data".to_string());
        super::set(&mut db, "Test2".to_string(), "Test Data2".to_string());
        super::store(&db);
        super::clear(&mut db);
        super::read(&mut db);
        let data1 = super::get(&db, "Test".to_string());
        let data2 = super::get(&db, "Test2".to_string());
        fs::remove_file("default.sky").unwrap();
        assert_eq!(data1, "Test Data".to_string());
        assert_eq!(data2, "Test Data2".to_string());
    }

    #[test]
    fn test_clear() {
        let mut db = super::init_db();
        super::set(&mut db, "Test".to_string(), "Test Data".to_string());
        super::set(
            &mut db,
            "Test Hash".to_string(),
            "Test Hash Data".to_string(),
        );
        super::clear(&mut db);
        let state = &db.state;
        let model = &state[0].state_model;
        let hash = model[0].hash;
        if hash != 0 {
            panic!("Database not cleared!")
        }
    }
}
