#[derive(Clone)]
pub struct Combo {
    combo: (String, String),
    result: String
}

impl Combo {
    pub fn new(combo: (String, String), result: String) -> Self {
        Self {combo, result}
    }

    pub fn matches(&self, combo: &(String, String)) -> bool {
        &self.combo == combo
    }

    pub fn get_result(&self) -> &String {
        &self.result
    }
}

