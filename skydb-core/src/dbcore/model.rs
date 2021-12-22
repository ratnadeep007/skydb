use crate::dbcore::extras::calculate_hash;

pub struct DB {
    pub state: State,
}

impl DB {
    pub fn init() -> DB {
        DB {
            state: State::new(None),
        }
    }

    pub fn set(&mut self, key: String, data: String) -> bool {
        let model = Model::new(key, data);
        let hash = calculate_hash(&model);
        let new_state_model = StateModel::new(model, hash);
        let _ = &self.state.state_model.push(new_state_model);
        true
    }

    pub fn get(&self, key: String) -> String {
        let state_models = &self.state.state_model;
        for model in state_models {
            if model.model.key == key {
                return model.model.data.to_string();
            }
        }
        "".to_string()
    }

    pub fn get_hash(&self, key: String) -> u64 {
        let state_models = &self.state.state_model;
        for model in state_models {
            if model.model.key == key {
                return model.hash;
            }
        }
        0
    }

    pub fn keys(&self) -> Vec<String> {
        let mut keys = Vec::new();
        let state_models = &self.state.state_model;
        for model in state_models {
            if model.model.key.to_string() == "New State" {
                continue;
            }
            keys.push(model.model.key.to_string());
        }
        keys
    }
}

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

#[derive(Hash)]
pub struct Model {
    pub key: String,
    pub data: String,
}

impl Model {
    pub fn new(key: String, data: String) -> Model {
        Model { key, data }
    }

    pub fn init() -> Model {
        Model {
            key: String::from("New State"),
            data: String::from("Nothing to see here"),
        }
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
