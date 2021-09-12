use crate::YamlProcessor;
use std::collections::HashMap;
use std::env;
use std::process::{Command, Stdio, exit};
use std::fmt;
use std::io::Error;

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
    // TODO -- BUG -- needs to return Result<(), Err>
    pub fn execute(&self) -> Result<(), CommandError>{
        println!("Launching command {}", self.description);
        println!(
            "================================================================================"
        );

        for command in &self.commands {

            // Execute this command and check for errors
            match CommandBlock::run_shell_command(&command){
                Err(err) => return Err(err),
                Ok(()) => (),
            }
        }

        // All commands executed well
        return Ok(());
    }

    /// Runs a given shell command in interactive mode
    /// It should never be called directly, thus is private
    /// Returns the result of executing the command
    fn run_shell_command(command: &str) -> Result<(), CommandError>{
        let user_env_vars: HashMap<String, String> = env::vars().collect();
        let command_handle = Command::new("bash")
            .env_clear()
            .envs(user_env_vars)
            .stdin(Stdio::inherit())
            .arg("-c")
            .arg(command)
            .arg("-Syyu") // TODO -- why this
            .spawn();

        let mut command_handle = match command_handle{
            Ok(command_handle) => command_handle,
            Err(err) => {
                return Err(CommandError::new_not_started(err));
            }
        };

        // Wait for the command
        match command_handle.wait(){
            // We are not interested in the result of the execution, just that if has finished
            Ok(_) => return Ok(()),

            // An error has ocurred while executing the command
            Err(err) => {
                return Err(CommandError::new_exection_error(err));
            }
        }
    }
}

/// Types of error or launching a Command
enum CommandErrorType{
    /// The command was not able to run at first place
    NotStarted,

    /// The command was able to start, but had some failure during execution
    ExecutionError,
}

/// Represents an error ocurred during command execution
pub struct CommandError{
    error_type: CommandErrorType,
    description: String,
}

impl CommandError{
    /// The command could not start its execution
    pub fn new_not_started(error: std::io::Error) -> Self{
        return Self{
            error_type: CommandErrorType::NotStarted,
            description: error.to_string(),
        };
    }

    /// The command started properly, but had a problem during its execution
    pub fn new_exection_error(error: std::io::Error) -> Self{
        return Self{
            error_type: CommandErrorType::ExecutionError,
            description: error.to_string(),
        };
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error_type{
            CommandErrorType::NotStarted => {
                let first_line = "[Error] Command could not start";
                let second_line = format!("\tDescription: {}", self.description);
                let msg = format!("{}\n{}", first_line, second_line);
                return write!(f, "{}", msg);
            }

            CommandErrorType::ExecutionError => {
                let first_line = "[Error] Command started but failed during execution";
                let second_line = format!("\tDescription: {}", self.description);
                let msg = format!("{}\n{}", first_line, second_line);
                return write!(f, "{}", msg);
            }
        }
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
