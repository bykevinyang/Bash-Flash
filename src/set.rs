use crate::card::{Card, CardStack};

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
}