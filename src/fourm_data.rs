use serde::Deserialize;

#[derive(Deserialize)]
pub struct Combonation {
    left: String,
    right: String,
}

// unsafe impl std::marker::Send for Combonation {}

impl Combonation {
    pub fn to_tuple(&self) -> (String, String) {
        (self.left.clone(), self.right.clone())
    }
}

#[derive(Deserialize)]
pub struct Vote {
    left: String,
    right: String,
    result: String,
}

// unsafe impl std::marker::Send for Vote {}

impl Vote {
    pub fn to_tuple(&self) -> (String, String) {
        (self.left.clone(), self.right.clone())
    }

    pub fn result(&self) -> String {
        self.result.clone()
    }
}