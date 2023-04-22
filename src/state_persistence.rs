use std::{fs::File, io::prelude::*};
use serde::{Serialize, de::DeserializeOwned};

static STATE_FILE_PATH: &str = "state.json";

pub fn save_state<S: Serialize>(state: &S) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(state).unwrap();
    let mut file = File::create(STATE_FILE_PATH)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_state<D: DeserializeOwned>() -> Result<D, Box<dyn std::error::Error>> {
    let mut file = File::open(STATE_FILE_PATH)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let state: D = serde_json::from_str(&contents)?;
    Ok(state)
}

