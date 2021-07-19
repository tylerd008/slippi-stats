use crate::gamedata::GameData;
use crate::parsable_enum::GameDataCondition;
use std::fmt::Display;

pub enum Player {
    Player,
}

impl GameDataCondition for Player {
    fn game_data_condition(&self, _game: &GameData) -> bool {
        true
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Overall")
    }
}
