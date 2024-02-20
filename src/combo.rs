use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Combo {
    combo: (String, String),
    pub result: String,
}

impl Combo {
    pub fn new(combo: (String, String), result: String) -> Self {
        Self { combo, result }
    }

    pub fn matches(&self, combo: &(String, String)) -> bool {
        &self.combo == combo
    }

    pub fn get_result(&self) -> &String {
        &self.result
    }
}
