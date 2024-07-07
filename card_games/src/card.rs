use std::fmt;
use std::fmt::{Display, Formatter};

use strum_macros::EnumIter;

#[derive(PartialEq, Clone)]
pub struct Card {
    pub level: i8,
    pub suit: Suit,
}

impl Card {
    pub fn new(level: i8, color: Suit) -> Self {
        Card { level, suit: color }
    }

    pub fn get_level(&self) -> i8 {
        self.level
    }

    pub fn get_suit(&self) -> Suit {
        self.suit
    }

    pub fn get_drawing(&self) -> String {
        match self.level {
            1 => format!("A{}", self.suit),
            11 => format!("J{}", self.suit),
            12 => format!("Q{}", self.suit),
            13 => format!("K{}", self.suit),
            _ => format!("{}{}", self.level, self.suit),
        }
    }

    pub fn can_play_on(&self, other: &Card) -> bool {
        self.suit == other.suit || self.level == other.level
    }

    pub fn get_punishment(&self) -> usize {
        match self.level {
            2 => 2,
            _ => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Suit::Diamonds => write!(f, "♦"),
            Suit::Clubs => write!(f, "♣"),
            Suit::Hearts => write!(f, "♥"),
            Suit::Spades => write!(f, "♠"),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_getters() {
        let card = super::Card::new(1, super::Suit::Diamonds);
        assert_eq!(1, card.get_level());
        assert_eq!(super::Suit::Diamonds, card.get_suit());
    }

    #[test]
    fn test_get_drawing_ace() {
        let card = super::Card::new(1, super::Suit::Diamonds);
        assert_eq!("A♦", card.get_drawing());
    }
}
