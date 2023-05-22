use std::process::exit;

use crate::YamlProcessor;
use crate::Commands::CommandBlock;

use lib_commands::SingleCommand;

/// TODO -- DESIGN -- parsing files seems should go to different modules
/// Given a yaml file path, it returns the CommandOptions vector which are used to launch a command
pub fn parse_yaml_command(file_path: &str) -> Vec<CommandBlock> {

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
