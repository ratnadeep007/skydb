extern crate skydbcore;

use skydbcore::dbcore::model::DB;
use skydbcore::get as get_action;
use skydbcore::get_hash as get_hash_action;
use skydbcore::keys as keys_action;
use skydbcore::read as read_action;
use skydbcore::set as set_action;
use skydbcore::store as write_action;

pub fn set(split_string: Vec<&str>, db: &mut DB) {
    if split_string.len() == 3 {
        let key = split_string[1];
        let data = split_string[2];
        set_action(db, key.to_string(), data.to_string());
        println!("1 insert successful");
    } else {
        println!("Usage: set <key> <data>");
    }
}

pub fn get(split_string: Vec<&str>, db: &mut DB) {
    if split_string.len() == 2 {
        let key = split_string[1];
        let data = get_action(db, key.to_string());
        println!("{}", data);
    } else {
        println!("Usage: get <key>");
    }
}

pub fn get_hash(split_string: Vec<&str>, db: &mut DB) {
    if split_string.len() == 2 {
        let key = split_string[1];
        let hash = get_hash_action(db, key.to_string());
        println!("{}", hash);
    } else {
        println!("Usage: get_hash <key>");
    }
}

pub fn keys(split_string: Vec<&str>, db: &mut DB) {
    if split_string.len() == 1 {
        let keys = keys_action(db);
        println!("{:?}", keys);
    } else {
        println!("Usage: keys");
    }
}

pub fn write(split_string: Vec<&str>, db: &mut DB) {
    if split_string.len() == 1 {
        write_action(db);
        println!("Write successful");
    } else {
        println!("Usage: write");
    }
}

pub fn read(split_string: Vec<&str>, db: &mut DB) {
    if split_string.len() == 1 {
        read_action(db);
        println!("Read successful");
    } else {
        println!("Usage: read");
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use skydbcore::dbcore::extras::calculate_hash;
    use skydbcore::dbcore::model::Model;

    #[test]
    fn test_set() {
        use super::*;
        let mut db = DB::init();
        set(vec!["set", "key", "data"], &mut db);
        assert_eq!(db.get("key".to_string()), "data");
    }

    #[test]
    fn test_get() {
        use super::*;
        let mut db = DB::init();
        set(vec!["set", "key", "data"], &mut db);
        get(vec!["get", "key"], &mut db);
        assert_eq!(db.get("key".to_string()), "data");
    }

    #[test]
    fn test_get_hash() {
        use super::*;
        let mut db = DB::init();
        set(vec!["set", "key", "data"], &mut db);
        get_hash(vec!["get_hash", "key"], &mut db);
        let model = Model::new("key".to_string(), "data".to_string());
        assert_eq!(db.get_hash("key".to_string()), calculate_hash(&model));
    }

    #[test]
    fn test_keys() {
        use super::*;
        let mut db = DB::init();
        set(vec!["set", "key", "data"], &mut db);
        keys(vec!["keys"], &mut db);
        assert_eq!(db.keys(), vec!["key".to_string()]);
    }

    #[test]
    fn test_write() {
        use super::*;
        let mut db = DB::init();
        set(vec!["set", "key", "data"], &mut db);
        write(vec!["write"], &mut db);
        assert!(Path::new("default.sky").exists());
    }

    #[test]
    fn test_read() {
        use super::*;
        let mut db = DB::init();
        set(vec!["set", "key", "data"], &mut db);
        write(vec!["read"], &mut db);
        let data = get_action(&db, "key".to_string());
        assert_eq!(data, "data");
    }
}
