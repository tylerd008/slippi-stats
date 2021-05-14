mod gamedata;

use gamedata::{GameResult, GameResults};

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if env::args().len() < 3 {
        println!("Too few arguments. First argument is your netplay code, second is the directory where your replays are stored.");
        return;
    }

    let args: Vec<String> = env::args().collect();
    let np_code = args.get(1).unwrap();
    let p = PathBuf::from(args.get(2).unwrap());

    let mut results = GameResults::new();

    let mut count = 0;

    for entry in fs::read_dir(p).unwrap() {
        let path = entry.unwrap().path();
        match GameResult::has_player(&path, np_code.to_string()) {
            Ok(has_player) => {
                if !has_player {
                    println!("Game does not contain player. Skipping.");
                    continue;
                }
            }
            Err(e) => {
                println!("Error {:?}, when parsing game: {:?}", e, path);
                continue;
            }
        }

        let result = match GameResult::parse_game(path, np_code.to_string()) {
            Ok(g) => g,
            Err(e) => {
                println!("Error when parsing game result: {:?}", e);
                continue;
            }
        };

        //println!("{}", &result);
        results.add_game(result);
        count += 1;
        if count % 50 == 0 {
            println!("Processed game number: {}", count);
        }
    }
    println!("win percent: {0:.2}", results.win_percentage() * 100.0);
}
