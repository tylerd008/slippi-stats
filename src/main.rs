mod gamedata;

use gamedata::{GameResult, GameResults};

use std::fs::File;

use peppi::parse;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if env::args().len() < 3 {
        println!("Too few arguments");
        return;
    }

    let args: Vec<String> = env::args().collect();
    let np_code = args.get(1).unwrap();
    let p = PathBuf::from(args.get(2).unwrap());

    let mut results = GameResults::new();

    for entry in fs::read_dir(p).unwrap() {
        let path = entry.unwrap().path();
        let game = match peppi::game(
            &mut File::open(&path).unwrap(),
            Some(parse::Opts { skip_frames: false }),
        ) {
            Ok(val) => val,
            Err(e) => {
                println!("Error {:?}, when parsing game: {:?}", e, path);
                continue;
            }
        };

        let result = match GameResult::parse_game(game, np_code.to_string()) {
            Ok(g) => g,
            Err(e) => {
                println!("Error when parsing game result: {:?}", e);
                continue;
            }
        };

        println!("{}", &result);
        results.add_game(result);
    }
}
