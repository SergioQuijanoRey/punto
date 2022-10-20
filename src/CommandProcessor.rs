// TODO -- split in different modules, file too large

use crate::YamlProcessor;
use std::collections::HashMap;
use std::env;
use crate::SingleCommand::SingleCommand;

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
