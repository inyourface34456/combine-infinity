use crate::utils::{get_unix_epoch, to_outer, VOTE_EXPIRE};
use crate::{Errors, Outer};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Combos {
    elements: Outer<Vec<String>>,
    combos: Outer<HashMap<(String,String), String>>,
    props: Outer<HashMap<(String,String), (HashMap<String, u32>, u64)>>,
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
            combos: to_outer(HashMap::new()),
            props: to_outer(HashMap::new()),
        }
    }

    pub fn combine(&self, combo: (&String, &String)) -> Result<String, Errors> {
        if let Ok(elements) = self.elements.read() {
            if !(elements.contains(&combo.0) || elements.contains(&combo.1)) {
                return Err(Errors::ElementDoesNotExist);
            }
            if &combo.0.len() > &64 || &combo.1.len() > &64 {
                return Err(Errors::NameTooLarge);
            }
        }

        if let Ok(combos) = self.combos.read() {
            match combos.get(&(combo.0.clone(), combo.1.clone())) {
                Some(dat) => Ok(dat.clone()),
                None => Err(Errors::VotingInProgress)
            }
            
        } else {
            Err(Errors::InternelServerError)
        }
    }

    pub fn clean(&self) {
        if let Ok(mut map) = self.props.write() {
            for i in map.clone() {
                if i.1.1 < get_unix_epoch() {
                    let mut max = (String::new(), 0);

                    for j in i.1.0.clone() {
                        if j.1 > max.1 {
                            max = (j.0, j.1);
                        }
                    }

                    if let Ok(mut combos) = self.combos.write() {
                        combos.insert(i.clone().0, max.0);
                    }

                    map.remove(&i.0);
                }
            }
        }
    }

    pub fn vote(&self, combo: (String, String), result: String) -> (String, HashMap<String, u32>) {
        let result = result.to_lowercase();

        if let Ok(mut props) = self.props.write() {
            match props.get_mut(&combo) {
                Some(dat) => {
                    match dat.0.get_mut(&result) {
                        Some(votes) => {
                            *votes += 1;
                            (result, dat.0.clone())
                        },
                        None => {
                            dat.0.insert(result.clone(), 1);
                            (result, dat.0.clone())
                        }
                    }
                },
                None => {
                    props.insert(combo, (HashMap::from([(result.clone(), 1)]), get_unix_epoch()+VOTE_EXPIRE));
                    (result.clone(), HashMap::from([(result, 1)]))
                }
            }


        } else {
            (String::new(), HashMap::new())
        }
    }
}