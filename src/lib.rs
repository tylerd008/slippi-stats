mod character;
mod gamedata;
pub mod input;
mod parsable_enum;
mod player;
mod playerdata;
mod stage;
mod text;

#[cfg(test)]
mod tests {
    use crate::character::Character;
    use crate::stage::Stage;
    use std::convert::TryFrom;
    use std::str::FromStr;
    #[test]
    fn char_parse_from_usize() {
        let char_from_usize = Character::try_from(0).unwrap();
        assert_eq!(Character::CaptainFalcon, char_from_usize);
    }
    #[test]
    fn char_parse_from_str() {
        let char_from_str = Character::from_str("captain falcon").unwrap();
        assert_eq!(Character::CaptainFalcon, char_from_str);
    }
    #[test]
    fn stage_parse_from_usize() {
        let stage_from_usize = Stage::try_from(2).unwrap();
        assert_eq!(Stage::FountainOfDreams, stage_from_usize);
    }
    #[test]
    fn stage_parse_from_usize_nonexistent_stage() {
        let stage_from_usize = Stage::try_from(0);
        assert!(stage_from_usize.is_err());
    }
}
