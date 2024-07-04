use crate::pile::Pile;
use crate::player::Player;
use crate::players::Players;
use crate::settings::Settings;

pub trait Game<T> where T: Player{
    fn start(&mut self);

    fn get_players(&self) -> &Players<T>;
    fn get_players_mut(&mut self) -> &mut Players<T>;
    fn get_settings(&self) -> &Settings;

    fn add_player(&mut self, player: T) {
        self.get_players_mut().add_player(player);
    }
}
