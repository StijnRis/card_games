use crate::player::Player;
use crate::pile::Pile;

pub struct Players<T> where T: Player {
    current_player_index: usize,
    pub players: Vec<T>,
}

impl<T> Players<T> where T: Player {
    pub fn new() -> Players<T> {
        Players {
            current_player_index: 0,
            players: vec![],
        }
    }

    pub fn get(index: usize) -> &Player<T> {
        
    }

    pub fn add_player(&mut self, player: T) {
        self.players.push(player);
    }

    pub fn get_current_player(&self) -> &T {
        self.players.get(self.current_player_index).unwrap()
    }

    pub fn get_current_player_mut(&mut self) ->  &mut T {
        self.players.get_mut(self.current_player_index).unwrap()
    }

    pub fn get_next_player_at_mut(&mut self, position: usize) -> &mut T {
        let index = (self.current_player_index + position) % self.players.len();
        self.players.get_mut(index).unwrap()
    }

    pub fn get_next_player_at(&self, position: usize) -> &T {
        let index = (self.current_player_index + position) % self.players.len();
        self.players.get(index).unwrap()
    }

    pub fn next_round(&mut self) {
        self.current_player_index = (self.current_player_index + 1) % self.players.len();
    }
}



#[cfg(test)]
mod tests {
    use crate::player::{DefaultPlayer, Player};
    use crate::players::Players;

    #[test]
    fn add_players() {
        let mut players = Players::new();
        players.add_player(DefaultPlayer::new("Piet".to_owned()));
        players.add_player(DefaultPlayer::new("Jan".to_owned()));
        players.add_player(DefaultPlayer::new("Klaas".to_owned()));

        assert_eq!(players.players.len(), 3);
    }

    #[test]
    fn get_current_player() {
        let mut players = Players::new();
        players.add_player(DefaultPlayer::new("Piet".to_owned()));
        players.add_player(DefaultPlayer::new("Jan".to_owned()));
        players.add_player(DefaultPlayer::new("Klaas".to_owned()));

        assert_eq!(players.get_current_player().get_name(), "Piet");
        players.next_round();
        assert_eq!(players.get_current_player().get_name(), "Jan");
        players.next_round();
        assert_eq!(players.get_current_player().get_name(), "Klaas");
    }

    #[test]
    fn go_round() {
        let mut players = Players::new();
        players.add_player(DefaultPlayer::new("Piet".to_owned()));
        players.add_player(DefaultPlayer::new("Jan".to_owned()));

        assert_eq!(players.get_current_player().get_name(), "Piet");
        players.next_round();
        assert_eq!(players.get_current_player().get_name(), "Jan");
        players.next_round();
        assert_eq!(players.get_current_player().get_name(), "Piet");
    }

    #[test]
    fn get_next_player() {
        let mut players = Players::new();
        players.add_player(DefaultPlayer::new("Piet".to_owned()));
        players.add_player(DefaultPlayer::new("Jan".to_owned()));
        players.add_player(DefaultPlayer::new("Klaas".to_owned()));

        assert_eq!(players.get_next_player_at(0).get_name(), "Piet");
        assert_eq!(players.get_next_player_at(1).get_name(), "Jan");
        assert_eq!(players.get_next_player_at(2).get_name(), "Klaas");
        assert_eq!(players.get_next_player_at(3).get_name(), "Piet");
    }
}