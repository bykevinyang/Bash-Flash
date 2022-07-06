use std::collections::HashMap;

use card::CardStack;
use set::Set;

use crate::card:: {CardStatistics, Card};

mod bash_flash;
mod card;
mod set;

fn main() {
    bash_flash::start::run();
    // let mut set = set::Set {
    //     title: "Les Mots De Liason".to_string(),
    //     subject: "French".to_string(),
    //     cards: CardStack::build_cardstack(vec![])
    // };
    Set::read_yml("test".to_string());
    // let card1 = Card::build_card("Joe".to_string(), "Mama".to_string());
    // let card2 = Card::build_card("term".to_string(), "defi".to_owned());
    // let card3 = Card::build_card("Hi".to_string(), "there!".to_string());
    // let card4 = Card::build_card("Why".to_string(), "leave me alone".to_string());
    // let mut cardStack = CardStack::build_cardstack(vec![]);
    // cardStack.add_card_to_stack(card1);
    // cardStack.add_card_to_stack(card2);
    // cardStack.add_card_to_stack(card3);
    // cardStack.add_card_to_stack(card4);
    // cardStack.add_correct_from_top();
    // cardStack.add_miss_from_top();
    // println!("{}", cardStack);
    // cardStack.shuffle();
    // println!("{:?}", cardStack.pop());
}