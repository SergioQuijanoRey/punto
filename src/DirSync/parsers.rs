use std::collections::HashMap;
use std::convert::{TryInto, TryFrom};
use std::fs;

/// Module where we parse yaml files to Rust structs that our program can use
/// Also, more than one parser can be implemented here
/// For example, parser for yaml files, for toml files, ...

// TODO -- This module is very messy

use yaml_rust::Yaml;
use serde::Deserialize;
use thiserror::Error;

use crate::DirSync::dir_block::{DirBlock, DirFileType};
use crate::YamlProcessor;
use crate::DirSync::directories_descr::DirectoriesDescr;


// TODO -- DESING -- we are holding some errors as strings, which is kinda weird
#[derive(Debug, Error)]
pub enum ParsingError {

    #[error("Could not read the contents of the file")]
    CouldNotReadContentsOfFile{
        reason: String,
    },

    #[error("Could not parse file {file} to a rust object, reason was:\n{reason}")]
    CouldNotParseFile{
        file: String,
        reason: String,
    },

    #[error("Could not get section {section_name} at block {dir_block_name:?} from the parsed file\nCheck that the contents of the block are properly indented")]
    SectionNotFound{
        section_name: String,
        dir_block_name: Option<String>,
    },

    #[error("Could not convert intermediate representation to DirectoriesDescr, reason was:\n{reason}")]
    IntermediateReprToFinalRepr{
        reason: String
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
                return Err(ParsingError::CouldNotParseFile{
                    file: path.to_string(),
                    reason: format!("{}", err),
                });
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

/// Intermediate representation of a `DirectoriesDescr`, used when parsing
/// from a Toml file
#[derive(Deserialize, Debug)]
struct DirectoriesDescrTomlRepresentation {
    repo_base: String,
    system_base: String,

    #[serde(flatten)]
    entries: HashMap<String, Entry>
}

#[derive(Deserialize, Debug)]
struct Entry {
    repo_path: String,
    system_path: String,
    sync_type: Option<String>,
}

/// Errors that can happen when parsing a intermediate representation for TOML
/// into a `DirectoriesDescrTomlRepresentation`
#[derive(Error, Debug)]
pub enum TomlToDirDescrError {
    #[error("Sync type is neither 'file' or 'dir', it is {0}")]
    BadSyncType(String),

}

/// Implement the conversion from the intermediate representation to the final
/// representation that we want
impl TryFrom<DirectoriesDescrTomlRepresentation> for DirectoriesDescr {
    type Error = TomlToDirDescrError;

    fn try_from(repr: DirectoriesDescrTomlRepresentation) -> Result<Self, Self::Error> {
        let mut dir_blocks = vec![];

        // TODO -- might be a good idea to implement `Into<DirBlock> for Entry`
        // because that's what we are doing here
        for (key, entry) in repr.entries{

            // Get the sync type for this entry
            let sync_type = entry.sync_type.unwrap_or("file".to_string());
            let sync_type = match sync_type.as_str() {
                "file" => DirFileType::File,
                "dir" => DirFileType::Dir,
                other => return Err(TomlToDirDescrError::BadSyncType(other.to_string())),
            };


            let curr_block = DirBlock::new(
                entry.repo_path,
                entry.system_path,
                sync_type,

                // TODO -- BUG -- Need to be implemented
                // TODO -- BUG -- We are not ignoring files!
                vec![],
            );

            dir_blocks.push(curr_block);
        }

        return Ok(DirectoriesDescr::new(
            repr.repo_base,
            repr.system_base,
            dir_blocks
        ));
    }
}

/// Parser for yaml files
pub struct TomlDirParser;
impl ParseDirectories for TomlDirParser {
    fn parse_file(path: &str) -> Result<DirectoriesDescr, ParsingError> {

        // Read the raw data from the given file
        let data = fs::read_to_string(path)
            .map_err(|e| ParsingError::CouldNotReadContentsOfFile{reason: format!("{}", e)})?;

        // Parse that data to a intermediate struct representation
        let intermediate_representation: DirectoriesDescrTomlRepresentation = toml::from_str(&data)
            .map_err(|e| ParsingError::CouldNotParseFile { file: path.to_string(), reason: format!("{}", e) })?;

        // Convert the intermediate representation to `DirectoriesDescr` struct
        let dir_descr = DirectoriesDescr::try_from(intermediate_representation)
            .map_err(|e| ParsingError::IntermediateReprToFinalRepr { reason: format!("{}", e) })?;

        return Ok(dir_descr);
    }
}
