use std::process::Command;
use std::env;
use std::fs;
use std::collections::HashMap;
use yaml_rust::YamlLoader;

#[derive(Debug)]
struct CommandOptions{
    description: String,
    quiet: bool,
    command: String,
    sudo: bool,
}

/// Runs a given shell command
/// TODO -- We are loosing the colors of the output
fn run_shell_command(command: &str) -> String{
    let user_env_vars : HashMap<String, String> = env::vars().collect();
    let output = Command::new("bash")
        .env_clear()
        .envs(user_env_vars)
        .arg("-c")
        .arg(command)
        .output()
        .expect(&("Failed to run command".to_owned() + command));

    let output = String::from_utf8(output.stdout).unwrap_or("Command not runned well".to_string());
    return output;
}

/// Reads the shell yaml config file and executes commands
pub fn shell_command() {
    println!("Running shell commands defined in shell.yaml");
    println!("================================================================================");
    let commands = parse_yaml_command("/home/sergio/GitProjects/punto/shell.yaml");
    for command in commands{
        launch_command(command);
    }
}

fn launch_command(command: CommandOptions){
    println!("Launching command {}", command.description);
    println!("================================================================================");

    let output = run_shell_command(&command.command);
    if command.quiet == false{
        println!("{}", output);
    }
}

// TODO -- this has to go into YamlProcessor.rs
fn parse_yaml(file_path: &str) -> yaml_rust::Yaml {
    // Opening yaml file and parsing it
    let contents = fs::read_to_string(file_path).unwrap();
    let parsed_contents = YamlLoader::load_from_str(&contents).unwrap();
    let parsed_contents = parsed_contents[0].clone();
    return parsed_contents;
}

/// Given a yaml file path, it returns the CommandOptions vector which are used to launch a command
fn parse_yaml_command(file_path: &str) -> Vec<CommandOptions> {
    let parsed_contents = parse_yaml(file_path);

    // Getting the commands from the yaml file into struct
    let mut commands = vec![];
    for (_, value) in parsed_contents.as_hash().unwrap().iter(){
        commands.push(CommandOptions{
            description: value["description"].as_str().unwrap_or("No description provided").to_string(),
            quiet: value["quiet"].as_bool().unwrap_or(false),
            command: value["command"].as_str().unwrap().to_string(), // Only mandatory field for command
            sudo: value["sudo"].as_bool().unwrap_or(false),
        });
    }

    return commands;
}
