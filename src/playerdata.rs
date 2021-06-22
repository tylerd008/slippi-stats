use peppi::game::{Frames, Game, Port};
use std::fmt;
use std::fs;
use std::fs::File;
use std::str::FromStr;
use std::time::Instant;

use chrono::{DateTime, Utc};
use peppi::metadata::Player as PlayerMD;
use peppi::parse;
use peppi::ParseError;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use indicatif::{HumanDuration, ProgressBar, ProgressStyle};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerData {
    cache_ver: usize,
    results: Vec<GameData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    player_char: usize,
    opponent_char: usize,
    stage: usize,
    match_result: MatchResult,
    timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
enum MatchResult {
    Victory(MatchEndType),
    Loss(MatchEndType),
    EarlyEnd(usize),
    Tie,
}

#[derive(Debug, Serialize, Deserialize)]
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
    PeppiError(ParseError),
}

#[derive(Debug)]
pub enum ArgType {
    Stage(usize),
    Character(usize),
    Player,
}

pub enum ArgTypeParseError {
    UnrecognizedInput,
}

enum DataType {
    Stages,
    Characters,
    Opponents,
}

#[derive(Debug)]
struct FavBestData {
    favorite: usize, //(wins, total games)
    best: usize,
}

impl PlayerData {
    const CACHE_VER: usize = 7;
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            cache_ver: PlayerData::CACHE_VER,
        }
    }

    pub fn parse_dir(p: PathBuf, np_code: String) -> Result<Self, GameParseError> {
        let mut cache_path = String::from(p.as_path().to_str().unwrap());
        cache_path.push_str(&format!("/{}.cache", np_code));
        let mut cache = match fs::read_to_string(&cache_path) {
            Ok(c) => c,
            Err(_) => "".to_string(),
        };
        let mut results: PlayerData = match serde_json::from_str(&cache) {
            Ok(gr) => gr,
            Err(_) => PlayerData::new(),
        };

        if results.cache_ver != PlayerData::CACHE_VER {
            println!("Cache detected but out of date. Rebuilding.");
            results = PlayerData::new();
            cache = "".to_string();
        }

        let total = count_replays(&p);
        let pb = ProgressBar::new(total);
        let start = Instant::now();
        pb.set_style(
            ProgressStyle::default_bar().template(
                "[{elapsed_precise}] [{wide_bar:.green/white}] {pos}/{len} ({eta_precise})",
            ),
        );
        for entry in fs::read_dir(p).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().unwrap() != "slp" {
                continue;
            }
            let game_data = match GameData::get_game_data(&path, true) {
                Ok(gd) => gd,
                Err(e) => {
                    pb.println(format!("error {:?} when parsing game {:?}", e, path));
                    pb.inc(1);
                    continue;
                }
            };
            let dt = serde_json::to_string(&game_data.metadata.date.unwrap()).unwrap();
            if cache.contains(&dt) {
                //not sure what's better: this, or loading the deserialized data and then iterating through it and checking each gamedata
                pb.inc(1);
                continue;
            }

            match GameData::has_player(&game_data, np_code.to_string()) {
                Ok(has_player) => {
                    if !has_player {
                        pb.println(format!("Game does not contain player. Skipping."));
                        pb.inc(1);
                        continue;
                    }
                }
                Err(e) => {
                    pb.println(format!("Error {:?}, when parsing game: {:?}", e, path));
                    pb.inc(1);
                    continue;
                }
            }
            let gamedata_with_frames = match GameData::get_game_data(&path, false) {
                Ok(gd) => gd,
                Err(e) => {
                    pb.println(format!("error {:?} when parsing game {:?}", e, path));
                    pb.inc(1);
                    continue;
                }
            };
            let result = match GameData::parse_game(gamedata_with_frames, np_code.to_string()) {
                Ok(g) => g,
                Err(e) => {
                    pb.println(format!("Error when parsing game result: {:?}", e));
                    pb.inc(1);
                    continue;
                }
            };
            results.add_game(result);
            pb.inc(1);
        }
        let serial = serde_json::to_string(&results).unwrap();
        fs::write(cache_path, serial).unwrap();
        pb.finish_and_clear();
        let end = start.elapsed();
        println!("{} replays scanned in {}", total, HumanDuration(end));
        Ok(results)
    }

    pub fn add_game(&mut self, game: GameData) {
        self.results.push(game);
    }

    pub fn winrate(&self, arg: &ArgType) {
        let mut games = 0;
        let mut wins = 0;

        for game in &self.results {
            match arg {
                ArgType::Character(num) => {
                    if &game.player_char == num {
                        games += 1;
                        if game.is_victory() {
                            wins += 1;
                        }
                    }
                }
                ArgType::Stage(num) => {
                    if &game.stage == num {
                        games += 1;
                        if game.is_victory() {
                            wins += 1;
                        }
                    }
                }
                ArgType::Player => {
                    games += 1;
                    if game.is_victory() {
                        wins += 1;
                    }
                }
            }
        }
        let ws = match winrate_string(wins, games, true) {
            Some(s) => s,
            None => {
                println!("No data for given input");
                return;
            }
        };
        match arg {
            ArgType::Character(num) => {
                println!("As {}:", num_to_char(*num));
            }
            ArgType::Stage(num) => {
                println!("On {}:", num_to_stage(*num));
            }
            ArgType::Player => {
                println!("Overall:");
            }
        }
        println!("{}", ws);
    }

    pub fn matchups(&self, arg: &ArgType) {
        let mut matchup_data: Vec<(usize, usize)> = vec![(0, 0); 26];

        for game in &self.results {
            match arg {
                ArgType::Character(num) => {
                    if &game.player_char == num {
                        matchup_data[game.opponent_char].1 += 1;
                        if game.is_victory() {
                            matchup_data[game.opponent_char].0 += 1;
                        }
                    }
                }
                ArgType::Stage(num) => {
                    if &game.stage == num {
                        matchup_data[game.opponent_char].1 += 1;
                        if game.is_victory() {
                            matchup_data[game.opponent_char].0 += 1;
                        }
                    }
                }
                ArgType::Player => {
                    matchup_data[game.opponent_char].1 += 1;
                    if game.is_victory() {
                        matchup_data[game.opponent_char].0 += 1;
                    }
                }
            }
        }
        match arg {
            ArgType::Character(num) => {
                println!("As {}:", num_to_char(*num));
            }
            ArgType::Stage(num) => {
                println!("On {}:", num_to_stage(*num));
            }
            ArgType::Player => {
                println!("Overall:");
            }
        }
        print_data(DataType::Opponents, &matchup_data);
    }

    pub fn stages(&self, arg: &ArgType) {
        let char_num = match arg {
            ArgType::Character(num) => num,
            _ => {
                println!("This function only accepts character input.");
                return;
            }
        };
        let mut stage_data: Vec<(usize, usize)> = vec![(0, 0); 33];

        for game in &self.results {
            if &game.player_char == char_num {
                stage_data[game.stage].1 += 1;
                if game.is_victory() {
                    stage_data[game.stage].0 += 1;
                }
            }
        }
        println!("As {}", num_to_char(*char_num));
        print_data(DataType::Stages, &stage_data);
    }

    pub fn characters(&self, arg: &ArgType) {
        let stage_num = match arg {
            ArgType::Stage(num) => num,
            _ => {
                println!("This function only accepts stage input.");
                return;
            }
        };
        let mut char_data: Vec<(usize, usize)> = vec![(0, 0); 36];

        for game in &self.results {
            if &game.stage == stage_num {
                char_data[game.player_char].1 += 1;
                if game.is_victory() {
                    char_data[game.player_char].0 += 1;
                }
            }
        }
        println!("On {}", num_to_stage(*stage_num));
        print_data(DataType::Characters, &char_data);
    }

    pub fn matchup(&self, player: ArgType, opponent: ArgType) {
        let player = match player {
            ArgType::Character(num) => num,
            _ => unreachable!(),
        };

        let opponent = match opponent {
            ArgType::Character(num) => num,
            _ => unreachable!(),
        };
        let mut games = 0;
        let mut wins = 0;
        let mut stage_data = vec![(0, 0); 33];

        for game in &self.results {
            if game.player_char == player && game.opponent_char == opponent {
                games += 1;
                stage_data[game.stage].1 += 1;
                if game.is_victory() {
                    wins += 1;
                    stage_data[game.stage].0 += 1;
                }
            }
        }
        let ws = match winrate_string(wins, games, true) {
            Some(s) => s,
            None => {
                println!("No data for given input");
                return;
            }
        };

        println!("{} vs. {}:", num_to_char(player), num_to_char(opponent));
        println!("{}", ws);
        print_data(DataType::Stages, &stage_data);
    }

    pub fn last(&self, num_games: usize) {
        let mut i = self.results.len() - num_games;
        let end = self.results.len();
        while i < end {
            println!("{}", self.results[i]);
            i += 1;
        }
    }

    pub fn overview(&self) {
        let mut char_data: Vec<(usize, usize)> = vec![(0, 0); 26];
        let mut opponent_data: Vec<(usize, usize)> = vec![(0, 0); 26];
        let mut stage_data: Vec<(usize, usize)> = vec![(0, 0); 33];
        for game in &self.results {
            char_data[game.player_char].1 += 1;
            opponent_data[game.opponent_char].1 += 1;
            stage_data[game.stage].1 += 1;
            if game.is_victory() {
                char_data[game.player_char].0 += 1;
                opponent_data[game.opponent_char].0 += 1;
                stage_data[game.stage].0 += 1;
            }
        }
        let char_fb = PlayerData::fav_best(&char_data);
        let opponent_fb = PlayerData::fav_best(&opponent_data);
        let stage_fb = PlayerData::fav_best(&stage_data);

        print_fb(DataType::Characters, char_fb, char_data);
        print_fb(DataType::Opponents, opponent_fb, opponent_data);
        print_fb(DataType::Stages, stage_fb, stage_data);
    }

    fn fav_best(data: &Vec<(usize, usize)>) -> FavBestData {
        let mut favorite = 0;
        let mut best = 0;
        let mut best_winrate = 0.0;
        for i in 0..data.len() {
            if data[i].1 > data[favorite].1 {
                favorite = i;
            }
            let current_winrate = data[i].0 as f64 / data[i].1 as f64;
            if (data[i].1 > 20) && (current_winrate > best_winrate) {
                //min 20 games so things with 1 game and 1 win don't end up taking the spot
                best = i;
                best_winrate = current_winrate;
            }
        }
        FavBestData { favorite, best }
    }
}

