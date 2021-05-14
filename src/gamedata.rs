use peppi::game::{Frames, Game, Player, Port};
use peppi::metadata::Player as PlayerMD;
use std::fmt;
#[derive(Debug)]
pub struct GameResults {
    results: Vec<GameResult>,
}

#[derive(Debug)]
pub struct GameResult {
    player_char: usize,
    opponent_char: usize,
    stage: usize,
    match_result: MatchResult,
}

#[derive(Debug)]
enum MatchResult {
    Victory(MatchEndType),
    Loss(MatchEndType),
    EarlyEnd(usize),
    Tie,
}

#[derive(Debug)]
enum MatchEndType {
    Stocks,
    Timeout,
}

#[derive(Debug)]
pub enum GameParseError {
    CorruptedCharData(usize),
    CorruptedStageData(usize),
    CorruptedPlayerData,
    EmptyCharData,
    IncorrectPlayerCount,
    GameDoesNotContainPlayer, //not sure that this should be an error but i'm not sure how else to handle it right now
}

impl GameResults {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn add_game(&mut self, game: GameResult) {
        self.results.push(game);
    }
}

impl GameResult {
    pub fn parse_game(game: Game, np_code: String) -> Result<Self, GameParseError> {
        let player_num = match get_player_num(&game, np_code) {
            Ok(Some(num)) => num,
            Ok(None) => {
                return Err(GameParseError::CorruptedPlayerData);
            }
            Err(e) => {
                return Err(e);
            }
        };
        let match_result = match get_match_result(&game, player_num) {
            Ok(game_res) => game_res,
            Err(e) => {
                return Err(e);
            }
        };

        let player_char = get_char(&game, player_num)?;
        let opponent_char = get_char(&game, 1 - player_num)?;

        let stage = game.start.stage.0 as usize;

        if stage == 0 || stage == 1 || stage == 21 || stage > 32 {
            return Err(GameParseError::CorruptedStageData(stage));
        }

        Ok(Self {
            player_char,
            opponent_char,
            stage,
            match_result,
        })
    }
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} vs {} on {}. {}",
            num_to_char(self.player_char),
            num_to_char(self.opponent_char),
            num_to_stage(self.stage),
            self.match_result
        )
    }
}

impl fmt::Display for MatchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MatchResult::Victory(endtype) => write!(f, "Won by {}.", endtype),
            MatchResult::Loss(endtype) => write!(f, "Lost by {}.", endtype),
            MatchResult::EarlyEnd(player_num) => {
                write!(f, "Match ended early by player {}.", player_num)
            }
            MatchResult::Tie => write!(f, "Ended in a tie."),
        }
    }
}

impl fmt::Display for MatchEndType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MatchEndType::Stocks => write!(f, "stocks"),
            MatchEndType::Timeout => write!(f, "timeout"),
        }
    }
}

fn get_player_num(game: &Game, np_code: String) -> Result<Option<usize>, GameParseError> {
    let players = game.metadata.players.as_ref().unwrap();
    let p1_np_code = get_np_code(&players, 0)?;
    let p2_np_code = get_np_code(&players, 1)?;
    if p1_np_code == &np_code {
        Ok(Some(0))
    } else if p2_np_code == &np_code {
        Ok(Some(1))
    } else {
        Err(GameParseError::GameDoesNotContainPlayer)
    }
}

fn get_match_result(game: &Game, player_num: usize) -> Result<MatchResult, GameParseError> {
    let frames = &game.frames;
    let data = match frames {
        Frames::P2(d) => d,
        _ => {
            return Err(GameParseError::IncorrectPlayerCount);
        }
    };
    let pp_lf = &data[data.len() - 1].ports[player_num]; //player port last frame

    let op_lf = &data[data.len() - 1].ports[1 - player_num]; //opponent player last frame

    let p_end_stocks = pp_lf.leader.post.stocks;
    let o_end_stocks = op_lf.leader.post.stocks;

    let ev20 = game.end.v2_0.as_ref().unwrap();

    if ev20.lras_initiator != None {
        let port = ev20.lras_initiator.unwrap();
        match port {
            Port::P1 => {
                return Ok(MatchResult::EarlyEnd(1));
            }
            Port::P2 => {
                return Ok(MatchResult::EarlyEnd(2));
            }
            Port::P3 => {
                return Ok(MatchResult::EarlyEnd(3));
            }
            Port::P4 => {
                return Ok(MatchResult::EarlyEnd(4));
            }
        }
    }

    if p_end_stocks > o_end_stocks {
        Ok(MatchResult::Victory(MatchEndType::Stocks))
    } else if p_end_stocks < o_end_stocks {
        Ok(MatchResult::Loss(MatchEndType::Stocks))
    } else {
        if pp_lf.leader.post.damage < op_lf.leader.post.damage {
            Ok(MatchResult::Victory(MatchEndType::Timeout))
        } else if pp_lf.leader.post.damage > op_lf.leader.post.damage {
            Ok(MatchResult::Loss(MatchEndType::Timeout))
        } else {
            Ok(MatchResult::Tie)
        }
    }
}

