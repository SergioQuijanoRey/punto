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
pub fn parse_yaml_command(file_path: &str) -> Vec<CommandProcessor::CommandBlock> {
    let parsed_contents = parse_yaml(file_path);
    let mut commands = vec![];

    // Getting the commands from the yaml file into struct
    for (_, value) in parsed_contents.as_hash().unwrap().iter(){

        // TODO -- panics here
        let vector_of_commands = value["commands"].as_vec().unwrap();
        let vector_of_commands: Vec<String> = vector_of_commands.into_iter().map(|command| command.as_str().unwrap().to_string()).collect();

        commands.push(CommandProcessor::CommandBlock{
            description: value["description"].as_str().unwrap_or("No description provided").to_string(),
            quiet: value["quiet"].as_bool().unwrap_or(false),
            commands: vector_of_commands,
            sudo: value["sudo"].as_bool().unwrap_or(false),
        });
    }

    println!("{:?}", commands);
    println!("Exiting the function");
    return commands;
}
