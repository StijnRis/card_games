use crate::{
    card::Card, game::Game, pile::Pile, player::Player, players::Players, settings::Settings,
};

pub struct ShitHead {
    settings: Settings,
    players: Players<ShitHeadPlayer>,

    stock: Pile,
    played: Pile,
    trash: Pile,
}

impl Game<ShitHeadPlayer> for ShitHead {
    fn get_players(&self) -> &Players<ShitHeadPlayer> {
        &self.players
    }

    fn get_players_mut(&mut self) -> &mut Players<ShitHeadPlayer> {
        &mut self.players
    }

    fn get_settings(&self) -> &Settings {
        &self.settings
    }

    fn start(&mut self) {
        self.stock.add_deck();
        // for i in 0..self.players.iter() {
        //     self.players.get(i).deal();
        // }

        loop {
            let current_player = self.players.get_current_player_mut();

            let mut card: Option<Card> = None;
            while card.is_none() {
                if current_player.get_hand().len() != 0 {
                    card = current_player.get_hand_mut().select_card();
                } else if current_player.get_shown().len() != 0 {
                    card = current_player.get_shown_mut().select_card();
                }
            }

            let difference = 3 - current_player.get_hand().get_cards().len();
            current_player
                .get_hand()
                .add_cards(self.stock.take_top_cards(difference));

            self.players.next_round();
        }
    }
}

pub struct ShitHeadPlayer {
    name: String,

    hand: Pile,
    hidden: Pile,
    shown: Pile,
}

impl ShitHeadPlayer {
    fn get_hand(&self) -> &Pile {
        &self.hand
    }

    fn get_hand_mut(&mut self) -> &mut Pile {
        &mut self.hand
    }

    fn get_hidden_mut(&mut self) -> &mut Pile {
        &mut self.hidden
    }

    fn get_shown_mut(&mut self) -> &mut Pile {
        &mut self.shown
    }

    fn get_hidden(&self) -> &Pile {
        &self.hidden
    }

    fn get_shown(&self) -> &Pile {
        &self.shown
    }

    fn deal(&mut self, deck: &mut Pile, number_of_cards: usize) {
        self.hand.add_cards(deck.take_top_cards(3));
        self.hidden.add_cards(deck.take_top_cards(3));
        self.shown.add_cards(deck.take_top_cards(3));
    }
}

impl Player for ShitHeadPlayer {
    fn get_name(&self) -> &String {
        &self.name
    }
}
