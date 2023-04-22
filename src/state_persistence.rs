use std::{fs::File, io::prelude::*};
use serde::{Serialize, de::DeserializeOwned};

pub fn save_state_to_file<S: Serialize>(state: &S, file_path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(state).unwrap();
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_state_from_file<D: DeserializeOwned>(file_path: &str) -> Result<D, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let state: D = serde_json::from_str(&contents)?;
    Ok(state)
}

