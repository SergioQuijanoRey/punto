use crate::YamlProcessor;
use std::process::exit;
use std::fs;

pub fn handle_download() {
    println!("==> Getting files from git repo to your system!");

    // Get directives from yaml file
    let dir_descr = parse_yaml_directories("/home/sergio/GitProjects/punto/directories.yaml");

    // Download
    dir_descr.dowload_from_repo_to_system();
}


#[derive(Debug)]
pub enum DirFileType {
    File,
    Dir,
}

/// Represent the directories yaml description
#[derive(Debug)]
pub struct DirectoriesDescr {
    repo_base: String,
    dir_blocs: Vec<DirBlock>,
}

impl DirectoriesDescr {
    pub fn push(&mut self, dir_block: DirBlock) {
        self.dir_blocs.push(dir_block);
    }

    /// Downloads files from repo to the system
    pub fn dowload_from_repo_to_system(&self){
        for dir_block in &self.dir_blocs{
            println!("Downloading {} to {} path", dir_block.repo_path, dir_block.system_path);
            match &dir_block.sync_type{
                DirFileType::File => {
                    let from = &dir_block.repo_path;
                    let to = &dir_block.system_path;
                    match std::fs::copy(from, to){
                        Err(err) => {
                            eprintln!("Error copying file {} to file {}", from, to);
                            eprintln!("Error code was {}", err);
                        }
                        Ok(_) => ()
                    };
                },

                DirFileType::Dir => {
                    println!("Syncing directorie")
                }
            }
        }
    }
}

/// Represent a dir block inside yaml description
#[derive(Debug)]
pub struct DirBlock {
    repo_path: String,
    system_path: String,
    sync_type: DirFileType,
}

impl DirBlock {
    pub fn new(repo_path: String, system_path: String, sync_type: DirFileType) -> Self {
        return DirBlock {
            repo_path,
            system_path,
            sync_type,
        };
    }
}

/// From yaml description of the directories sync, gets the DirectoriesDescr struct
pub fn parse_yaml_directories(file_path: &str) -> DirectoriesDescr {
    let parsed_contents = YamlProcessor::parse_yaml(file_path);
    // TODO -- this block of code is repeated
    let parsed_contents = match parsed_contents {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Could not parse {}, exiting now", file_path);
            eprintln!("Error code was {}", err);
            exit(-1);
        }
    };

    // We get the repo_base section from the yaml file
    let mut dir_descr = DirectoriesDescr {
        repo_base: parsed_contents["repo_base"].as_str().expect("repo_base: <path> is not specified well").to_string(),
        dir_blocs: vec![],
    };

    // Yaml section of files
    let dir_blocks = parsed_contents["directories"].as_vec().unwrap();
    for dir_block in dir_blocks {
        // We ignore the name of the block
        for (_, value) in dir_block.as_hash().unwrap() {

            // Default or error type is File
            let sync_type = value["sync_type"].as_str().unwrap();
            let sync_type = if sync_type == "dir" {
                DirFileType::Dir
            } else {
                DirFileType::File
            };

            let repo_path = value["repo_path"].as_str().unwrap();
            let system_path = value["system_path"].as_str().unwrap();

            dir_descr.push(DirBlock::new(
                repo_path.to_string(),
                system_path.to_string(),
                sync_type,
            ));
        }
    }

    return dir_descr;
}
