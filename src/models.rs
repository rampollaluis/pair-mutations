use std::collections::{HashMap, HashSet};

pub struct Data {
    pub history: HashMap<String, Vec<Vec<String>>>,
    pub members: HashSet<String>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            history: HashMap::new(),
            members: HashSet::new(),
        }
    }

    pub fn add_entry(&mut self, date: String, pairs: Vec<Vec<String>>) {
        self.history.insert(date, pairs);
    }
    
    pub fn add_member(&mut self, member: String) {
        self.members.insert(member);
    }
}