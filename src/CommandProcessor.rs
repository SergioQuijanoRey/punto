use std::process::Command;
use std::env;
use std::collections::HashMap;
use crate::YamlProcessor;

// TODO -- struct fields have to be private and methods need to be implemented
#[derive(Debug)]
pub struct CommandOptions{
    pub description: String,
    pub quiet: bool,
    pub command: String,
    pub sudo: bool,
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
    let commands = YamlProcessor::parse_yaml_command("/home/sergio/GitProjects/punto/shell.yaml");
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

