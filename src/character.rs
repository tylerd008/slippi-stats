use crate::gamedata::GameData;
use crate::parsable_enum;
use crate::parsable_enum::Numbered;
use crate::parsable_enum::UnnamedTrait;
parsable_enum! {
    pub enum Character {
        "Captain Falcon"; "falcon", => CaptainFalcon = 0,
        "Donkey Kong"; "dk", => DonkeyKong = 1,
        "Fox"; => Fox = 2,
        "Mr. Game and Watch"; "mr game and watch", "game and watch", "gnw", => MrGameAndWatch = 3,
        "Kirby"; => Kirby = 4 ,
        "Bowser"; => Bowser = 5,
        "Link"; => Link = 6,
        "Luigi"; => Luigi = 7,
        "Mario"; => Mario = 8,
        "Marth"; => Marth = 9,
        "Mewtwo"; => Mewtwo = 10,
        "Ness"; => Ness = 11,
        "Peach"; => Peach = 12 ,
        "Pikachu"; => Pikachu = 13,
        "Ice Climbers"; "ics", "ic", => IceClimbers = 14,
        "Jigglypuff"; "puff", => Jigglypuff = 15,
        "Samus"; => Samus = 16,
        "Yoshi"; => Yoshi = 17,
        "Zelda"; => Zelda = 18,
        "Sheik"; => Sheik = 19,
        "Falco"; => Falco = 20,
        "Young Link"; "yl", => YoungLink = 21,
        "Dr. Mario"; "dr mario" , "doc", => DrMario = 22,
        "Roy"; => Roy = 23,
        "Pichu"; => Pichu = 24,
        "Ganondorf"; "ganon", => Ganondorf = 25,
    }
}

impl Numbered for Character {
    const NUM_VALUES: usize = 26;
}

impl UnnamedTrait for Character {
    fn condition(&self, game: &GameData) -> bool {
        game.player_char == *self as usize
    }
}
