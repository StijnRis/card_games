use crate::pile::Pile;
use crate::player::Player;
use crate::players::Players;
use crate::settings::Settings;

pub struct MauMau {
    settings: Settings,
    players: Players,

    stock: Pile,
    played: Pile,
}

impl MauMau {
    pub fn new() -> MauMau {
        MauMau {
            settings: Settings::new(),
            players: Players::new(),

            stock: Pile::with_deck(),
            played: Pile::new(),
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
            let current_player = self.players.get_current_player_mut();
            let card = current_player.select_card();

            if card.is_some() {
                self.played.add_card(card.unwrap());
            } else {
                break;
            }

            self.players.next_round();
        }
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
