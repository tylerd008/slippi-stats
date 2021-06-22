mod input;
mod playerdata;
mod text;

use playerdata::PlayerData;

use std::env;
use std::path::PathBuf;

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
    input::main_loop(results);
}
