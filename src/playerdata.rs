use std::fmt;
use std::fs;
use std::time::Instant;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use indicatif::{HumanDuration, ProgressBar, ProgressStyle};

use crate::gamedata::GameData;

use crate::character::Character;
use crate::stage::Stage;

use std::fmt::Display;

use crate::parsable_enum::{GameDataCondition, Numbered, Parsable};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerData {
    cache_ver: usize,
    results: Vec<GameData>,
}

enum DataType {
    Stages,
    Characters,
    Opponents,
}

#[derive(Debug)]
struct FavBestData {
    favorite: usize,
    best: usize,
}

#[derive(Clone)]
struct WinLossData {
    games: usize,
    wins: usize,
}

type AssociatedTryFromError<T> = <T as std::convert::TryFrom<usize>>::Error;

struct WinLossVec<T: Parsable + Numbered>
where
    AssociatedTryFromError<T>: std::fmt::Debug,
{
    data: Vec<WinLossData>,
    parser: fn(usize) -> Result<T, T::Error>,
}

impl WinLossData {
    fn new() -> WinLossData {
        Self { games: 0, wins: 0 }
    }

    fn add_game(&mut self, is_win: bool) {
        self.games += 1;
        if is_win {
            self.wins += 1;
        }
    }

    fn winrate(&self) -> f64 {
        (self.wins as f64) / (self.games as f64) * 100.0
    }
}

impl Display for WinLossData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Won {} of {} games. ({:.2}%).",
            self.wins,
            self.games,
            self.winrate()
        )
    }
}

impl<T: Parsable + Numbered> WinLossVec<T>
where
    AssociatedTryFromError<T>: std::fmt::Debug,
{
    fn new() -> Self {
        Self {
            data: vec![WinLossData::new(); T::NUM_VALUES],
            parser: T::try_from,
        }
    }

    fn add_game(&mut self, is_win: bool, elt_num: usize) {
        //todo: figure out a way to use a generic into usize with this so every call doesn't involve `as usize`
        self.data[elt_num].add_game(is_win)
    }

    fn is_empty(&self) -> bool {
        let mut tot_games = 0;
        for asdf in &self.data {
            tot_games += asdf.games;
        }
        tot_games == 0
    }

    fn fav_best(&self) -> FavBestData {
        let mut favorite = 0;
        let mut best = 2; //default this to 0 because the way things are set up right now this causes issues with low game counts as stage 0 does not exist
        let mut best_winrate = 0.0;
        for i in 0..self.data.len() {
            if self.data[i].games > self.data[favorite].games {
                favorite = i;
            }
            let current_winrate = self.data[i].winrate();
            if (self.data[i].games > 20) && (current_winrate > best_winrate) {
                //min 20 games so things with 1 game and 1 win don't end up taking the spot (will change this to be percent based once I decide on an appropriate percent)
                best = i;
                best_winrate = current_winrate;
            }
        }
        FavBestData { favorite, best }
    }

    fn print_fb_data(&self, d_type: DataType, fb: FavBestData) {
        let data_labels = match d_type {
            DataType::Characters => ("Favorite character", "Best character"),
            DataType::Opponents => ("Most common opponent", "Easiest opponent"),
            DataType::Stages => ("Most played stage", "Best stage"),
        };
        let parser = self.parser;
        println!(
            "{}: {} ({} games)",
            data_labels.0,
            parser(fb.favorite).unwrap(),
            self.data[fb.favorite].games
        );
        println!(
            "{}: {} ({:.2}% winrate)",
            data_labels.1,
            parser(fb.best).unwrap(),
            self.data[fb.best].winrate()
        );
    }
}

impl<T: Parsable + Numbered> Display for WinLossVec<T>
where
    AssociatedTryFromError<T>: std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "No data for given input.");
        }
        let mut output = String::from("");
        for i in 0..self.data.len() {
            if self.data[i].games == 0 {
                continue;
            }
            output.push_str(&format!("{}: {}\n", T::try_from(i).unwrap(), self.data[i]));
            //unwrap cause if the parser returns an error then something went wrong somewhere else
        }
        write!(f, "{}", output)
    }
}

