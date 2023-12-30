mod parsers;

use lib_commands::{SingleCommand, SingleCommandError};

use crate::Commands::parsers::{YamlCommandsParser, ParseCommandsFile};

/// Represent a group of commands to execute in sequence
/// If one command fails, the rest of the commands won't be executed
// TODO -- Add the option to keep executing commands even though one fails
// TODO -- DESIGN -- Really bad desing, as shown in `TomlCommandsParser`
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
    let command_blocks = YamlCommandsParser::parse_file(yaml_file).unwrap();
    for command in command_blocks {
        command.execute();
    }
}

