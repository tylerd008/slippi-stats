mod gamedata;
mod macros;
mod text;

use gamedata::{ArgType, GameResults};

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

    let results = match GameResults::parse_dir(p, np_code.to_string()) {
        Ok(r) => r,
        Err(e) => {
            println!("error {:?} parsing gameresults", e);
            return;
        }
    };
    command_loop!(
        false,
        "The available commands are `character`, `stage`, `matchup`, and `end`.",
        "character", "adf" => character(&results),
        "stage", "text::STAGE_HELP_TEXT" => stage(&results),
        "matchup", "text::MATCHUP_HELP_TEXT" => matchup(&results),
        "end", "text::END_HELP_TEXT" => {
            break;
        }
    );
}

fn character(data: &GameResults) {
    println!("Input the name of a character.");
    let character = char_loop();
    command_loop!(
        true,
        "The available commands are `winrate`, `stages`, and `matchups`.",
        "winrate", text::C_WINRATE_HELP_TEXT => data.winrate(&character),
        "stages", text::C_STAGES_HELP_TEXT => data.stages(&character),
        "matchups", text::C_MATCHUPS_HELP_TEXT => data.matchups(&character)
    );
}

fn stage(data: &GameResults) {
    let stage = stage_loop();
    command_loop!(
        true,
        "The available commands are `winrate`, `characters`, and `matchups`.",
        "winrate", text::S_WINRATE_HELP_TEXT => data.winrate(&stage),
        "characters", text::S_CHARACTERS_HELP_TEXT => data.characters(&stage),
        "matchups", text::S_MATCHUPS_HELP_TEXT => data.matchups(&stage)
    );
}

fn matchup(data: &GameResults) {
    println!("Input player character:");
    let player_char = char_loop();
    println!("Input opponent character:");
    let opponent_char = char_loop();
    data.matchup(player_char, opponent_char);
}

fn char_loop() -> ArgType {
    let character: ArgType;
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let arg = match parse_arg(&format_input(input)) {
            Some(a) => a,
            None => {
                println!("Unrecognized character.");
                continue;
            }
        };

        character = match arg {
            ArgType::Character(num) => ArgType::Character(num),
            ArgType::Stage(_) => {
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
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let arg = match parse_arg(&format_input(input)) {
            Some(a) => a,
            None => {
                println!("Unrecognized stage.");
                continue;
            }
        };

        stage = match arg {
            ArgType::Stage(num) => ArgType::Stage(num),
            ArgType::Character(_) => {
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

fn parse_arg(arg: &str) -> Option<ArgType> {
    let arg = match &format_input(arg.to_string())[..] {
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
            return None;
        }
    };
    Some(arg)
}