impl GameData {
    pub fn parse_game(game: Game, np_code: String) -> Result<Self, GameParseError> {
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

        let stage = game.start.stage.0 as usize;

        if stage == 0 || stage == 1 || stage == 21 || stage > 32 {
            return Err(GameParseError::CorruptedStageData(stage));
        }

        Ok(Self {
            player_char,
            opponent_char,
            stage,
            match_result,
            timestamp,
        })
    }

    pub fn get_game_data(path: &PathBuf, skip_frames: bool) -> Result<Game, GameParseError> {
        match peppi::game(
            &mut File::open(&path).unwrap(),
            Some(parse::Opts { skip_frames }),
        ) {
            Ok(val) => Ok(val),
            Err(e) => {
                return Err(GameParseError::PeppiError(e));
            }
        }
    }

    pub fn has_player(game: &Game, np_code: String) -> Result<bool, GameParseError> {
        let players = game.metadata.players.as_ref().unwrap();
        let p1_np_code = get_np_code(&players, 0)?;
        let p2_np_code = get_np_code(&players, 1)?;

        Ok(p1_np_code == np_code || p2_np_code == np_code)
    }

    pub fn is_victory(&self) -> bool {
        match self.match_result {
            MatchResult::Victory(_) => true,
            _ => false,
        }
    }
}

