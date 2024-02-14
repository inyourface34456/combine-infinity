use crate::get_unix_epoch;
use crate::Combo;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq)]
pub struct Proposal {
    combo: (String, String),
    result: HashMap<String, u32>,
    expire: u64,
}

impl Proposal {
    pub fn new(combo: (String, String)) -> Self {
        Self {
            combo,
            result: HashMap::new(),
            expire: get_unix_epoch() + 3600,
        }
    }

    pub fn matches(&self, combo: &(String, String)) -> bool {
        &self.combo == combo
    }

    pub fn finalize(&self) -> Option<Combo> {
        if self.expire < get_unix_epoch() {
            let mut max = (&"".into(), 0);

            for i in self.result.iter() {
                if *i.1 > max.1 {
                    max = (i.0, *i.1);
                }
            }

            Some(Combo::new(self.combo.clone(), max.0.clone()))
        } else {
            None
        }
    }
}
