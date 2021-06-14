#[macro_export]
macro_rules! command_loop {
    ($break_at_end:expr, $help_text:expr, $ ($cmd:expr, $cmd_help_text:expr => $result:expr),*) => {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            let input = format_input(input);
            let help_txt = String::from("The available commands are ");
            $ (help_txt.push_str(format!("`{}` ", stringify!($cmd)));) *
            match &input[..] {
                $($cmd => $result, format!("help{}", $cmd) => println!("{}", $cmd_help_text),)*
                "help" => {println!("{}", help_txt);
                    continue;
                    },
                //$(format!("help{}", $cmd) => println!("{}", $cmd_help_text),)*
                _ => println!("Unrecognized command."),
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
