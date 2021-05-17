mod gamedata;

use gamedata::{GameResult, GameResults};

use std::env;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

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
    //println!("win percent: {0:.2}", results.total_win_percentage() * 100.0);
}
