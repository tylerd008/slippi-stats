use crate::parsable_enum;
use crate::parsable_enum::UnnamedTrait;
use crate::playerdata::GameData;

parsable_enum! {
    pub enum Stage {
        "Fountain of Dreams"; "fountain", "fod", => FountainOfDreams = 2,
        "Pokémon Stadium"; "pokemon stadium", "pokemon", "stadium", =>PokemonStadium = 3,
        "Yoshi's Story"; "yoshi's", "ys", => YoshisStory = 8,
        "Dream Land N64"; "dream land", "dreamland", "dl", => DreamLandN64 = 28,
        "Battlefield"; "bf", => Battlefield = 31,
        "Final Destination"; "fd", => FinalDestination = 32,
        "Princess Peach's Castle"; "peach's castle", "ppc", => PrincessPeachsCastle = 4,
        "Kongo Jungle"; "kj", => KongoJungle = 5,
        "Brinstar"; => Brinstar = 6,
        "Corneria"; => Corneria = 7,
        "Onett"; => Onett = 9,
        "Mute City"; "mc", => MuteCity = 10,
        "Rainbow Cruise"; "rc", => RainbowCruise = 11,
        "Jungle Japes"; "jj", => JungleJapes = 12,
        "Great Bay"; "gb", => GreatBay = 13,
        "Hyrule Temple"; "temple", "ht", => HyruleTemple = 14,
        "Brinstar Depths"; "bd", => BrinstarDepths = 15,
        "Yoshi's Island"; "yi", => YoshisIsland = 16,
        "Green Greens"; "gg", => GreenGreens = 17,
        "Fourside"; => Fourside = 18,
        "Mushroom Kingdom I"; "mushroom kingdom 1", "mk1", => MushroomKingdomI = 19,
        "Mushroom Kingdom II"; "mushroom kingdom 2", "mk2", => MushroomKingdomII = 20,
        "Venom"; => Venom = 22,
        "Poké Floats"; "poke floats", "pf", => PokeFloats = 23,
        "Big Blue"; "bb", => BigBlue = 24,
        "Icicle Mountain"; "im", => IcicleMountain = 25,
        "Flat Zone"; "fz", => FlatZone = 27,
        "Yoshi's Island N64"; "yoshi's island 64", "yi64", => YoshisIslandN64 = 29,
        "Kongo Jungle N64"; "kongo jungle 64", "kj64", => KongoJungleN64 = 30,
    }
}

impl UnnamedTrait for Stage {
    fn condition(&self, game: &GameData) -> bool {
        game.stage == *self as usize
    }
}
