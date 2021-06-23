mod input;
mod playerdata;
mod text;

fn main() {
    loop {
        let results = input::load_data();
        if input::main_loop(results) {
            break;
        }
    }
}
