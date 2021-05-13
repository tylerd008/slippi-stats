use peppi::game::Game;
use peppi::game::Frames;
use peppi::frame::Frame2;
use std::fmt;

#[derive(Debug)]
pub struct GameResults{
    results: Vec<GameResult>
}

#[derive(Debug)]
pub struct GameResult{
    player_char: usize,
    opponent_char: usize,
    stage: usize,
    match_result: MatchResult,
}

#[derive(Debug)]
enum MatchResult{
    Victory(MatchEndType),
    Loss(MatchEndType),
    EarlyEnd,
}

#[derive(Debug)]
enum MatchEndType {
    Stocks,
    Timeout
}

#[derive(Debug)]
enum GameParseError{
    CorruptedCharData(usize),
    CorruptedStageData(usize),
    CorruptedPlayerData,
    EmptyCharData,
    IncorrectPlayerCount,
}

impl GameResults{
    fn new() -> Self{
        Self{
            results: Vec::new()
        }
    }
}

impl GameResult{
    fn parse_game(game: Game) -> Result<Self, GameParseError>{
        let player_num =  match get_player_num(&game){
            Some(num) => num,
            None => {
                return Err(GameParseError::CorruptedPlayerData);
            }
        };

        let frames = &game.frames;
		let data = match frames {
			Frames::P2(d) => d,
			_ => {
				return Err(GameParseError::IncorrectPlayerCount);
			}
		};
        let match_result = get_match_result(&data, player_num);

        let player_char = get_char(&game, player_num)?;
        let opponent_char = get_char(&game, 1- player_num)?;
        
        let stage = game.start.stage.0 as usize;

        if stage == 0 || stage == 1 || stage == 21 || stage > 32{
            return Err(GameParseError::CorruptedStageData(stage));
        }

        Ok(
            Self{
                player_char,
                opponent_char,
                stage,
                match_result,
            }
        )
    }
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} vs {} on {}. {}", num_to_char(self.player_char), num_to_char(self.opponent_char), num_to_stage(self.stage), self.match_result)
    }
}

impl fmt::Display for MatchResult{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            MatchResult::Victory(endtype) => write!(f, "Won by {}.", endtype),
            MatchResult::Loss(endtype) => write!(f, "Lost by {}.", endtype),
            MatchResult::EarlyEnd => write!(f, "Match ended early."),
        }
    }
}

impl fmt::Display for MatchEndType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            Stocks => write!(f, "stocks"),
            Timeout => write!(f, "timeout"),
        }
    }
}

fn get_player_num(game: &Game) -> Option<usize>{//rewrite this to return player number, not the player struct
	let players = game.metadata.players.as_ref().unwrap();
	let p2_md = players.get(1).unwrap();
	let p2_np_code = match &p2_md.netplay{
		Some(c) => &c.code,
		None => {
			return None;
		}
	};
	if p2_np_code == "FLOS#497"{
		Some(1)
	} else {
		Some(0)
	}
}

fn get_match_result(data: &Vec<Frame2>, player_num: usize) -> MatchResult{
	let pp_lf = &data[data.len()-1].ports[player_num];//player port last frame

	let op_lf = &data[data.len()-1].ports[1 - player_num];//opponent player last frame

	let p_end_stocks = pp_lf.leader.post.stocks;
	let o_end_stocks = op_lf.leader.post.stocks;

	if p_end_stocks > o_end_stocks{
		MatchResult::Victory(MatchEndType::Stocks)
	} else if p_end_stocks < o_end_stocks {
		MatchResult::Loss(MatchEndType::Stocks)
	} else {
		if pp_lf.leader.post.damage < op_lf.leader.post.damage {
            MatchResult::Victory(MatchEndType::Timeout)
	    } else {
            MatchResult::Loss(MatchEndType::Timeout)
        }
    }
}

fn get_char(game: &Game, player: usize) -> Result<usize, GameParseError>{
    let char_num = match game.start.players.get(player) {
        Some(character) => character,
        None => {
            return Err(GameParseError::EmptyCharData);
        }
    }.character.0 as usize;

    if char_num > 26{
         return Err(GameParseError::CorruptedCharData(char_num));
    }
    Ok(char_num)
}

fn num_to_char(char_num: usize) -> String{
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
        _ => unreachable!()//any other value is errored out before this fn is ever called
    }
}

fn num_to_stage(stage_num: usize) -> String{

    String::from("adfasdf")
}