use std::io;

use crate::card::{Card, Suit};
use strum::IntoEnumIterator;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Pile {
    cards: Vec<Card>,
}

impl Pile {
    pub fn new() -> Pile {
        Pile { cards: vec![] }
    }

    pub fn with_deck() -> Pile {
        let mut pile = Pile::new();
        pile.add_deck();
        pile
    }

    pub fn add_deck(&mut self) {
        let mut cards = Vec::with_capacity(52);

        for color in Suit::iter() {
            for level in 1..=13 {
                let card = Card { level, suit: color };
                cards.push(card);
            }
        }
        cards.shuffle(&mut thread_rng());

        self.cards.extend(cards);
    }

    pub fn take_cards_from(&mut self, deck: &mut Pile, amount: usize) {
        self.add_cards(deck.take_top_cards(amount));
    }

    pub fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn view_top(&self) -> Option<&Card> {
        self.cards.last()
    }

    pub fn take_card(&mut self, index: usize) -> Option<Card> {
        if index < self.cards.len() {
            Some(self.cards.remove(index))
        } else {
            None
        }
    }

    pub fn take_top_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn take_top_cards(&mut self, amount: usize) -> Vec<Card> {
        let start_index = self.cards.len().saturating_sub(amount);
        let cards = self.cards.drain(start_index..).collect();

        cards
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_cards(&mut self, cards: Vec<Card>) {
        self.cards.extend(cards);
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }

    pub fn get_drawing(&self) -> String {
        let mut drawing = String::new();
        for card in &self.cards {
            drawing.push_str(&card.get_drawing());
            drawing.push_str(" ");
        }
        drawing
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn select_card(&mut self) -> Option<Card> {
        println!("Your hand: {}", self.get_drawing());
        println!("Select a card to play by typing its index: ");
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        let index: usize = input_line.trim().parse().expect("Input not an integer");
        self.take_card(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        card::{Card, Suit},
        pile::Pile,
    };

    #[test]
    fn test_add_deck() {
        let mut cards = Pile::new();
        cards.add_deck();

        assert_eq!(52, cards.cards.len());
    }

    #[test]
    fn test_get_drawing() {
        let mut cards = Pile::new();
        cards.add_card(Card::new(1, Suit::Hearts));
        cards.add_card(Card::new(2, Suit::Hearts));
        cards.add_card(Card::new(3, Suit::Hearts));

        assert_eq!("A♥ 2♥ 3♥ ", cards.get_drawing());
    }
}
