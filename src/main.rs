mod gamedata;

use std::path::Path;
use std::fs::File;

use peppi::parse;
use peppi::character;
use std::fs;
use std::path::PathBuf;
use std::io;
use peppi::metadata::{Player, Netplay};
use peppi::game::Game;
use peppi::game::Frames;
use peppi::frame::Frame2;

#[derive(Debug)]
struct GameData{

}

fn main() {
	let p = PathBuf::from("C:/Users/Flossy/Documents/Slippi");

	let mut char_vec = vec![0; 33];
	let mut stage_count = vec![0; 33];
	let char_name_vec = vec!["Captain Falcon", "Donkey Kong", "Fox", "Mr. Game and Watch", "Kirby", "Bowser", "Link", "Luigi", "Mario", "Marth", "Mewtwo", "Ness", "Peach", "Pikachu", "Ice Climbers", "Jigglypuff", "Samus", "Yoshi", "Zelda", "Sheik", "Falco", "Young Link", "Dr. Mario", "Roy", "Pichu", "Gandondorf", "Master Hand", "Male Wire Frame", "Female Wire Frame", "Giga Bowser", "Crazy Hand", "Sandbag", "Popo"];
	let stage_names = vec!["undefined", "undefined", "Fountain of Dreams", "Pokemon Stadium", "Princess Peach's Castle", "Kongo Jungle", "Brinstar", "Corneria", "Yoshi's Story", "Onett", "Mute City", "Rainbow Cruise", "Jungle Japes", "Great Bay", "Hyrule Temple", "Brinstar Depths", "Yoshi's Island", "Green Greens", "Fourside", "Mushroom Kingdom I", "Mushroom Kingdom II", "undefined", "Venom", "Oke Floats", "Big Blue", "Icicle Mountain", "Icetop", "Flat Zone", "Dream Land N64", "Yoshi's Island N64", "Kongo Jungle N64", "Battlefield", "Final Destination"];
	let mut tot_games: usize = 0;
	let mut win_count: usize = 0;

	for entry in fs::read_dir(p).unwrap() {
		let path = entry.unwrap().path();
		let game = match peppi::game(&mut File::open(&path).unwrap(), Some(parse::Opts{skip_frames: false})){
			Ok(val) => val,
			Err(e) => {
				println!("Error {:?}, when parsing game: {:?}", e, path);
				continue;
			}
		};
		let frames = &game.frames;
		let data = match frames {
			Frames::P2(d) => d,
			_ => {
				continue;//ignore games with more than 2 players
			}
		};
		let stage = game.start.stage.0 as usize;
		let stage_num = if stage > 32 {
			continue;//skip replays with corrupted stage data
		} else {
			stage
		};
		stage_count[stage_num] += 1;
		let player_num = match get_player_num(&game){
			Some(p) => p,
			None => {//skip replays with corrupted character data
				continue;
			},
		};
		if is_victory(&data, player_num){
			win_count += 1;
		}
		let player_char = game.start.players.get(player_num).unwrap().character.0;
		if player_char > 25{
			//println!("game: {:?}", &path);
		}
		char_vec[player_char as usize] += 1;
		tot_games += 1;
		if tot_games % 50 == 0{
			println!("processed game {}", tot_games);
		}
	}
	for i in 0..26 {
		if char_vec[i] == 0{
			continue;
		}
		println!("{}: {} games",char_name_vec[i], char_vec[i]);
	}

	for i in 1..33 {
		if stage_count[i] == 0 || i == 1 || i == 21{
			continue;
		}
		println!("{}: {} games", stage_names[i], stage_count[i]);
	}

	println!("Won {} out of {} games", win_count, tot_games);
}

fn get_player_num(game: &Game) -> Option<usize>{//rewrite this to return player number, not the player struct
	let players = game.metadata.players.as_ref().unwrap();
	let p2_md = players.get(1).unwrap();
	let p2_np_code = match &p2_md.netplay{
		Some(c) => &c.code,
		None => {
			return None;
		}
	};
	if p2_np_code == "FLOS#497"{
		Some(1)
	} else {
		Some(0)
	}
}

fn is_victory(data: &Vec<Frame2>, player_num: usize) -> bool{
	let pp_lf = &data[data.len()-1].ports[player_num];//player port last frame

	let op_lf = &data[data.len()-1].ports[1 - player_num];//opponent player last frame

	let p_end_stocks = pp_lf.leader.post.stocks;
	let o_end_stocks = op_lf.leader.post.stocks;

	if p_end_stocks > o_end_stocks{
		true
	} else if p_end_stocks < o_end_stocks {
		false
	} else {
		pp_lf.leader.post.damage < op_lf.leader.post.damage//this doesn't take into account what happens if damage is equal at the end but that requires a timeout, 
		//which is already rare enough, and having equal damage at the end, (esp since its stored to higher precision than an iteger) which is even rarer, so i'll just leave this for now
	}
}