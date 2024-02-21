use serde::Serialize;

use crate::get_unix_epoch;
use crate::{Combo, VOTE_EXPIRE};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Serialize, Debug)]
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
            expire: get_unix_epoch() + VOTE_EXPIRE,
        }
    }

    pub fn matches(&self, combo: &(String, String)) -> bool {
        let combo_lr = &(combo.1.clone(), combo.0.clone());

        &self.combo == combo || &self.combo == combo_lr
    }

    pub fn finalize(&self) -> Option<Combo> {
        if self.expire < get_unix_epoch() {
            let mut max = (&String::new(), 0);

            for i in &self.result {
                if i.1 > &max.1 {
                    max = (i.0, *i.1);
                }
            }

            Some(Combo::new(self.combo.clone(), max.0.to_string()))
        } else {
            None
        }
    }
    
    pub fn vote_for(&mut self, prop: &String) {
        match self.result.get_mut(prop) {
            Some(dat) => {
                *dat += 1;
                dbg!(&self);
                //println!("incrmenting exsisting vote {} to value {}", prop, dat);
            }
            None => {
                let _result = self.result.insert(prop.clone(), 1);
                dbg!(&self);
                //println!("making new entry, name is {}, result is {:?}", prop, result);
            }
        };
    }

    pub fn get_options(&self) -> HashMap<String, u32> {
        self.result.clone()
    }
}
