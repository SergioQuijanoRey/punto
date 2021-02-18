use crate::YamlProcessor;
use std::collections::HashMap;
use std::env;
use std::process::{Command, Stdio, exit};

#[derive(Debug)]
pub struct CommandBlock {
    description: String,
    quiet: bool,
    commands: Vec<String>,
    sudo: bool,
}

impl CommandBlock {
    /// Creates a new CommandBlock
    pub fn new(description: String, quiet: bool, commands: Vec<String>, sudo: bool) -> Self {
        return CommandBlock {
            description,
            quiet,
            commands,
            sudo,
        };
    }

    /// Executes all commands of the command block
    pub fn execute(&self) {
        println!("Launching command {}", self.description);
        println!(
            "================================================================================"
        );

        for command in &self.commands {
            CommandBlock::run_shell_command(&command);
        }
    }

    /// Runs a given shell command in interactive mode
    /// It should never be called directly, thus is private
    /// TODO -- handle exceptions
    fn run_shell_command(command: &str) {
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
}

/// Handler to --shell cli argument
/// Reads the shell yaml config file and executes commands described in the yaml file
pub fn handle_shell_command(yaml_file: &str) {
    println!("Running shell commands defined in shell.yaml");
    println!("================================================================================");
    let command_blocks = parse_yaml_command(yaml_file);
    for command in command_blocks {
        command.execute();
    }
}

/// Given a yaml file path, it returns the CommandOptions vector which are used to launch a command
fn parse_yaml_command(file_path: &str) -> Vec<CommandBlock> {
    let parsed_contents = YamlProcessor::parse_yaml(file_path);
    // TODO -- this block of code is repeated
    let parsed_contents = match parsed_contents{
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Could not parse {}, exiting now", file_path);
            eprintln!("Error code was {}", err);
            exit(-1);
        }
    };

    let mut commands = vec![];

    // Getting the commands from the yaml file into struct
    for (_, value) in parsed_contents.as_hash().unwrap().iter() {
        // TODO -- panics here
        let vector_of_commands = value["commands"].as_vec().unwrap();
        let vector_of_commands: Vec<String> = vector_of_commands
            .into_iter()
            .map(|command| command.as_str().unwrap().to_string())
            .collect();

        commands.push(CommandBlock {
            description: value["description"]
                .as_str()
                .unwrap_or("No description provided")
                .to_string(),
            quiet: value["quiet"].as_bool().unwrap_or(false),
            commands: vector_of_commands,
            sudo: value["sudo"].as_bool().unwrap_or(false),
        });
    }

    println!("{:?}", commands);
    println!("Exiting the function");
    return commands;
}
