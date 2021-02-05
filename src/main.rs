use std::process::Command;
use std::env;
use std::fs;
use std::collections::HashMap;
use yaml_rust::{YamlLoader, YamlEmitter};


type CallBack = fn() -> ();


/// Runs a given shell command
/// TODO -- We are loosing the colors of the output
fn run_shell_command(command: &str){
    let user_env_vars : HashMap<String, String> = env::vars().collect();
    let output = Command::new("bash")
        .env_clear()
        .envs(user_env_vars)
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
    parse_yaml("/home/sergio/GitProjects/punto/shell.yaml");
    //run_shell_command("git log --oneline --color");
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

#[derive(Debug)]
struct CommandOptions{
    description: String,
    quiet: bool,
    command: String,
    sudo: bool,
}

/// Given a yaml file path, it returns the CommandOptions vector which are used to launch a command
fn parse_yaml(file_path: &str) -> Vec<CommandOptions> {
    println!("We are parsing {}", file_path);
    println!("But we are doing nothing by the moment");

    // Opening yaml file and parsing it
    let contents = fs::read_to_string(file_path).unwrap();
    let parsed_contents = YamlLoader::load_from_str(&contents).unwrap();
    let parsed_contents = &parsed_contents[0];

    // Getting the commands from the yaml file into struct
    let mut commands = vec![];
    for (_, value) in parsed_contents.as_hash().unwrap().iter(){
        commands.push(CommandOptions{
            description: value["description"].as_str().unwrap().to_string(),
            quiet: value["quiet"].as_bool().unwrap(),
            command: value["command"].as_str().unwrap().to_string(),
            sudo: value["sudo"].as_bool().unwrap(),
        });
    }

    println!("Vector of commands is {:?}", commands);
    return commands;
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
