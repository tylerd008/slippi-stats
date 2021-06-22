use crate::text;
use crate::{command_loop, input_loop};
use std::io;

use crate::playerdata::{ArgType, PlayerData};

pub fn main_loop(results: PlayerData) {
    command_loop!(
        false,
        "player", text::PLAYER_HELP_TEXT => player(&results),
        "character", text::CHARACTER_HELP_TEXT => character(&results),
        "stage", text::STAGE_HELP_TEXT => stage(&results),
        "matchup", text::MATCHUP_HELP_TEXT => matchup(&results),
        "last", text::LAST_HELP_TEXT => last(&results),
        "end", text::END_HELP_TEXT => {
            break;
        }
    );
}

fn player(data: &PlayerData) {
    command_loop!(
        true,
        "winrate", text::P_WINRATE_HELP_TEXT => data.winrate(&ArgType::Player),
        //"characters", text::PLACEHOLDER_TEXT => data.characters(), not sure how i want to implement these right now
        //"stages", text::PLACEHOLDER_TEXT => data.stages(),
        "matchups", text::P_MATCHUPS_HELP_TEXT => data.matchups(&ArgType::Player),
        "overview", text::P_OVERVIEW_HELP_TEXT => data.overview()
    );
}

fn character(data: &PlayerData) {
    println!("Input the name of a character.");
    let character = char_loop();
    command_loop!(
        true,
        "winrate", text::C_WINRATE_HELP_TEXT => data.winrate(&character),
        "stages", text::C_STAGES_HELP_TEXT => data.stages(&character),
        "matchups", text::C_MATCHUPS_HELP_TEXT => data.matchups(&character)
    );
}

fn stage(data: &PlayerData) {
    let stage = stage_loop();
    command_loop!(
        true,
        "winrate", text::S_WINRATE_HELP_TEXT => data.winrate(&stage),
        "characters", text::S_CHARACTERS_HELP_TEXT => data.characters(&stage),
        "matchups", text::S_MATCHUPS_HELP_TEXT => data.matchups(&stage)
    );
}

fn matchup(data: &PlayerData) {
    println!("Input player character:");
    let player_char = char_loop();
    println!("Input opponent character:");
    let opponent_char = char_loop();
    data.matchup(player_char, opponent_char);
}

fn last(data: &PlayerData) {
    println!("Last how many games?");
    let num = input_loop!(usize);
    data.last(num);
}

fn char_loop() -> ArgType {
    let character: ArgType;
    loop {
        let arg = input_loop!(ArgType);
        character = match arg {
            ArgType::Character(num) => ArgType::Character(num),
            _ => {
                println!("Please input a character name.");
                continue;
            }
        };
        break;
    }
    character
}

fn stage_loop() -> ArgType {
    let stage: ArgType;
    println!("Input the name of a stage.");
    loop {
        let arg = input_loop!(ArgType);
        stage = match arg {
            ArgType::Stage(num) => ArgType::Stage(num),
            _ => {
                println!("Please input a stage name.");
                continue;
            }
        };
        break;
    }
    stage
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
            if &input[..] == "help"{
                println!("{}", help_txt);
                println!("Type `help` followed by a command name to get info on that command.");
                continue;
            } $(else if &input[..] == $cmd {
                $result
            } else if &input[..] == &format!("help {}", $cmd){
                println!("{}", $cmd_help_text);
                continue;
            })*
            else {
                println!("Unrecognized command.");
                continue;
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
