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
        }
    };
}

#[macro_export] //idk if this can work but it feels close so i'm just gonna leave it here for now and come back to it every now and then until i figure it out/give up
macro_rules! input_loop {
    ($ ($name:ident),*) => {
        $ (fn $name() -> ArgType{
            let $name: ArgType = ArgType::Character(0);
            loop {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("failed to read line");
                let arg = match parse_arg(&format_input(input)) {
                    Some(a) => a,
                    None => {
                        println!("Unrecognized {}", stringify!($name));
                        continue;
                    }
                };
                let $name = match arg {
                    ArgType::$name(num) => ArgType::$name(num),
                    _ => {
                        continue;
                    }
                };
                break;
            }
            println!("{:?}", $name);
            $name
        })*
    };
}
