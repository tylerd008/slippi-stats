mod macros;
mod playerdata;
mod text;

use playerdata::{ArgType, PlayerData};

use std::env;
use std::path::PathBuf;

use std::io;

fn main() {
    if env::args().len() < 3 {
        println!("Too few arguments. First argument is your netplay code, second is the directory where your replays are stored.");
        return;
    }

    let args: Vec<String> = env::args().collect();
    let np_code = args.get(1).unwrap();
    let p = PathBuf::from(args.get(2).unwrap());

    let results = match PlayerData::parse_dir(p, np_code.to_string()) {
        Ok(r) => r,
        Err(e) => {
            println!("error {:?} parsing PlayerData", e);
            return;
        }
    };
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
