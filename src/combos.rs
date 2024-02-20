use crate::{Combo, Errors, Outer, Proposal};
use crate::utils::{SaveFormat, to_outer};
use std::fs;
use std::io::Write;

#[derive(Clone)]
pub struct Combos {
    elements: Outer<Vec<String>>,
    combos: Outer<Vec<Combo>>,
    props: Outer<Vec<Proposal>>,
}

impl Combos {
    pub fn new() -> Self {
        Self {
            elements: to_outer(vec!["earth".into(), "fire".into(), "air".into(), "water".into()]),
            combos: to_outer(vec![]),
            props: to_outer(vec![]),
        }
    }

    pub fn load(path: String) -> Self {
        let data = fs::read_to_string(&path).expect("could not read file");
        let save_data: SaveFormat = serde_json::from_str(&data).expect("save data invalid");

        Self {elements: to_outer(save_data.elements), combos: to_outer(save_data.combos), props: to_outer(vec![])}
    }

    pub fn combine(&self, combo: (String, String)) -> Result<String, Errors> {
        if let Ok(elements) = self.elements.read() {
            if !(elements.contains(&combo.0) || elements.contains(&combo.1)) {
                return Err(Errors::ElementDoesNotExist)
            } 
            if &combo.0.len() > &64 || &combo.1.len() > &64 {
                return Err(Errors::NameTooLarge)
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
            combos.dedup_by(|x, y| {x == y})
        }
    }

    pub fn vote(&self, combo: (String, String), result: String) -> String {
        let result = result.to_lowercase();
        
        if let Ok(mut props) = self.props.write() {
            for mut i in props.clone() {
                if i.matches(&combo) {
                    i.vote_for(&result);
                    return result;
                }
            }
            let combo = (combo.0.to_lowercase(), combo.1.to_lowercase());

            let mut prop = Proposal::new(combo);
            prop.vote_for(&result);
            props.push(prop);
            result
        } else {
            String::new()
        }
    }

    pub fn to_string(&self) -> String {
        let mut output = SaveFormat::new();

        if let Ok(elements) = self.elements.read() {
            output.set_elements(&elements);
        }

        if let Ok(props) = self.props.read() {
            if let Ok(mut combos) = self.combos.write() {
                for i in props.iter() {
                    combos.push(i.finlize_unchecked())
                }
            }
        }

        if let Ok(combos) = self.combos.read() {
            output.set_combos(&combos)
        }

        serde_json::to_string(&output).unwrap()
    }
}

impl Drop for Combos {
    fn drop(&mut self) {
        match fs::OpenOptions::new().write(true).truncate(true).open("data.json") {
            Ok(mut file) => {
                file.write_all(self.to_string().as_bytes()).expect("could not write to file")
            },
            Err(_) => {
                fs::File::create("data.json").expect("could not create file");
                let mut file = fs::OpenOptions::new().write(true).truncate(true).open("data.json").expect("could not open file");
                file.write_all(self.to_string().as_bytes()).expect("could not write to file")
            }
        }
    }
}