impl FromStr for ArgType {
    type Err = ArgTypeParseError;
    fn from_str(arg: &str) -> Result<Self, Self::Err> {
        let output = match &(arg.to_string())[..] {
            "captain falcon" | "falcon" => ArgType::Character(0),
            "donkey kong" | "dk" => ArgType::Character(1),
            "fox" => ArgType::Character(2),
            "mr. game and watch" | "mr game and watch" | "game and watch" | "gnw" => {
                ArgType::Character(3)
            }
            "kirby" => ArgType::Character(4),
            "bowser" => ArgType::Character(5),
            "link" => ArgType::Character(6),
            "luigi" => ArgType::Character(7),
            "mario" => ArgType::Character(8),
            "marth" => ArgType::Character(9),
            "mewtwo" => ArgType::Character(10),
            "ness" => ArgType::Character(11),
            "peach" => ArgType::Character(12),
            "pikachu" => ArgType::Character(13),
            "ice climbers" | "ics" | "ic" => ArgType::Character(14),
            "jigglypuff" | "puff" => ArgType::Character(15),
            "samus" => ArgType::Character(16),
            "yoshi" => ArgType::Character(17),
            "zelda" => ArgType::Character(18),
            "sheik" => ArgType::Character(19),
            "falco" => ArgType::Character(20),
            "young link" | "yl" => ArgType::Character(21),
            "dr. mario" | "dr mario" | "doc" => ArgType::Character(22),
            "roy" => ArgType::Character(23),
            "pichu" => ArgType::Character(24),
            "ganondorf" | "ganon" => ArgType::Character(25),
            "fountain of dreams" | "fountain" | "fod" => ArgType::Stage(2),
            "pokémon stadium" | "pokemon stadium" | "pokemon" | "stadium" => ArgType::Stage(3),
            "yoshi's story" | "yoshi's" | "ys" => ArgType::Stage(8),
            "dream land n64" | "dream land" | "dreamland" | "dl" => ArgType::Stage(29),
            "battlefield" | "bf" => ArgType::Stage(31),
            "final destination" | "fd" => ArgType::Stage(32),
            //rest of the stages included just for completeness' sake
            "princess peach's castle" | "peach's castle" | "ppc" => ArgType::Stage(4),
            "kongo jungle" | "kj" => ArgType::Stage(5),
            "brinstar" => ArgType::Stage(6),
            "corneria" => ArgType::Stage(7),
            "onett" => ArgType::Stage(9),
            "mute city" | "mc" => ArgType::Stage(10),
            "rainbow cruise" | "rc" => ArgType::Stage(11),
            "jungle japes" | "jj" => ArgType::Stage(12),
            "great bay" | "gb" => ArgType::Stage(13),
            "hyrule temple" | "temple" | "ht" => ArgType::Stage(14),
            "brinstar depths" | "bd" => ArgType::Stage(15),
            "yoshi's island" | "yi" => ArgType::Stage(16),
            "green greens" | "gg" => ArgType::Stage(17),
            "fourside" => ArgType::Stage(18),
            "mushroom kingdom i" | "mushroom kingdom 1" | "mk1" => ArgType::Stage(19),
            "mushroom kingdom ii" | "mushroom kingdom 2" | "mk2" => ArgType::Stage(20),
            "venom" => ArgType::Stage(22),
            "poké floats" | "poke floats" | "pf" => ArgType::Stage(23),
            "big blue" | "bb" => ArgType::Stage(24),
            "icicle mountain" | "im" => ArgType::Stage(25),
            "flat zone" | "fz" => ArgType::Stage(27),
            "yoshi's island n64" | "yoshi's island 64" | "yi64" => ArgType::Stage(29),
            "kongo jungle n64" | "kongo jungle 64" | "kj64" => ArgType::Stage(30),
            _ => {
                return Err(ArgTypeParseError::UnrecognizedInput);
            }
        };
        Ok(output)
    }
}

