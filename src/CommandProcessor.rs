// TODO -- split in different modules, file too large

use crate::YamlProcessor;
use std::collections::HashMap;
use std::env;
use std::process::{Command, Stdio, exit};
use std::fmt;

/// Represent a group of commands to execute in sequence
/// If one command fails, the rest of the commands won't be executed
// TODO -- Add the option to keep executing commands even though one fails
#[derive(Debug)]
pub struct CommandBlock {

    /// The sequence of commands
    commands: Vec<SingleCommand>,

    /// To describe what is the purpose of this block of commands
    description: String,

}

impl CommandBlock {
    /// Creates a new CommandBlock
    pub fn new(commands: Vec<SingleCommand>, description: String) -> Self {
        return CommandBlock {
            commands,
            description,
        };
    }

    /// Executes all commands of the command block
    pub fn execute(&self) -> Result<(), String>{
        println!("Launching command block {}", self.description);
        println!(
            "================================================================================"
        );

        for command in &self.commands {
            // Run the command. Return the error if we find one
            let result = command.run()?;
        }

        // All commands executed well
        return Ok(());
    }

    // /// Runs a given shell command in interactive mode
    // /// It should never be called directly, thus is private
    // /// Returns the result of executing the command
    // // TODO -- remove deprecated function
    // fn run_shell_command(command: &str) -> Result<(), CommandError>{
    //     let user_env_vars: HashMap<String, String> = env::vars().collect();

    //     // Launch the command
    //     // TODO -- BUG -- because we're passing bash as command, never fails
    //     // TODO -- BUG -- pass directly the command
    //     // TODO -- try with this stackoverflow post: https://stackoverflow.com/questions/21011330/how-do-i-invoke-a-system-command-and-capture-its-output
    //     let status = Command::new(command)
    //         .env_clear()
    //         .envs(user_env_vars)
    //         .stdin(Stdio::inherit())
    //         .status();

    //     // Check for status of the command
    //     match status{
    //         Ok(exit_status) => {

    //             // Check if status code was ok
    //             if exit_status.success() == true {
    //                 return Ok(());
    //             }

    //             // Status code is not ok, return error
    //             match exit_status.code() {

    //                 // Execution ended not because user terminated it
    //                 Some(value) => return Err(CommandError::new_execution_error(value)),

    //                 // Exectution ended because user terminated it
    //                 None => {
    //                     return Err(CommandError::new_user_termination());
    //                 }
    //             };
    //         },

    //         // Some weird error happened
    //         // TODO -- not sure at all about this
    //         Err(err) => {
    //             return Err(CommandError::new_not_started(err));
    //         },
    //     };
    // }
}

/// Represents a shell command to execute
#[derive(Debug)]
pub struct SingleCommand{

    /// Wether or not output the result of the command
    quiet: bool,

    /// The comand itself
    command: String,

    /// If we need to use sudo for executing this command
    sudo: bool,
}

impl SingleCommand{
    pub fn new(command: String, quiet: bool, sudo: bool) -> Result<Self, String>{

        // Remove useless whitespaces, so we can check if sudo is inside the command
        let command = command.trim().to_string();

        // Check if command string has sudo
        if command.starts_with("sudo"){
            return Err("Given command string has sudo, that has to be specified with a boolean".to_string());
        }

        return Ok(Self{command, quiet, sudo});
    }

    /// Runs the command
    pub fn run(&self) -> Result<(), String>{

        // Get the builder of the command
        let mut builder = self.get_builder_command();

        // Spawn the command and get the handler
        let handler = match builder.spawn(){
            Err(err) => return Err("Command failed to execute".to_string()),
            Ok(child) => child,
        };

        return Ok(());
    }

