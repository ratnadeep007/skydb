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
        let state = &mut self.state;
        let state_model = &mut state.state_model;
        let model = Model::new(key, data);
        let _ = &state_model.model.push(model);
        true
    }

    pub fn get(&self, key: String) -> String {
        let state = &self.state;
        let state_model = &state.state_model;
        let model = &state_model.model;
        let mut result = String::new();
        for item in model {
            if item.key == key {
                result = item.data.clone();
            }
        }
        result
    }
}

pub struct State {
    pub name: String,
    pub state_model: StateModel,
}

impl State {
    pub fn new(name: Option<String>) -> State {
        State {
            name: name.unwrap_or("default".to_string()),
            state_model: StateModel::init(),
        }
    }
}

pub struct StateModel {
    pub model: Vec<Model>,
    pub hash: u64,
}

impl StateModel {
    pub fn new() -> StateModel {
        StateModel {
            model: Vec::new(),
            hash: 0,
        }
    }

    pub fn init() -> StateModel {
        StateModel {
            model: vec![Model::init()],
            hash: 0,
        }
    }
}

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