impl fmt::Display for GameData {
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

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::Stages => write!(f, "On"),
            DataType::Characters => write!(f, "As"),
            DataType::Opponents => write!(f, "Vs."),
        }
    }
}

impl DataType {
    fn parse(&self, data_label: usize) -> String {
        match self {
            DataType::Stages => num_to_stage(data_label),
            DataType::Characters => num_to_char(data_label),
            DataType::Opponents => num_to_char(data_label),
        }
    }
}

fn print_data(data_type: DataType, data: &Vec<(usize, usize)>) {
    let mut is_data = false;
    for i in 0..data.len() {
        let ws = match winrate_string(data[i].0, data[i].1, false) {
            Some(s) => {
                is_data = true;
                s
            }
            None => {
                continue;
            }
        };

        println!("{} {} {}", data_type, data_type.parse(i), ws);
    }
    if !is_data {
        println!("No data for given input.");
    }
}

fn print_fb(d_type: DataType, fb: FavBestData, data: Vec<(usize, usize)>) {
    let data_labels = match d_type {
        DataType::Characters => ("Favorite character", "Best character"),
        DataType::Opponents => ("Most common opponent", "Easiest opponent"),
        DataType::Stages => ("Most played stage", "Best stage"),
    };
    let data_func = match d_type {
        DataType::Characters => num_to_char,
        DataType::Opponents => num_to_char,
        DataType::Stages => num_to_stage,
    };
    println!(
        "{}: {} ({} games)",
        data_labels.0,
        data_func(fb.favorite),
        data[fb.favorite].0
    );
    println!(
        "{}: {} ({:.2}% winrate)",
        data_labels.1,
        data_func(fb.best),
        (data[fb.best].0 as f64 / data[fb.best].1 as f64) * 100.0
    );
}

