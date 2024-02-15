use crate::{Combo, Errors, Outer, Proposal};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Combos {
    combos: Outer<Vec<Combo>>,
    props: Outer<Vec<Proposal>>,
}

impl Combos {
    pub fn new() -> Self {
        // let mut map = HashMap::new();

        // map.insert(("".into(), "".into()), "fire".into());
        // map.insert(("".into(), "".into()), "earth".into());
        // map.insert(("".into(), "".into()), "water".into());
        // map.insert(("".into(), "".into()), "air".into());

        Self {
            //elements: Arc::new(RwLock::new(map)),
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

            Err(Errors::VotingInProgress)
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

    pub fn vote(&self, combo: (String, String), result: String) -> String {
        if let Ok(mut props) = self.props.write() {
            for mut i in props.clone().into_iter() {
                if i.matches(&combo) {
                    i.vote_for(&result);
                    return result;
                }
            }
            let mut prop = Proposal::new(combo);
            prop.vote_for(&result);
            props.push(prop);
            result
        } else {
            "".into()
        }
    }
}
