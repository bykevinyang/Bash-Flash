use std::{fs, any::type_name};

use crate::card::{Card, CardStack};
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};

// Study Set Struct
#[derive(Debug)]
pub struct Set {
    pub title: String,
    pub subject: String,
    pub cards: CardStack
}

impl Set {
    pub fn add_card(&mut self, card: Card) {
        self.cards.add_card_to_stack(card);
    }

    pub fn remove_card(&mut self, term: String) {
        // self.cards.remove();
    }

    pub fn read_yml(file_name: String) -> Result<Set, String> {
        let raw = fs::read_to_string(format!("sets/{}.yml", file_name)).expect("smth went wrong when reading");
        let docs = YamlLoader::load_from_str(&raw).unwrap();
        let doc = &docs[0];

        let title = doc["Title"].as_str();

        match title {
            None => {
                return Err("Incorrect File Setup. No Title".to_string());
            }
            Some(x) => x
        };

        let subject = doc["Subject"].as_str();
        match subject {
            None => {
                return Err("Incorrect File Setup. No Subject".to_string());
            }
            Some(x) => x
        };
        
        let cards = doc["Cards"].as_hash();
        match cards {
            None => {
                return Err("No cards to be found!".to_string());
            }
            Some(x) => x
        };

        println!("{:?}", cards);
        let mut stack = CardStack::default();

        for c in cards {
            // println!("{:?}", c);
            let keys = c.keys();
            
            for key in keys {
                let raw_term = key.as_str();
                let raw_def = c[key].as_str();

                let term = match raw_term {
                    None => {
                        return Err(format!("Incorrect formatting for term."));
                    }
                    Some(x) => x
                };

                let definition = match raw_def {
                    None => {
                        return Err(format!("Incorrect formatting for definition at term: {}", raw_term.unwrap()));
                    }
                    Some(x) => x
                };
                
                let card = Card::build_card(term.to_string(), definition.to_string());

                stack.add_card_to_stack(card);
            }
        }

        return Ok(Set {
            title: title.unwrap().to_string(),
            subject: subject.unwrap().to_string(),
            cards: stack
        })
    }
}