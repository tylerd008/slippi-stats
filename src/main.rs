mod gamedata;

use gamedata::{GameResults, GameResult};

use std::fs::File;

use peppi::parse;
use std::fs;
use std::path::PathBuf;

fn main() {
	let p = PathBuf::from("C:/Users/Flossy/Documents/Slippi");

	let mut results = GameResults::new();

	for entry in fs::read_dir(p).unwrap() {
		let path = entry.unwrap().path();
		let game = match peppi::game(&mut File::open(&path).unwrap(), Some(parse::Opts{skip_frames: false})){
			Ok(val) => val,
			Err(e) => {
				println!("Error {:?}, when parsing game: {:?}", e, path);
				continue;
			}
		};

		let result = match GameResult::parse_game(game) {
			Ok(g) => g,
			Err(e) => {
				println!("ererere: {:?}", e);
				continue;
			}
		};

		println!("{}", &result);
		results.add_game(result);
	}
}