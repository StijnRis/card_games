use crate::{card::Card, mau_mau::MauMau, player::Player, rule::CardPlayedRule};

pub struct Settings {
    card_played_rule: Vec<Box<dyn CardPlayedRule>>,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            card_played_rule: Vec::new(),
        }
    }

    pub fn add_card_played_rule(&mut self, rule: Box<dyn CardPlayedRule>) {
        self.card_played_rule.push(rule);
    }

    pub fn card_played(&self, game: &mut MauMau, player: &mut Player, card: &mut Card) {
        for rule in self.card_played_rule {
            rule.execute(game, player, card);
        }
    }
}
