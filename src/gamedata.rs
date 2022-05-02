use peppi::game::{Frames, Game, Port};
use std::cmp::Ordering;
use std::fmt;
use std::fs::File;

use chrono::{DateTime, Utc};
use peppi::metadata::Player as PlayerMD;
use peppi::parse;
use peppi::ParseError;
use std::path::Path;

use crate::character::Character;
use crate::stage::Stage;

use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    pub player_char: Character,
    pub opponent_char: Character,
    pub stage: Stage,
    pub match_result: MatchResult,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MatchResult {
    Victory(MatchEndType),
    Loss(MatchEndType),
    EarlyEnd(usize),
    Tie,
}

#[derive(Debug)]
pub enum GameParseError {
    CorruptedCharData(usize),
    CorruptedStageData(usize),
    CorruptedPlayerData,
    EmptyCharData,
    IncorrectPlayerCount,
    GameDoesNotContainPlayer,
    PeppiError(ParseError),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MatchEndType {
    Stocks,
    Timeout,
}

impl GameData {
    pub fn parse_game(game: Game, np_code: String) -> Result<Self, GameParseError> {
        if game.metadata.players.as_ref().unwrap().len() > 2 {
            return Err(GameParseError::IncorrectPlayerCount);
        }
        if !has_player(&game, &np_code)? {
            return Err(GameParseError::GameDoesNotContainPlayer);
        }
        let player_num = match get_player_num(&game, np_code) {
            Some(num) => num,
            None => {
                return Err(GameParseError::CorruptedPlayerData);
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

        let timestamp = game.metadata.date.unwrap();

        let stage_num = game.start.stage.0 as usize;

        if stage_num == 0 || stage_num == 1 || stage_num == 21 || stage_num > 32 {
            return Err(GameParseError::CorruptedStageData(stage_num));
        }

        let stage = Stage::try_from(stage_num).unwrap();

        Ok(Self {
            player_char,
            opponent_char,
            stage,
            match_result,
            timestamp,
        })
    }

    pub fn get_game_data(path: &Path, skip_frames: bool) -> Result<Game, GameParseError> {
        match peppi::game(
            &mut File::open(&path).unwrap(),
            Some(parse::Opts { skip_frames }),
        ) {
            Ok(val) => Ok(val),
            Err(e) => Err(GameParseError::PeppiError(e)),
        }
    }

    pub fn is_victory(&self) -> bool {
        matches!(self.match_result, MatchResult::Victory(_))
    }
}

fn has_player(game: &Game, np_code: &str) -> Result<bool, GameParseError> {
    let players = game.metadata.players.as_ref().unwrap();
    let p1_np_code = get_np_code(&players, 0)?;
    let p2_np_code = get_np_code(&players, 1)?;

    Ok(p1_np_code == np_code || p2_np_code == np_code)
}

fn get_player_num(game: &Game, np_code: String) -> Option<usize> {
    let players = game.metadata.players.as_ref().unwrap();
    let p2_md = players.get(1).unwrap();
    let p2_np_code = match &p2_md.netplay {
        Some(c) => &c.code,
        None => {
            return None;
        }
    };
    if p2_np_code == &np_code {
        Some(1)
    } else {
        Some(0)
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

    let op_lf = &data[data.len() - 1].ports[1 - player_num]; //opponent port last frame

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

    let result = match p_end_stocks.cmp(&o_end_stocks) {
        Ordering::Greater => MatchResult::Victory(MatchEndType::Stocks),
        Ordering::Less => MatchResult::Loss(MatchEndType::Stocks),
        Ordering::Equal => {
            if pp_lf.leader.post.damage < op_lf.leader.post.damage {
                MatchResult::Victory(MatchEndType::Timeout)
            } else if pp_lf.leader.post.damage > op_lf.leader.post.damage {
                MatchResult::Loss(MatchEndType::Timeout)
            } else {
                MatchResult::Tie
            }
        }
    };
    Ok(result)
}

fn get_char(game: &Game, player: usize) -> Result<Character, GameParseError> {
    let char_num = match game.start.players.get(player) {
        Some(character) => character,
        None => {
            return Err(GameParseError::EmptyCharData);
        }
    }
    .character
    .0 as usize;

    if char_num >= 26 {
        return Err(GameParseError::CorruptedCharData(char_num));
    }
    Ok(Character::try_from(char_num).unwrap())
}

fn get_np_code(players: &[PlayerMD], p_number: usize) -> Result<&str, GameParseError> {
    let p_md = players.get(p_number).unwrap();
    match &p_md.netplay {
        Some(c) => Ok(&c.code),
        None => Err(GameParseError::CorruptedPlayerData),
    }
}

impl fmt::Display for GameData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} vs {} on {}. {}",
            self.player_char, self.opponent_char, self.stage, self.match_result
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
