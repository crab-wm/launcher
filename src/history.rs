use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use crate::consts::*;

#[derive(Serialize, Deserialize)]
pub struct History {
    pub stack: HashMap<String, u64>
}

impl History {
    pub fn new() -> Self {
        let data_dir = dirs::data_local_dir().unwrap();
        let data_dir = data_dir.to_str().unwrap();

        fs::create_dir_all(format!("{}{}", data_dir, DATA_DIR)).unwrap();

        match fs::File::open(format!("{}{}", data_dir, DATA_HISTORY_TEMP_FILE)) {
            Ok(file) => serde_json::from_reader(file).unwrap(),
            _ => Self { stack: HashMap::new() }
        }
    }

    pub fn save(&self) {
        let data_dir = dirs::data_local_dir().unwrap();
        let data_dir = data_dir.to_str().unwrap();

        fs::create_dir_all(format!("{}{}", data_dir, DATA_DIR)).unwrap();

        let mut file = fs::File::create(format!("{}{}", data_dir, DATA_HISTORY_TEMP_FILE)).unwrap();
        file.write_all(serde_json::to_string(self).unwrap().as_bytes()).unwrap();
    }

    pub fn update(&mut self, id: String) {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        self.stack.insert(id, time.as_secs());
    }
}