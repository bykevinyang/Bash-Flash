use core::fmt;

use rand::seq::SliceRandom;
use rand::thread_rng;

// #[derive(Debug, Clone)]
// enum Answer {
//     Correct,
//     Miss,
// }

#[derive(Clone)]
pub struct Card {
    pub term: String,
    pub definition: String,
    pub statistics: CardStatistics,
}

impl Card {
    pub fn build_card(term: String, definition: String) -> Card {
        return Card {
            term: term,
            definition: definition,
            statistics: CardStatistics::build_card_statistics(),
        }
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.term, self.definition)
    }
}

#[derive(Debug, Clone)]
pub struct CardStatistics {
    correct: i32,
    misses: i32,
    misses_in_a_row: i32,
}

impl CardStatistics {
    pub fn build_card_statistics() -> CardStatistics {
        return CardStatistics { correct: 0, misses: 0, misses_in_a_row: 0 }
    }
}

type Cards = Vec<Card>;

// impl fmt::Display for Cards {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!()
//     }
// }

#[derive(Debug)]
pub struct CardStack {
    pub stack: Cards,         // Reference stack. Should never be touched past creation. 
                            // For Reference Use Only!

    pub hand: Cards,       // Cards in "play". This is what the user goes through.
    pub correct: Cards,     // Transferred cards from self.cards when answered correctly
    pub missed: Cards       // Transferred cards from self.cards when answered wrong
}

impl CardStack {
    pub fn build_cardstack(cards: Vec<Card>) -> CardStack {
        return CardStack {
            stack: cards.clone(),
            hand: cards.clone(),
            correct: vec![],
            missed: vec![],
        }
    }
    
    pub fn add_card_to_stack(&mut self, card: Card) {
        self.stack.push(card.clone());
        self.hand.push(card.clone());
    }

    pub fn shuffle(&mut self) {
        self.reset();
        let mut rng = thread_rng();
        self.hand.shuffle(&mut rng);
    }

    pub fn reset(&mut self) {
        self.hand = self.stack.to_owned();
    }

    pub fn add_correct(&mut self, card: Card) {
        self.correct.push(card)
    }

    pub fn add_correct_from_top(&mut self) {
        if self.hand.len() != 0 {
            self.correct.push(self.hand.pop().unwrap())
        }
    }

    pub fn add_miss(&mut self, card: Card) {
        self.missed.push(card)
    }

    pub fn add_miss_from_top(&mut self) {
        if self.hand.len() != 0 {
            self.missed.push(self.hand.pop().unwrap());
        }
    }

    pub fn pop(&mut self) -> Option<Card> {
        return self.hand.pop()
    }
}

impl Default for CardStack {
    fn default() -> Self {
        CardStack { stack: [].to_vec(), hand: [].to_vec(), correct: [].to_vec(), missed: [].to_vec() }
    }
}

impl fmt::Display for CardStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
        "CardStack:
        - Stack:
            {:?}
        - Hand:
            {:?}
        - Correct:
            {:?}
        - Missed:
            {:?}", self.stack, self.hand, self.correct, self.missed)
    }
}
