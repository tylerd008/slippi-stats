/* #[feature(trace_macros)]

trace_macros!(true); */

#[macro_export]
macro_rules! command_loop {
    ($break_at_end:expr, $ ($cmd:expr, $cmd_help_text:expr => $result:expr),*) => {
        let mut cmds = Vec::new();
        $ (cmds.push(format!("{}, ", stringify!($cmd)));) *
        let help_txt = format_help_txt(cmds);
        println!("{}", help_txt);
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            let input = format_input(input);
            if &input[..] == "help"{
                println!("{}", help_txt);
                println!("Type `help` followed by a command name to get info on that command.");
                continue;
            } $(else if &input[..] == $cmd {
                $result
            } else if &input[..] == &format!("help {}", $cmd){
                println!("{}", $cmd_help_text);
                continue;
            })*
            else {
                println!("Unrecognized command.");
                continue;
            }
            if $break_at_end{//this is so we can keep the main input loop running, while ending the others after a subcommand is ran
                break;
            }
            println!("{}", help_txt);//this is so the main input loop will have its commands reprinted after a loop, so the user is aware they've returned back to it
        }
    };
}

#[macro_export]
macro_rules! input_loop {
    ($output:ty) => {{
        let arg: $output;
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            let input = &format_input(input);
            arg = match input.parse() {
                Ok(fs) => fs,
                Err(_) => {
                    println!("Unrecognized input!");
                    continue;
                }
            };
            break;
        }
        arg
    }};
}
