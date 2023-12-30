use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;

use crate::YamlProcessor;
use crate::Commands::CommandBlock;

use lib_commands::SingleCommand;
use serde::Deserialize;
use thiserror::Error;

/// All parsers must take a file path and return a vector of `CommandBlock`
pub trait ParseCommandsFile{
    fn parse_file(path: &str) -> Result<Vec<CommandBlock>, ParsingError>;
}

/// Errors that can happen while parsing a file into a vector of `CommandBlock`
#[derive(Error, Debug)]
pub enum ParsingError {

    #[error("Could not read the file, reason was {reason}")]
    CouldNotReadFile{
        reason: String
    },


    #[error("Could not parse contents of the file, reason was {reason}")]
    ParsingContent{
        reason: String,
    },

    #[error("Field {field_name} is mandatory, and not found in this block")]
    MissingField{
        field_name: String,
    },

    #[error("Could not convert field '{field_name}' to type '{type_should_be}'")]
    BadFieldType{
        field_name: String,
        type_should_be: String,
    },

    #[error("Command '{command_string}' is not valid, reason is '{reason}'")]
    BadCommand{
        command_string: String,
        reason: String
    },

    #[error("Could not convert intermediate representation to the final representation. Reason was '{reason:?}'")]
    IntermediateReprToFinalRepr{
        reason: IntermediateReprToFinalReprError
    }

}

/// Implementation of parsing for yaml files
pub struct YamlCommandsParser;
impl ParseCommandsFile for YamlCommandsParser{
    fn parse_file(path: &str) -> Result<Vec<CommandBlock>, ParsingError> {

        let mut command_blocks = vec![];

        // Getting the commands from the yaml file into struct
        for (_, value) in YamlProcessor::parse_yaml(path)
            .map_err(|err| ParsingError::ParsingContent{reason: err.to_string()})?
            .as_hash().
            ok_or(ParsingError::ParsingContent { reason: "Could not convert contents to a hash map".to_string() })?.iter() {

            println!("Value hash is {:?}", value["description"]);

            // Get the description of the block
            let description = value["description"].as_str().unwrap_or("No description provided");

            // Get if the commands need sudo
            let sudo = value["sudo"].as_bool().unwrap_or(false);

            // Get if the commands need to be run quietly
            let quiet = value["quiet"].as_bool().unwrap_or(false);


            // Get all the commands of this block
            let vector_of_commands = value["commands"].as_vec()
                .ok_or(ParsingError::MissingField{field_name: "commands".to_string()})?;

            let vector_of_commands: Result<Vec<&str>, ParsingError> = vector_of_commands
                .into_iter()
                .map(|command| command.as_str().ok_or(ParsingError::BadFieldType {
                    field_name: "command".to_string(), type_should_be: "String".to_string()
                }))
                .collect();

            let vector_of_commands: Vec<String> = vector_of_commands?
                .into_iter()
                .map(|string| string.to_string())
                .collect();

            // Create a vector of single commands
            let commands: Result<Vec<SingleCommand>, ParsingError> = vector_of_commands
                .into_iter()
                .map(|command_string|
                    SingleCommand::new(command_string.clone(), quiet, sudo)
                        .map_err(|err| ParsingError::BadCommand{command_string, reason: err.to_string()})
                )
                .collect();
            let commands = commands?;

            // Now create the current command block
            let current_command_block = CommandBlock::new(commands, description.to_string());

            // And add that command block to our vector of command blocks
            command_blocks.push(current_command_block);
        }

        return Ok(command_blocks);
    }
}




/// Intermediate representation of a `DirectoriesDescr`, used when parsing
/// from a Toml file
#[derive(Deserialize, Debug)]
struct CommandsDescrTomlRepresentation {
    #[serde(flatten)]
    entries: HashMap<String, Entry>
}

#[derive(Deserialize, Debug)]
struct Entry {
    description: String,
    quiet: Option<bool>,
    sudo: Option<bool>,
    commands: Vec<String>,
}

#[derive(Error, Debug)]
pub enum IntermediateReprToFinalReprError{

    #[error("Error while constructing a single command\nCommand was: '{command_str}'\nError was '{err}'")]
    ConstructingSingleCommand {
        command_str: String,
        err: lib_commands::SingleCommandError,
    }

}

/// Given a CommandsDescrTomlRepresentation, we want to get the final representation
/// as a vector of `CommandBlock`
impl TryFrom<CommandsDescrTomlRepresentation> for Vec<CommandBlock> {
    type Error = IntermediateReprToFinalReprError;

    fn try_from(value: CommandsDescrTomlRepresentation) -> Result<Self, Self::Error> {
        let mut blocks = vec![];

        for (name, entry) in value.entries {

            let mut curr_commands = vec![];
            for command in entry.commands {
                let curr_command = SingleCommand::new(
                    command.clone(),
                    entry.quiet.unwrap_or(false),
                    entry.sudo.unwrap_or(false),
                )
                .map_err(|err| IntermediateReprToFinalReprError::ConstructingSingleCommand {
                    command_str: command.to_string(),
                    err
                })?;

                curr_commands.push(curr_command);

            }

            let curr_block = CommandBlock::new(
                curr_commands,
                entry.description,
            );

            blocks.push(curr_block);
        }

        return Ok(blocks);
    }
}

pub struct TomlCommandsParser;
impl ParseCommandsFile for TomlCommandsParser {
    fn parse_file(path: &str) -> Result<Vec<CommandBlock>, ParsingError> {

        // Read the raw data from the given file
        let data = fs::read_to_string(path)
            .map_err(|e| ParsingError::CouldNotReadFile { reason: format!("{:?}", e)})?;

        // Parse that data to a intermediate struct representation
        let intermediate_representation: CommandsDescrTomlRepresentation = toml::from_str(&data)
            .map_err(|e| ParsingError::ParsingContent { reason: e.to_string() })?;

        // Convert the intermediate representation to `DirectoriesDescr` struct
        let dir_descr = Vec::<CommandBlock>::try_from(intermediate_representation)
            .map_err(|e| ParsingError::IntermediateReprToFinalRepr { reason: e })?;

        return Ok(dir_descr);
    }
}
