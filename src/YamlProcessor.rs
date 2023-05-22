use std::fs;
use std::error::Error;
use yaml_rust::YamlLoader;

/// Gets a yaml file path and returns its parsed object
/// It is the base for other specific parser
pub fn parse_yaml(file_path: &str) -> Result<yaml_rust::Yaml, Box<dyn Error>> {
    // Opening yaml file and parsing it
    let contents = fs::read_to_string(file_path)?;
    let parsed_contents = YamlLoader::load_from_str(&contents)?;
    let parsed_contents = parsed_contents[0].clone();
    return Ok(parsed_contents);
}

