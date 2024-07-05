use crate::{card::Card, mau_mau::MauMau, player::Player};

pub trait CardPlayedRule {
    fn execute(&self, game: &mut MauMau, player: &mut Player, card: &Card);
}

pub struct TakeCards {
    pub card: Card,
    pub amount: usize,
    pub next_player: usize,
}

impl CardPlayedRule for TakeCards {
    fn execute(&self, game: &mut MauMau, player: &mut Player, card: &Card) {
        if *card != self.card {
            return;
        }

        let cards_to_add = game.get_stock_mut().take_top_cards(self.amount);
        let player = game
            .get_players_mut()
            .get_ith_player_after_mut(player, self.next_player);
        let hand = player.get_hand_mut();
        hand.add_cards(cards_to_add);
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Card, Suit};
    use crate::mau_mau::MauMau;
    use crate::player::Player;
    use crate::rule::{CardPlayedRule, TakeCards};

    #[test]
    fn take_two_cards() {
        let mut game = MauMau::new();
        game.add_player("Piet".to_owned());
        game.add_player("Jan".to_owned());
        game.add_player("Klaas".to_owned());

        game.setup();

        let action = TakeCards {
            card: Card::new(0, Suit::Hearts),
            amount: 2,
            next_player: 1,
        };
        action.execute(
            &mut game,
            &mut game.get_players_mut().get_mut(0).unwrap(),
            &Card::new(0, Suit::Hearts),
        );

        let player1 = game.get_players().get_next_player_at(0);
        let player2 = game.get_players().get_next_player_at(1);
        assert_eq!(player1.get_hand().get_cards().len(), 0);
        assert_eq!(player2.get_hand().get_cards().len(), 2);
    }
}
