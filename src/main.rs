mod input;
mod playerdata;
mod text;

fn main() {
    let results = input::load_data();
    input::main_loop(results);
}
