use fs_extra;
use std::fs;
use std::path::Path;
use std::process::exit;
use crate::DirSync::dir_file_type::DirFileType;
use crate::YamlProcessor;

pub mod directories_descr;
pub mod dir_block;
pub mod dir_file_type;

use crate::DirSync::directories_descr::DirectoriesDescr;
use crate::DirSync::dir_block::DirBlock;

/// Handle the download command
pub fn handle_download(yaml_file: &str) {
    println!("📂 Getting files from git repo to your system!");

    // Get directives from yaml file
    let dir_descr = parse_yaml_directories(yaml_file);

    // Download
    dir_descr.download_from_repo_to_system();
}

/// Handle the upload command
pub fn handle_upload(yaml_file: &str) {
    println!("📂 Uploading files from your system to the repo");

    // Get directives from yaml file
    let dir_descr = parse_yaml_directories(yaml_file);

    // Upload
    dir_descr.upload_from_system_to_repo();
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
    let mut dir_descr = DirectoriesDescr::new(
        parsed_contents["repo_base"]
            .as_str()
            .expect("repo_base: <path> is not specified well")
            .to_string(),
        vec![],
    );

    // Yaml section of files
    let dir_blocks = parsed_contents["directories"].as_vec().unwrap();
    for dir_block in dir_blocks {
        // We ignore the name of the block
        for (_, value) in dir_block.as_hash().unwrap() {
            // Default or error type is File
            let sync_type = value["sync_type"].as_str().unwrap_or("file");
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

/// Creates recursively a dir if does not exist
fn create_dir_if_not_exists(path: &str) {
    if Path::exists(Path::new(path)) == false {
        println!("==> Dir {} does not exist, creating it...", path);
        match fs::create_dir_all(path) {
            Err(err) => {
                eprintln!("Could not create dir {}", path);
                eprintln!("Error code was {}", err);
                exit(-1);
            }
            Ok(_) => (),
        }
    }
}

/// Copies one dir to other recursively
fn copy_dir_recursively(from: &str, to: &str) {
    let from = vec![from];

    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;
    let copy_options = copy_options;

    match fs_extra::copy_items(&from, to, &copy_options) {
        Err(err) => {
            eprintln!("Error copying dir {} to dir {}", from[0], to);
            eprintln!("Error code was {}", err);
            exit(-1);
        }
        Ok(_) => (),
    };
}
