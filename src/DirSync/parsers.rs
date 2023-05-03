/// Module where we parse yaml files to Rust structs that our program can use
/// Also, more than one parser can be implemented here
/// For example, parser for yaml files, for toml files, ...

use yaml_rust::Yaml;

use crate::DirSync::dir_file_type::DirFileType;
use crate::DirSync::dir_block::DirBlock;
use crate::YamlProcessor;
use crate::DirSync::directories_descr::DirectoriesDescr;


use thiserror::Error;
#[derive(Debug, Error)]
pub enum ParsingError {

    #[error("Could not parse file.yaml to a rust object. Parsing error code was {0}")]
    CouldNotParseFile(String),

    #[error("Could not get section {section_name} at block {dir_block_name:?} from the parsed file\nCheck that the contents of the block are properly indented")]
    SectionNotFound{
        section_name: String,
        dir_block_name: Option<String>,
    },
}

/// All parsers must take a file path and return a `DirectoriesDescr`
pub trait ParseDirectories {
    fn parse_file(path: &str) -> Result<DirectoriesDescr, ParsingError>;
}

/// Parser for yaml files
pub struct YamlDirParser;
impl ParseDirectories for YamlDirParser {
    fn parse_file(path: &str) -> Result<DirectoriesDescr, ParsingError> {

        // Parse the yaml file to a Yaml rust object
        // TODO -- error handling should be easier
        let parsed_contents = YamlProcessor::parse_yaml(path);
        let parsed_contents = match parsed_contents {
            Ok(contents) => contents,
            Err(err) => {
                return Err(ParsingError::CouldNotParseFile(format!("{}", err)));
            }
        };

        // We get the repo_base section from the yaml file
        let mut dir_descr = DirectoriesDescr::new(
            parsed_contents["repo_base"]
                .as_str()
                .ok_or(ParsingError::SectionNotFound{
                    section_name: "repo_base".to_string(),
                    dir_block_name: None,
                })?
                .to_string(),
            parsed_contents["system_base"]
                .as_str()
                .ok_or(ParsingError::SectionNotFound{
                    section_name: "system_base".to_string(),
                    dir_block_name: None,
                })?
                .to_string(),
            vec![],
        );

        // Yaml section of files
        let dir_blocks = parsed_contents["directories"]
            .as_vec()
            .ok_or(ParsingError::SectionNotFound{
                section_name: "directories (vector)".to_string(),
                dir_block_name: None,
            })?;

        for dir_block in dir_blocks {
            // We ignore the name of the block
            for (block_name, value) in dir_block.as_hash().unwrap() {
                // Default or error type is File
                let sync_type = value["sync_type"].as_str().unwrap_or("file");
                let sync_type = if sync_type == "dir" {
                    DirFileType::Dir
                } else {
                    DirFileType::File
                };

                let repo_path = value["repo_path"]
                    .as_str()
                    .ok_or(ParsingError::SectionNotFound{
                        section_name: "repo_path".to_string(),
                        dir_block_name: Some(
                            block_name
                                .as_str()
                                .unwrap_or_else(|| "Could not get the name of the dir block that caused the failure")
                                .to_string()
                        )
                    })?;

                let system_path = value["system_path"]
                    .as_str()
                    .ok_or(ParsingError::SectionNotFound{
                        section_name: "system_path".to_string(),
                        dir_block_name: Some(
                            block_name
                                .as_str()
                                .unwrap_or_else(|| "Could not get the name of the dir block that caused the failure")
                                .to_string()
                        )
                    })?;

                let empty_vec : Vec<Yaml> = Vec::new();
                let ignore_files = value["ignore_files"].
                    as_vec().
                    unwrap_or(&empty_vec).
                    into_iter().
                    map(|item| item.as_str().unwrap().to_string()).collect();

                dir_descr.push(DirBlock::new(
                    repo_path.to_string(),
                    system_path.to_string(),
                    sync_type,
                    ignore_files,
                ));
            }
        }

        return Ok(dir_descr);
    }
}
