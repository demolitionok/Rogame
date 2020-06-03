use serde::{Serialize, Deserialize};
use std::fs;


#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    resolution: (i32, i32)
}

impl Settings {
    pub fn from_file(path : String) -> Self {
        let json = fs::read_to_string(path).expect("Cant read json from file");
        Settings::from_json(json)
    }
    pub fn from_json(json: String) -> Self {
        let settings : Settings = serde_json::from_str(&json).unwrap();
        settings
    }
}

pub trait WithSettings {
    fn run(settings : &mut Settings);
}