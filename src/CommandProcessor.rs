// TODO -- split in different modules, file too large

use std::process::exit;
use crate::YamlProcessor;
use lib_commands::{SingleCommand, SingleCommandError};

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
    pub fn execute(&self) -> Result<(), SingleCommandError>{
        println!("Launching command block {}", self.description);
        println!(
            "================================================================================"
        );

        for command in &self.commands {
            // Run the command. Return the error if we find one
            let _result = command.run()?;
        }

        // All commands executed well
        return Ok(());
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

/// TODO -- DESIGN -- parsing files seems should go to different modules
/// Given a yaml file path, it returns the CommandOptions vector which are used to launch a command
fn parse_yaml_command(file_path: &str) -> Vec<CommandBlock> {

    let parsed_contents = YamlProcessor::parse_yaml(file_path);
    let parsed_contents = match parsed_contents{
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Could not parse {}, exiting now", file_path);
            eprintln!("Error code was {}", err);
            exit(-1);
        }
    };

    let mut command_blocks = vec![];

    // Getting the commands from the yaml file into struct
    for (_, value) in parsed_contents.as_hash().unwrap().iter() {

        println!("Value hash is {:?}", value["description"]);

        // Get the description of the block
        let description = value["description"].as_str().unwrap_or("No description provided");

        // Get if the commands need sudo
        let sudo = value["sudo"].as_bool().unwrap_or(false);

        // Get if the commands need to be run quietly
        let quiet = value["quiet"].as_bool().unwrap_or(false);


        // Get all the commands of this block
        let vector_of_commands = value["commands"].as_vec().unwrap();
        let vector_of_commands: Vec<String> = vector_of_commands
            .into_iter()
            .map(|command| command.as_str().unwrap().to_string())
            .collect();

        // Create a vector of single commands
        let commands: Vec<SingleCommand> = vector_of_commands
            .into_iter()
            .map(|command_string| SingleCommand::new(command_string, quiet, sudo).unwrap())
            .collect();

        // Now create the current command block
        let current_command_block = CommandBlock::new(commands, description.to_string());

        // And add that command block to our vector of command blocks
        command_blocks.push(current_command_block);
    }

    return command_blocks;
}
