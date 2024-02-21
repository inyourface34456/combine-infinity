use crate::utils::to_outer;
use crate::{Combo, Errors, Outer, Proposal};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Combos {
    elements: Outer<Vec<String>>,
    combos: Outer<Vec<Combo>>,
    props: Outer<Vec<Proposal>>,
}

impl Combos {
    pub fn new() -> Self {
        Self {
            elements: to_outer(vec![
                "earth".into(),
                "fire".into(),
                "air".into(),
                "water".into(),
            ]),
            combos: to_outer(vec![]),
            props: to_outer(vec![]),
        }
    }

    pub fn combine(&self, combo: (String, String)) -> Result<String, Errors> {
        if let Ok(elements) = self.elements.read() {
            if !(elements.contains(&combo.0) || elements.contains(&combo.1)) {
                return Err(Errors::ElementDoesNotExist);
            }
            if &combo.0.len() > &64 || &combo.1.len() > &64 {
                return Err(Errors::NameTooLarge);
            }
        }

        if let Ok(combos) = self.combos.read() {
            for i in combos.clone() {
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
            for i in props.clone() {
                if let Some(prop) = i.finalize() {
                    if let Ok(mut combos) = self.combos.write() {
                        combos.push(prop.clone());
                        if let Ok(mut elements) = self.elements.write() {
                            elements.push(prop.result.clone())
                        }
                    }
                    let pos = props.iter().position(|x| x == &i).unwrap();
                    props.remove(pos);
                }
            }
        }

        if let Ok(mut combos) = self.combos.write() {
            combos.dedup_by(|x, y| x == y)
        }
    }

    pub fn vote(&self, combo: (String, String), result: String) -> (String, HashMap<String, u32>) {
        let result = result.to_lowercase();

        if let Ok(mut props) = self.props.write() {
            // props.dedup();

            for mut i in props.clone() {
                if i.matches(&combo) {
                    i.vote_for(&result);
                    return (result, i.get_options());
                }
            }
            let combo = (combo.0.to_lowercase(), combo.1.to_lowercase());

            let mut prop = Proposal::new(combo);
            prop.vote_for(&result);
            props.push(prop.clone());
            (result, prop.get_options())
        } else {
            (String::new(), HashMap::new())
        }
    }
}