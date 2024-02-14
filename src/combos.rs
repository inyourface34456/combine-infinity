use std::sync::{Arc, RwLock};
use crate::{Proposal, Outer, Combo};

#[derive(Clone)]
pub struct Combos {
    elements: Outer<Vec<String>>,
    combos: Outer<Vec<Combo>>,
    props: Outer<Vec<Proposal>>
}

impl Combos {
    pub fn new() -> Self {
        Self {
            elements: Arc::new(RwLock::new(vec!["earth".into(), "air".into(), "fire".into(), "water".into()])),
            combos: Arc::new(RwLock::new(vec![])),
            props: Arc::new(RwLock::new(vec![]))
        }
    }

    pub fn combine(&self, combo: (String, String)) -> Result<String, String> {
        if let Ok(combos) = self.combos.read() {
            for i in combos.clone().into_iter() {
                if i.matches(&combo) {
                    return Ok(i.get_result().clone());
                }
            }

            return Err("prop in progress".into())
        } else {
            Err("internel server error".into())
        }
    }

    // pub fn new_prop(&self, combo: (String, String)) {
    //     if let Ok(mut props) = self.props.write() {
    //         for i in props.
    //     }
    // }
}