use std::fs;
use yaml_rust::YamlLoader;

/// Gets a yaml file path and returns its parsed object
/// It is the base for other specific parser
pub fn parse_yaml(file_path: &str) -> yaml_rust::Yaml {
    // Opening yaml file and parsing it
    let contents = fs::read_to_string(file_path).unwrap();
    let parsed_contents = YamlLoader::load_from_str(&contents).unwrap();
    let parsed_contents = parsed_contents[0].clone();
    return parsed_contents;
}

