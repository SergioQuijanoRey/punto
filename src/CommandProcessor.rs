// TODO -- split in different modules, file too large

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

        // Launch the command
        // TODO -- BUG -- because we're passing bash as command, never fails
        // TODO -- BUG -- pass directly the command
        let status = Command::new("bash")
            .env_clear()
            .envs(user_env_vars)
            .stdin(Stdio::inherit())
            .arg("-c")
            .arg("command")
            .status();

        // Check for status of the command
        match status{
            Ok(exit_status) => {

                // Check if status code was ok
                if exit_status.success() == true {
                    return Ok(());
                }

                // Status code is not ok, return error
                match exit_status.code() {

                    // Execution ended not because user terminated it
                    Some(value) => return Err(CommandError::new_execution_error(value)),

                    // Exectution ended because user terminated it
                    None => {
                        return Err(CommandError::new_user_termination());
                    }
                };
            },

            // Some weird error happened
            // TODO -- not sure at all about this
            Err(err) => {
                return Err(CommandError::new_not_started(err));
            },
        };
    }
}

/// Types of error or launching a Command
#[derive(Debug, PartialEq)]
enum CommandErrorType{
    /// The command failed in a really weird way (not abling to spawn, not having perms...)
    ExtremeFailure,

    /// The command was able to start, but had some failure during execution show in the exit code
    ExecutionError,

    /// The command failed because the user terminated it
    UserTermination,
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
            error_type: CommandErrorType::ExtremeFailure,
            description: error.to_string(),
        };
    }

    /// The command started properly, but had a problem during its execution
    /// This is shown at the exit code
    pub fn new_execution_error(exit_code: i32) -> Self{
        return Self{
            error_type: CommandErrorType::ExecutionError,
            description: format!("Ended badly with exit code {}", exit_code),
        };
    }

    /// The command failed because user terminated it
    pub fn new_user_termination() -> Self{
        return Self{
            description: "User terminated the execution".to_string(),
            error_type: CommandErrorType::UserTermination,
        };
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error_type{
            CommandErrorType::ExtremeFailure => {
                let first_line = "[Error] Command failed in an extreme way";
                let second_line = format!("\tDescription: {}", self.description);
                let msg = format!("{}\n{}", first_line, second_line);
                return write!(f, "{}", msg);
            }

            CommandErrorType::ExecutionError => {
                let first_line = format!("[Error] {}", self.description);
                let second_line = format!("\tDescription: {}", self.description);
                let msg = format!("{}\n{}", first_line, second_line);
                return write!(f, "{}", msg);
            }

            CommandErrorType::UserTermination => {
                return write!(f, "User ended execution");

            }
        }
    }
}



/// Handler to --shell cli argument
/// Reads the shell yaml config file and executes commands described in the yaml file
// TODO -- handle exceptions
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

/// Tests related to failing commands management
#[cfg(test)]
mod command_failures_test {
    use super::CommandBlock;
    use super::CommandError;
    use super::CommandErrorType;

    #[test]
    pub fn command_execution_fails(){
        let failing_command = CommandBlock::new(
            "Command that should fail".to_string(),
            false,
            vec!["this commnand does not exist".to_string()],
            false
        );

        let execution_result = failing_command.execute().expect_err("This command should generate an error").error_type;
        let execution_expected = CommandErrorType::ExecutionError;
        assert_eq!(execution_result, execution_expected, "Expected: {:?}, got: {:?}", execution_expected, execution_result);


    }

}
