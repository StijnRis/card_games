use std::io::{self};

use crate::{card::Card, pile::Pile};

pub trait Player {
    fn get_name(&self) -> &String;
}

pub struct DefaultPlayer {
    name: String,
}

impl DefaultPlayer {
    pub fn new(name: String) -> DefaultPlayer {
        DefaultPlayer { name: name }
    }
}

impl Player for DefaultPlayer {
    fn get_name(&self) -> &String {
        &self.name
    }
}