    /// Creates the `Command` struct, that we can use for spawning, getting the output, ...
    fn get_builder_command(&self) -> Command {

        // Get all the parts of the command
        let mut string_parts: Vec<&str> = self.command.split(" ").collect();

        // Construct the builder `Command`
        let mut builder = Command::new(string_parts[0]);
        for arg in string_parts.iter().skip(0){
            builder.arg(arg);
        }

        return builder;
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
    // TODO -- we're returning an empty vector
    return vec![];

    // let parsed_contents = YamlProcessor::parse_yaml(file_path);
    // // TODO -- this block of code is repeated
    // let parsed_contents = match parsed_contents{
    //     Ok(contents) => contents,
    //     Err(err) => {
    //         eprintln!("Could not parse {}, exiting now", file_path);
    //         eprintln!("Error code was {}", err);
    //         exit(-1);
    //     }
    // };

    // let mut commands = vec![];

    // // Getting the commands from the yaml file into struct
    // for (_, value) in parsed_contents.as_hash().unwrap().iter() {
    //     // TODO -- panics here
    //     let vector_of_commands = value["commands"].as_vec().unwrap();
    //     let vector_of_commands: Vec<String> = vector_of_commands
    //         .into_iter()
    //         .map(|command| command.as_str().unwrap().to_string())
    //         .collect();

    //     let quiet = value["quiet"].as_bool().unwrap_or(false);
    //     let sudo = value["sudo"].as_bool().unwrap_or(false);
    //     let vector_of_commands: Vec<SingleCommand> = vector_of_commands.iter().map(|command| SingleCommand::new(command, quiet, sudo));

    //     commands.append(&mut vector_of_commands);
    // }

    // // Now convert vector of `SingleCommand` to CommandBlock
    // let command_block = CommandBlock::new(commands, "TODO -- get the description".to_string());

    // println!("{:?}", command_block);
    // println!("Exiting the function");
    // return command_block;
}

/// Tests associated with the run of a single command
#[cfg(test)]
mod single_command_test{
    use super::SingleCommand;

    #[test]
    pub fn test_failing_command() -> Result<(), String>{
        // Build and run a failing command
        let command = SingleCommand::new(
            "This command does not exist".to_string(), false, false
        )?;
        let result = command.run();

        // Check that the command failed to run
        match result{
            Ok(value) => return Err(format!("Expected error, obtained {:?}", value)),
            Err(_) => return Ok(()),
        }
    }

    #[test]
    pub fn test_basic_command_runs() -> Result<(), String>{
        // Build and run a failing command
        let command = SingleCommand::new(
            "ls -la".to_string(), false, false
        )?;

        let result = command.run();

        // Check that the command didn't have problems running
        match result{
            Ok(_) => return Ok(()),
            Err(err) => return Err(format!("Command failed to run. Error code was: {:?}", err))
        }
    }

    #[test]
    pub fn test_install_unexisting_package_fails() -> Result<(), String>{
        // Build and run a failing command
        let command = SingleCommand::new(
            "pacman -S thispackagedoesnotexist".to_string(), false, true
        )?;

        let result = command.run();

        // Check that the install command failed to execute
        match result{
            Ok(_) => return Err(format!("Installation of thispackagedoesnotexist run succesfully")),
            Err(_) => return Ok(()),
        }
    }
}

// Tests related to failing commands management
// #[cfg(test)]
// mod command_failures_test {
    // use super::CommandBlock;

    // #[test]
    // pub fn command_execution_fails(){
    //     let failing_command = CommandBlock::new(
    //         "Testing with a command that does not exist".to_string(),
    //         false,
    //         vec![
    //             "bash".to_string(),
    //             "-c".to_string(),
    //             "commandthatdoesnotexist".to_string(),
    //             "-irrelevantparameter".to_string()
    //         ],
    //         false
    //     );

    //     let execution_result = failing_command.execute().expect_err("This command should generate an error");
    //     println!("Error is {}", execution_result);
    //     let execution_result = execution_result.error_type;
    //     let execution_expected = CommandErrorType::ExecutionError;
    //     assert_eq!(execution_result, execution_expected, "Expected: {:?}, got: {:?}", execution_expected, execution_result);


    // }

// }
