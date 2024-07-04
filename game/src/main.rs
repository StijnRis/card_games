
pub fn main() {
    let mut game = MauMau::new();
    game.add_player(Player::new("Alice"));
    game.add_player(Player::new("Bob"));
    game.start();
}
