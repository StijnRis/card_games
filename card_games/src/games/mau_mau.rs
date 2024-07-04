use crate::card::Card;
use crate::game::Game;
use crate::pile::Pile;
use crate::players::Players;
use crate::settings::Settings;
use crate::player::Player;

pub struct MauMau {
    settings: Settings,
    players: Players<MauMauPlayer>,

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

    fn get_stock_mut(&mut self) -> &mut Pile {
        &mut self.stock
    }

    pub fn deal(&mut self, deck: &mut Pile, number_of_cards: usize) {
        for player in self.players.players.iter_mut() {
            player.deal(deck, number_of_cards); 
        }
    }
}

impl Game<MauMauPlayer> for MauMau {
    fn get_players(&self) -> &Players<MauMauPlayer> {
        &self.players
    }

    fn get_players_mut(&mut self) -> &mut Players<MauMauPlayer> {
        &mut self.players
    }

    fn get_settings(&self) -> &Settings {
        &self.settings
    }

    fn start(&mut self) {
        self.stock.add_deck();
        self.deal(&mut self.stock, 5);

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

pub struct MauMauPlayer {
    name: String,
    hand: Pile,
}

impl MauMauPlayer {
    fn new(name: &str) -> MauMauPlayer {
        MauMauPlayer {
            name: name.to_string(),
            hand: Pile::new(),
        }
    }

    fn get_hand(&self) -> &Pile {
        &self.hand
    }

    fn get_hand_mut(&mut self) -> &mut Pile {
        &mut self.hand
    }

    fn deal(&mut self, deck: &mut Pile, number_of_cards: usize) {
        self.hand.add_cards(deck.take_top_cards(3));
    }

    fn select_card(&self) -> Option<Card> {
        self.hand.select_card()
    }
}

impl Player for MauMauPlayer {
    fn get_name(&self) -> &String {
        &self.name
    }
}