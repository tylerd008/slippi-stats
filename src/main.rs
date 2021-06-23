use slippi_stats::input;

fn main() {
    loop {
        let results = input::load_data();
        if input::main_loop(results) {
            break;
        }
    }
}
