use std::process::Command;
use std::collections::HashMap;


type CallBack = fn() -> ();


/// Runs a given shell command
fn run_shell_command(command: &str){
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect(&("Failed to run command".to_owned() + command));

    let output = String::from_utf8(output.stdout).unwrap_or("Command not runned well".to_string());
    println!("{}", output);
}

/// Reads the shell yaml config file and executes commands
fn shell_command() {
    println!("Running shell commands defined in shell.yaml");
    println!("================================================================================");
    run_shell_command("ls /home/sergio");
}

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
    arg_parser.insert("--shell".to_string(), Box::new(shell_command));

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