fn winrate_string(wins: usize, games: usize, standalone: bool) -> Option<String> {
    if games == 0 {
        return None;
    }
    let prefix: String;
    if standalone {
        prefix = String::from("Won");
    } else {
        prefix = String::from("won");
    }
    Some(format!(
        "{} {} of {} games ({:.2}%).",
        prefix,
        wins,
        games,
        (wins as f64 / games as f64) * 100.0
    ))
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

    if char_num >= 26 {
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

fn count_replays(path: &PathBuf) -> u64 {
    let mut count = 0;
    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().unwrap() == "slp" {
            count += 1;
        }
    }
    count
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
        //dummy results are so the function for printing data doesn't freak out
        0 => String::from("dummy"),
        1 => String::from("dummy"),
        2 => String::from("Fountain of Dreams"),
        3 => String::from("Pokémon Stadium"),
        4 => String::from("Princess Peach's Castle"),
        5 => String::from("Kongo Jungle"),
        6 => String::from("Brinstar"),
        7 => String::from("Corneria"),
        8 => String::from("Yoshi's Story"),
        9 => String::from("Onett"),
        10 => String::from("Mute City"),
        11 => String::from("Rainbow Cruise"),
        12 => String::from("Jungle Japes"),
        13 => String::from("Great Bay"),
        14 => String::from("Hyrule Temple"),
        15 => String::from("Brinstar Depths"),
        16 => String::from("Yoshi's Island"),
        17 => String::from("Green Greens"),
        18 => String::from("Fourside"),
        19 => String::from("Mushroom Kingdom I"),
        20 => String::from("Mushroom Kingdom II"),
        21 => String::from("dummy"),
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
