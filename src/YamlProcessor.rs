use std::fs;
use yaml_rust::YamlLoader;
use crate::CommandProcessor;

/// Gets a yaml file path and returns its parsed object
/// It is the base for other specific parser
fn parse_yaml(file_path: &str) -> yaml_rust::Yaml {
    // Opening yaml file and parsing it
    let contents = fs::read_to_string(file_path).unwrap();
    let parsed_contents = YamlLoader::load_from_str(&contents).unwrap();
    let parsed_contents = parsed_contents[0].clone();
    return parsed_contents;
}

/// Given a yaml file path, it returns the CommandOptions vector which are used to launch a command
pub fn parse_yaml_command(file_path: &str) -> Vec<CommandProcessor::CommandOptions> {
    let parsed_contents = parse_yaml(file_path);

    // Getting the commands from the yaml file into struct
    let mut commands = vec![];
    for (_, value) in parsed_contents.as_hash().unwrap().iter(){
        commands.push(CommandProcessor::CommandOptions{
            description: value["description"].as_str().unwrap_or("No description provided").to_string(),
            quiet: value["quiet"].as_bool().unwrap_or(false),
            command: value["command"].as_str().unwrap().to_string(), // Only mandatory field for command
            sudo: value["sudo"].as_bool().unwrap_or(false),
        });
    }

    return commands;
}
