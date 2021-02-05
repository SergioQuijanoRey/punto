use std::collections::HashMap;

mod CommandProcessor;

type CallBack = fn() -> ();

fn install_command() {
    println!("Installing packages")
}

fn not_found_command(command: String) {
    println!("Command {} is not valid", command);
    show_help();
}

fn show_help(){
    println!("Punto usage:");
    println!("\tpunto --install: install all packages, defined in install.yml");
    println!("\tpunto --shell: run custom shell scripts, defined in shell.yml");
    println!("\tpunto --all: run all of above commands");
}


fn parse_args(){
    // Arguments and their callbacks
    // TODO -- convert this into a struct with methods for easier use
    let mut arg_parser: HashMap<String, Box<CallBack> > = HashMap::new();
    arg_parser.insert("--install".to_string(), Box::new(install_command));
    arg_parser.insert("--shell".to_string(), Box::new(CommandProcessor::shell_command));

    // Iterate over given arguments
    let args = std::env::args();
    for (indx, arg) in args.enumerate() {
        // First arg is the cli app name
        if indx == 0{
            continue;
        }

        let command = arg_parser.get(&arg);
        match command{
            Some(command) => (command)(),
            None => not_found_command(arg),
        };
    }
}

fn main() {
    parse_args();
}
