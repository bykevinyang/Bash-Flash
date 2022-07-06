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

        let title = doc["Title"][0].as_str();
        match title {
            None => {
                return Err("Incorrect File Setup. No Title".to_string());
            }
            Some(x) => x
        };

        let subject = doc["Subject"][0].as_str();
        match subject {
            None => {
                return Err("Incorrect File Setup. No Subject".to_string());
            }
            Some(x) => x
        };
        println!("{:?}", title);
        return Ok(Set {
            title: "hi".to_string(),
            subject: "joe".to_string(),
            cards: CardStack::default()
        })
    }
}