use std::cmp::max;

use crate::card::Card;
use crate::pile::Pile;
use crate::player::{self, Player};
use crate::players::Players;

pub struct MauMau {
    players: Players,

    stock: Pile,
    played: Pile,

    next_punishment: usize,
}

impl MauMau {
    pub fn new() -> MauMau {
        MauMau {
            players: Players::new(),

            stock: Pile::with_deck(),
            played: Pile::new(),

            next_punishment: 0,
        }
    }

    pub fn deal(&mut self, number_of_cards: usize) {
        for player in self.players.players.iter_mut() {
            player.deal(&mut self.stock, number_of_cards);
        }
    }

    pub fn add_player(&mut self, name: String) {
        self.players.add_player(Player::new(&name));
    }

    pub fn setup(&mut self) {
        self.stock.add_deck();
        self.deal(7);
    }

    pub fn start(&mut self) {
        loop {
            let card_index = self.players.get_current_player_mut().select_card();

            // Skip turn
            if card_index == 0 {
                self.players
                    .get_current_player_mut()
                    .deal(&mut self.stock, max(1, self.next_punishment));
                self.next_punishment = 0;
                self.players.next_round();
                continue;
            }
            let card = self
                .players
                .get_current_player()
                .get_hand()
                .get_cards()
                .get(card_index)
                .unwrap();

            // Played invalid card
            if !self.can_play_card(card) {
                continue;
            }

            let card = self
                .players
                .get_current_player_mut()
                .get_hand_mut()
                .take_card(card_index)
                .unwrap();

            self.play_card(card);

            self.players.next_round();
        }
    }

    fn can_play_card(&self, card: &Card) -> bool {
        let top_card = self.played.view_top();
        if top_card.is_none() {
            return true;
        }
        let top_card = top_card.unwrap();
        if !card.can_play_on(top_card) {
            return false;
        }
        if self.next_punishment > 0 && card.get_punishment() == 0 {
            return false;
        }
        return true;
    }

    pub fn play_card(&mut self, card: Card) {
        self.next_punishment += card.get_punishment();
        self.played.add_card(card);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_game() {
        let mut game = MauMau::new();
        game.add_player("Piet".to_owned());
        game.add_player("Jan".to_owned());
        game.add_player("Klaas".to_owned());

        game.setup();

        let player1 = game.players.get_next_player_at(0);
        let player2 = game.players.get_next_player_at(1);
        let player3 = game.players.get_next_player_at(1);
        assert_eq!(player1.get_hand().get_cards().len(), 7);
        assert_eq!(player2.get_hand().get_cards().len(), 7);
        assert_eq!(player3.get_hand().get_cards().len(), 7);
    }
}