fn get_char(game: &Game, player: usize) -> Result<usize, GameParseError> {
    let char_num = match game.start.players.get(player) {
        Some(character) => character,
        None => {
            return Err(GameParseError::EmptyCharData);
        }
    }
    .character
    .0 as usize;

    if char_num > 26 {
        return Err(GameParseError::CorruptedCharData(char_num));
    }
    Ok(char_num)
}

fn get_np_code(players: &Vec<PlayerMD>, p_number: usize) -> Result<&str, GameParseError> {
    let p_md = players.get(p_number).unwrap();
    match &p_md.netplay {
        Some(c) => Ok(&c.code),
        None => Err(GameParseError::CorruptedPlayerData),
    }
}

fn num_to_char(char_num: usize) -> String {
    match char_num {
        0 => String::from("Captain Falcon"),
        1 => String::from("Donkey Kong"),
        2 => String::from("Fox"),
        3 => String::from("Mr. Game and Watch"),
        4 => String::from("Kirby"),
        5 => String::from("Bowser"),
        6 => String::from("Link"),
        7 => String::from("Luigi"),
        8 => String::from("Mario"),
        9 => String::from("Marth"),
        10 => String::from("Mewtwo"),
        11 => String::from("Ness"),
        12 => String::from("Peach"),
        13 => String::from("Pikachu"),
        14 => String::from("Ice Climbers"),
        15 => String::from("Jigglypuff"),
        16 => String::from("Samus"),
        17 => String::from("Yoshi"),
        18 => String::from("Zelda"),
        19 => String::from("Sheik"),
        20 => String::from("Falco"),
        21 => String::from("Young Link"),
        22 => String::from("Dr. Mario"),
        23 => String::from("Roy"),
        24 => String::from("Pichu"),
        25 => String::from("Ganondorf"),
        _ => unreachable!(), //any other value is errored out before this fn is ever called
    }
}

fn num_to_stage(stage_num: usize) -> String {
    match stage_num {
        2 => String::from("Fountain of Dreams"),
        3 => String::from("Pokémon Stadium"),
        4 => String::from("Princess Peach's Castle"),
        5 => String::from("Kongo Jungle"),
        6 => String::from("Brinstar"),
        7 => String::from("Corneria"),
        8 => String::from("Yoshi's Story"),
        9 => String::from("Onett"),
        10 => String::from("Mute City"),
        11 => String::from("Raianbow Cruise"),
        12 => String::from("Jungle Japes"),
        13 => String::from("Great Bay"),
        14 => String::from("Hyrule Temple"),
        15 => String::from("Brinstar Depths"),
        16 => String::from("Yoshi's Island"),
        17 => String::from("Green Greens"),
        18 => String::from("Fourside"),
        19 => String::from("Mushroom Kingdom I"),
        20 => String::from("Mushroom Kingdom II"),
        22 => String::from("Venom"),
        23 => String::from("Poké Floats"),
        24 => String::from("Big Blue"),
        25 => String::from("Icicle Mountain"),
        26 => String::from("Icetop"), //?
        27 => String::from("Flat Zone"),
        28 => String::from("Dream Land N64"),
        29 => String::from("Yoshi's Island N64"),
        30 => String::from("Kongo Jungle N64"),
        31 => String::from("Battlefield"),
        32 => String::from("Final Destination"),
        _ => unreachable!(), //any other value is errored out before this fn is ever called
    }
}
