use crate::game::{Game, GameWithStock};

pub trait Action<G: Game> {
    fn execute(&self, game: &mut G);
}

pub struct TakeCards {
    pub amount: usize,
    pub next_player: usize,
}

impl<G> Action<G> for TakeCards
where
    G: GameWithStock,
{
    fn execute(&self, game: &mut G) {
        let cards_to_add = game.get_stock_mut().take_top_cards(self.amount);
        let player = game
            .get_players_mut()
            .get_next_player_at_mut(self.next_player);
        let hand = player.get_hand_mut();
        hand.add_cards(cards_to_add);
    }
}

#[cfg(test)]
mod tests {
    use crate::action::{Action, TakeCards};
    use crate::game::{Game, GameWithStock};
    use crate::mau_mau::MauMau;
    use crate::player::Player;

    #[test]
    fn take_two_cards() {
        let mut game = MauMau::new();
        game.get_stock_mut().add_deck();
        let players = game.get_players_mut();
        let player1 = Player::new("Piet");
        players.add_player(player1);
        let player2 = Player::new("Jan");
        players.add_player(player2);
        let player3 = Player::new("Klaas");
        players.add_player(player3);

        let action = TakeCards {
            amount: 2,
            next_player: 1,
        };
        action.execute(&mut game);

        let player1 = game.get_players().get_next_player_at(0);
        let player2 = game.get_players().get_next_player_at(1);
        assert_eq!(player1.get_hand().get_cards().len(), 0);
        assert_eq!(player2.get_hand().get_cards().len(), 2);
    }
}
