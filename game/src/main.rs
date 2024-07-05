use card_games::mau_mau::MauMau;

pub fn main() {
    let mut game = MauMau::new();
    game.add_player("Alice".to_owned());
    game.add_player("Bob".to_owned());

    game.setup();
    game.start();
}
