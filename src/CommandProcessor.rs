use crate::YamlProcessor;
use std::collections::HashMap;
use std::env;
use std::process::{Command, Stdio};

// TODO -- struct fields have to be private and methods need to be implemented
// TODO -- change command to commands: Vec<String>
#[derive(Debug)]
pub struct CommandBlock {
    pub description: String,
    pub quiet: bool,
    pub commands: Vec<String>,
    pub sudo: bool,
}

/// Runs a given shell command in interactive mode
/// TODO -- handle exceptions
fn run_shell_command(command: &str){
    let user_env_vars: HashMap<String, String> = env::vars().collect();
    let mut command_handle = Command::new("bash")
        .env_clear()
        .envs(user_env_vars)
        .stdin(Stdio::inherit())
        .arg("-c")
        .arg(command)
        .arg("-Syyu")
        .spawn()
        .expect(&("Failed to run command".to_owned() + command));

    // Wait for the command
    command_handle.wait().unwrap();
 }

/// Reads the shell yaml config file and executes commands
pub fn shell_command() {
    println!("Running shell commands defined in shell.yaml");
    println!("================================================================================");
    let commands = YamlProcessor::parse_yaml_command("/home/sergio/GitProjects/punto/shell.yaml");
    for command in commands {
        launch_command(command);
    }
}

fn launch_command(command_block: CommandBlock) {
    println!("Launching command {}", command_block.description);
    println!("================================================================================");

    for command in command_block.commands{
        run_shell_command(&command);
    }
}
