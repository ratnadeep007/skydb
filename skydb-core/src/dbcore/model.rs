use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
};

use crate::dbcore::extras::calculate_hash;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DB {
    pub state: Vec<State>,
}

impl DB {
    pub fn init() -> DB {
        DB {
            state: vec![State::new(None)],
        }
    }

    pub fn get_state_name(&self) -> String {
        self.state[0].name.clone()
    }

    pub fn set(&mut self, key: String, data: String) -> bool {
        let model = Model::new(key, data);
        let hash = calculate_hash(&model);
        let new_state_model = StateModel::new(model, hash);
        let _ = &self.state[0].state_model.push(new_state_model);
        true
    }

    pub fn get(&self, key: String) -> String {
        let state_models = &self.state[0].state_model;
        for model in state_models {
            if model.model.key == key {
                return model.model.data.to_string();
            }
        }
        "".to_string()
    }

    pub fn get_hash(&self, key: String) -> u64 {
        let state_models = &self.state[0].state_model;
        for model in state_models {
            if model.model.key == key {
                return model.hash;
            }
        }
        0
    }

    pub fn keys(&self) -> Vec<String> {
        let mut keys = Vec::new();
        let state_models = &self.state[0].state_model;
        for model in state_models {
            if model.model.key.to_string() == "New State" {
                continue;
            }
            keys.push(model.model.key.to_string());
        }
        keys
    }

    pub fn get_all(&self) -> Vec<Model> {
        let mut models = Vec::new();
        let state_models = &self.state[0].state_model;
        for model in state_models {
            if model.model.key.to_string() == "New State" {
                continue;
            }
            models.push(model.model.clone());
        }
        models
    }

    pub fn bulk_set(&mut self) -> bool {
        let file = File::open(self.get_state_name().add(".sky")).unwrap();
        let reader = BufReader::new(file);
        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if line != "" {
                let key_value = line.split("`").collect::<Vec<&str>>();
                let key = key_value[0].to_string();
                let data = key_value[1].to_string();
                self.set(key, data);
            }
        }
        true
    }

    pub fn clear(&mut self) -> bool {
        self.state[0] = State::new(None);
        true
    }
}

#[derive(Debug)]
pub struct State {
    pub name: String,
    pub state_model: Vec<StateModel>,
}

impl State {
    pub fn new(name: Option<String>) -> State {
        State {
            name: name.unwrap_or("default".to_string()),
            state_model: vec![StateModel::init()],
        }
    }
}

#[derive(Debug)]
pub struct StateModel {
    pub model: Model,
    pub hash: u64,
}

impl StateModel {
    pub fn new(model: Model, hash: u64) -> StateModel {
        StateModel { model, hash }
    }

    pub fn init() -> StateModel {
        StateModel {
            model: Model::init(),
            hash: 0,
        }
    }
}

#[derive(Hash, Clone, Deserialize, Serialize, Debug)]
pub struct Model {
    pub key: String,
    pub data: String,
}

impl Model {
    pub fn new(key: String, data: String) -> Model {
        let key = key.replace(|c: char| !c.is_ascii(), "");
        Model { key, data }
    }

    pub fn init() -> Model {
        Model {
            key: String::from("New State"),
            data: String::from("Nothing to see here"),
        }
    }

    pub(crate) fn to_string(&self) -> String {
        format!("{}`{}\n", self.key, self.data)
    }
}

#[cfg(test)]
mod test {
    use crate::dbcore::extras::calculate_hash;

    #[test]
    fn set_and_get_correct_value() {
        let mut db = super::DB::init();
        super::DB::set(&mut db, "Test".to_string(), "Test Data".to_string());
        let data = super::DB::get(&db, "Test".to_string());
        assert_eq!(data, "Test Data".to_string());
    }

    #[test]
    fn get_returns_empty_string_if_key_not_found() {
        let db = super::DB::init();
        let data = super::DB::get(&db, "Test".to_string());
        assert_eq!(data, "".to_string());
    }

    #[test]
    fn check_hash() {
        let mut db = super::DB::init();
        super::DB::set(
            &mut db,
            "Test Hash".to_string(),
            "Test Hash Data".to_string(),
        );
        let model = super::Model::new("Test Hash".to_string(), "Test Hash Data".to_string());
        let hash = super::DB::get_hash(&db, "Test Hash".to_string());
        assert_eq!(hash, calculate_hash(&model));
    }
}
