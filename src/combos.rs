use crate::{Combo, Errors, Outer, Proposal};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Combos {
    elements: Outer<Vec<String>>,
    combos: Outer<Vec<Combo>>,
    props: Outer<Vec<Proposal>>,
}

impl Combos {
    pub fn new() -> Self {
        Self {
            elements: Arc::new(RwLock::new(vec![
                "earth".into(),
                "air".into(),
                "fire".into(),
                "water".into(),
            ])),
            combos: Arc::new(RwLock::new(vec![])),
            props: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn combine(&self, combo: (String, String)) -> Result<String, Errors> {
        if let Ok(combos) = self.combos.read() {
            for i in combos.clone().into_iter() {
                if i.matches(&combo) {
                    return Ok(i.get_result().clone());
                }
            }

            Err(Errors::ProposalExsits)
        } else {
            Err(Errors::InternelServerError)
        }
    }

    pub fn clean(&self) {
        if let Ok(mut props) = self.props.write() {
            let mut to_remove = vec![];
            for i in props.clone().into_iter() {
                if let Some(prop) = i.finalize() {
                    if let Ok(mut combos) = self.combos.write() {
                        combos.push(prop);
                    }
                    to_remove.push(i.clone());
                }
            }

            for i in to_remove {
                let pos = props.iter().position(|x| x == &i).unwrap();
                props.remove(pos);
            }
        }
    }

    // pub fn new_prop(&self, combo: (String, String)) {
    //     if let Ok(mut props) = self.props.write() {
    //         for i in props.
    //     }
    // }
}
