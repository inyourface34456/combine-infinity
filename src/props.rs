use crate::get_unix_epoch;
use std::collections::HashMap;
use crate::Combo;

// #[derive(Eq, PartialE)]
pub struct Proposal {
    combo: (String, String),
    result: HashMap<String, u32>,
    expire: u64
}

impl Proposal {
    pub fn new(combo: (String, String)) -> Self {
        Self {
            combo,
            result: HashMap::new(),
            expire: get_unix_epoch() + 3600
        }
    }

    pub fn matches(&self, combo: &(String, String)) -> bool {
        &self.combo == combo
    }

    pub fn finalize(self) -> Combo {
        let mut max = ("".into(), 0);
        
        for i in self.result {
            if i.1 > max.1 {
                max = i;
            }
        }

        Combo::new(self.combo, max.0)
    }
}