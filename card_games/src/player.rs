use crate::{card::Card, pile::Pile};

pub struct Player {
    name: String,
    hand: Pile,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            hand: Pile::new(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_hand(&self) -> &Pile {
        &self.hand
    }

    pub fn get_hand_mut(&mut self) -> &mut Pile {
        &mut self.hand
    }

    pub fn deal(&mut self, deck: &mut Pile, number_of_cards: usize) {
        self.hand.add_cards(deck.take_top_cards(number_of_cards));
    }

    pub fn select_card(&mut self) -> Option<Card> {
        self.hand.select_card()
    }
}
