use std::collections::HashMap;

// Study Set Struct
#[derive(Debug)]
pub struct Set {
    pub title: String,
    pub subject: String,
    pub cards: HashMap<i64, card::Card>
}

impl Set {
    pub fn add_card(&mut self, term: String, definition: String) {
        self.cards.insert(
            term.to_string(),
            definition.to_string(),
        );
    }

    pub fn remove_card(&mut self, term: String) {
        self.cards.remove(&term);
    }
}