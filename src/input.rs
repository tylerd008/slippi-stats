use crate::text;
use crate::{command_loop, input_loop};
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

use crate::playerdata::{ArgType, PlayerData};

use crate::character::Character;
use crate::stage::Stage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct NetplayCode {
    name: String,
    num: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct CacheLocation {
    np_code: NetplayCode,
    path: PathBuf,
}

enum NetplayCodeParseError {
    InvalidCode,
}

pub fn load_data() -> PlayerData {
    let cl = match fs::read_to_string("data.cache") {
        Ok(c) => {
            println!("Cache found, loading...");
            serde_json::from_str(&c).unwrap()
        }
        Err(_) => input_data(),
    };
    PlayerData::parse_dir(cl.path, format!("{}", cl.np_code))
}

fn input_data() -> CacheLocation {
    println!("Please input your np code:");
    let np_code = input_loop!(NetplayCode);
    println!("Enter the directory where your replays are stored:");
    let path = input_loop!(PathBuf);
    let cl = CacheLocation { np_code, path };
    let serial = serde_json::to_string(&cl).unwrap();
    match fs::write("data.cache", serial) {
        Ok(_) => println!("Data saved."),
        Err(e) => println!("Cache location could not be saved do to error `{:?}`", e),
    };
    cl
}

pub fn main_loop(results: PlayerData) -> bool {
    command_loop!(
        false,
        "player", text::PLAYER_HELP_TEXT => player(&results),
        "character", text::CHARACTER_HELP_TEXT => character(&results),
        "stage", text::STAGE_HELP_TEXT => stage(&results),
        "matchup", text::MATCHUP_HELP_TEXT => matchup(&results),
        "last", text::LAST_HELP_TEXT => last(&results),
        "change cache", text::CHANGECACHE_HELP_TEXT => {
            change_cache();
            return false;
        },
        "end", text::END_HELP_TEXT => {
            break;
        }
    );
    return true;
}

fn change_cache() {
    match fs::remove_file("data.cache") {
        Ok(_) => println!("Prevous cache location removed."),
        Err(e) => {
            println!(
                "Couldn't delete previous cache location data due to error `{:?}`",
                e
            );
            return;
        }
    }
    /* let cl_new = input_data();
    main_loop(PlayerData::parse_dir(
        cl_new.path,
        format!("{}", cl_new.np_code),
    )); */
}

fn player(data: &PlayerData) { //not sure how i want to do this with the new framework
    /* command_loop!(
        true,
        "winrate", text::P_WINRATE_HELP_TEXT => data.winrate(&ArgType::Player),
        //"characters", text::PLACEHOLDER_TEXT => data.characters(), not sure how i want to implement these right now
        //"stages", text::PLACEHOLDER_TEXT => data.stages(),
        "matchups", text::P_MATCHUPS_HELP_TEXT => data.matchups(&ArgType::Player),
        "overview", text::P_OVERVIEW_HELP_TEXT => data.overview()
    ); */
}

fn character(data: &PlayerData) {
    println!("Input the name of a character.");
    let character = input_loop!(Character);
    command_loop!(
        true,
        "winrate", text::C_WINRATE_HELP_TEXT => data.winrate(character),
        "stages", text::C_STAGES_HELP_TEXT => data.stages(character),
        "matchups", text::C_MATCHUPS_HELP_TEXT => data.matchups(character)
    );
}

fn stage(data: &PlayerData) {
    let stage = input_loop!(Stage);
    command_loop!(
        true,
        "winrate", text::S_WINRATE_HELP_TEXT => data.winrate(stage),
        "characters", text::S_CHARACTERS_HELP_TEXT => data.characters(stage),
        "matchups", text::S_MATCHUPS_HELP_TEXT => data.matchups(stage)
    );
}

fn matchup(data: &PlayerData) {
    println!("Input player character:");
    let player_char = input_loop!(Character);
    println!("Input opponent character:");
    let opponent_char = input_loop!(Character);
    data.matchup(player_char, opponent_char);
}

fn last(data: &PlayerData) {
    println!("Last how many games?");
    let num = input_loop!(usize);
    data.last(num);
}

fn format_input(arg: String) -> String {
    let arg = arg.trim();
    arg.to_lowercase()
}

fn format_help_txt(cmds: Vec<String>) -> String {
    let mut help_txt = String::from("The available commands are ");
    for i in 0..cmds.len() {
        if i == cmds.len() - 1 {
            help_txt.push_str("and ");
        }
        help_txt.push_str(&cmds[i]);
    }
    help_txt
}

impl fmt::Display for NetplayCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}#{}", self.name, self.num)
    }
}

impl FromStr for NetplayCode {
    type Err = NetplayCodeParseError;
    fn from_str(np_str: &str) -> Result<Self, Self::Err> {
        if !np_str.contains("#") || np_str.len() != 8 {
            return Err(NetplayCodeParseError::InvalidCode);
        }
        let v: Vec<&str> = np_str.split_terminator("#").collect();
        let num: usize = match v[1].parse() {
            Ok(n) => n,
            Err(_) => {
                return Err(NetplayCodeParseError::InvalidCode);
            }
        };
        Ok(Self {
            name: v[0].to_string().to_uppercase(),
            num,
        })
    }
}

#[macro_export]
macro_rules! command_loop {
    ($break_at_end:expr, $ ($cmd:expr, $cmd_help_text:expr => $result:expr),*) => {
        let mut cmds = Vec::new();
        $ (cmds.push(format!("{}, ", stringify!($cmd)));) *
        let help_txt = format_help_txt(cmds);
        println!("{}", help_txt);
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            let input = format_input(input);
            match &input[..]{
	            $($cmd => $result,)*
	            "help" => {
		            println!("{}", help_txt);
		            println!("Type `help` followed by another command to get more info on that command.");
		            continue;
		            },
	            $(x if x == &format!("help {}", $cmd) => {
		            println!("{}", $cmd_help_text);
		            continue;
		            },)*
	            _ => {
		            println!("Unrecognized command");
		            continue;
		        }
            }
            if $break_at_end{//this is so we can keep the main input loop running, while ending the others after a subcommand is ran
                break;
            }
            println!("{}", help_txt);//this is so the main input loop will have its commands reprinted after a loop, so the user is aware they've returned back to it
        }
    };
}

#[macro_export]
macro_rules! input_loop {
    ($output:ty) => {{
        let arg: $output;
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            let input = &format_input(input);
            arg = match input.parse() {
                Ok(fs) => fs,
                Err(_) => {
                    println!("Unrecognized input!");
                    continue;
                }
            };
            break;
        }
        arg
    }};
}