impl PlayerData {
    const CACHE_VER: usize = 8;
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            cache_ver: PlayerData::CACHE_VER,
        }
    }

    pub fn parse_dir(p: PathBuf, np_code: String) -> Self {
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
            let game_metadata = match GameData::get_game_data(&path, true) {
                Ok(gd) => gd,
                Err(e) => {
                    pb.println(format!("Error {:?} when parsing game {:?}", e, path));
                    pb.inc(1);
                    continue;
                }
            };
            let dt = serde_json::to_string(&game_metadata.metadata.date.unwrap()).unwrap();
            if cache.contains(&dt) {
                //not sure what's better: this, or loading the deserialized data and then iterating through it and checking each gamedata
                pb.inc(1);
                continue;
            }
            let gamedata_with_frames = match GameData::get_game_data(&path, false) {
                Ok(gd) => gd,
                Err(e) => {
                    pb.println(format!("Error {:?} when parsing game {:?}", e, path));
                    pb.inc(1);
                    continue;
                }
            };
            let result = match GameData::parse_game(gamedata_with_frames, np_code.to_string()) {
                Ok(g) => g,
                Err(e) => {
                    pb.println(format!("Error {:?}, when parsing game: {:?}", e, path));
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
        results
    }

    pub fn add_game(&mut self, game: GameData) {
        self.results.push(game);
    }

    pub fn winrate<T: GameDataCondition + fmt::Display>(&self, arg: T) {
        let mut win_loss_data = WinLossData::new();

        for game in &self.results {
            if arg.game_data_condition(game) {
                win_loss_data.add_game(game.is_victory());
            }
        }
        println!("{}:\n{}", arg, win_loss_data);
    }

    pub fn matchups<T: GameDataCondition + Display>(&self, arg: T) {
        let mut matchup_data = WinLossVec::<Character>::new();

        for game in &self.results {
            if arg.game_data_condition(game) {
                matchup_data.add_game(game.is_victory(), game.opponent_char as usize);
            }
        }
        println!("{}:\n{}", arg, matchup_data);
    }

    pub fn stages<T: GameDataCondition + Display>(&self, arg: T) {
        let mut stage_data = WinLossVec::<Stage>::new();

        for game in &self.results {
            if arg.game_data_condition(game) {
                stage_data.add_game(game.is_victory(), game.stage as usize);
            }
        }
        println!("{}:\n{}", arg, stage_data);
    }

    pub fn characters<T: GameDataCondition + Display>(&self, arg: T) {
        let mut char_data = WinLossVec::<Character>::new();

        for game in &self.results {
            if arg.game_data_condition(game) {
                char_data.add_game(game.is_victory(), game.player_char as usize);
            }
        }
        println!("{}:\n{}", arg, char_data);
    }

    pub fn matchup(&self, player: Character, opponent: Character) {
        let mut stage_data = WinLossVec::<Stage>::new();

        for game in &self.results {
            if game.player_char == player && game.opponent_char == opponent {
                stage_data.add_game(game.is_victory(), game.stage as usize);
            }
        }
        println!("{} vs. {}:\n{}", player, opponent, stage_data);
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
        let mut char_data = WinLossVec::<Character>::new();
        let mut opponent_data = WinLossVec::<Character>::new();
        let mut stage_data = WinLossVec::<Stage>::new();
        for game in &self.results {
            char_data.add_game(game.is_victory(), game.player_char as usize);
            opponent_data.add_game(game.is_victory(), game.opponent_char as usize);
            stage_data.add_game(game.is_victory(), game.stage as usize);
        }

        char_data.print_fb_data(DataType::Characters, char_data.fav_best());
        opponent_data.print_fb_data(DataType::Opponents, opponent_data.fav_best());
        stage_data.print_fb_data(DataType::Stages, stage_data.fav_best());
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
