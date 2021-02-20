use crate::YamlProcessor;
use fs_extra;
use std::fs;
use std::path::Path;
use std::process::exit;

pub fn handle_download(yaml_file: &str) {
    println!("==> Getting files from git repo to your system!");

    // Get directives from yaml file
    let dir_descr = parse_yaml_directories(yaml_file);

    // Download
    dir_descr.download_from_repo_to_system();
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
    dir_blocks: Vec<DirBlock>,
}

impl DirectoriesDescr {
    pub fn push(&mut self, dir_block: DirBlock) {
        self.dir_blocks.push(dir_block);
    }

    /// Downloads files from repo to the system
    pub fn download_from_repo_to_system(&self) {
        for dir_block in &self.dir_blocks {
            // In order to manage trailing / in paths
            // TODO -- TEST -- Test if presence or absence of trailing / generates problems
            let path = std::path::Path::new(&self.repo_base).join(&dir_block.repo_path);
            let from = path.to_str().unwrap();

            let to = &dir_block.system_path;
            println!("==> Downloading {} to {}", from, to);

            match &dir_block.sync_type {
                DirFileType::File => DirectoriesDescr::sync_file(from, to),
                DirFileType::Dir => DirectoriesDescr::sync_dir(from, to),
            }
        }
    }

    /// Uploads files from system to the repo
    pub fn upload_from_system_to_repo(&self) {
        for dir_block in &self.dir_blocks {

            // In order to manage trailing / in paths
            // TODO -- TEST -- Test if presence or absence of trailing / generates problems
            let to = std::path::Path::new(&self.repo_base).join(&dir_block.repo_path);
            let to = to.to_str().unwrap();

            let from = &dir_block.system_path;
            println!("==> Uploading {} to {}", from, to);

            match &dir_block.sync_type {
                DirFileType::File => DirectoriesDescr::sync_file(from, to),
                DirFileType::Dir => DirectoriesDescr::sync_dir(from, to),
            }
        }
    }

    fn sync_file(from: &str, to: &str) {

        // Create parent dir if not exists
        let to_parent_dir = DirectoriesDescr::parent_dir(to);
        create_dir_if_not_exists(to_parent_dir);

        match std::fs::copy(from, to) {
            Err(err) => {
                eprintln!("Error copying file {} to file {}", from, to);
                eprintln!("Error code was {}", err);
                exit(-1);
            }
            Ok(_) => (),
        };
    }

    // TODO -- BUG -- not removing deprecated files
    // ie, when wallpaper is removed, sync_dir wont remove that wallpaper on to
    // dir
    fn sync_dir(from: &str, to: &str) {
        let to_parent_dir = DirectoriesDescr::parent_dir(to);
        create_dir_if_not_exists(to_parent_dir);
        copy_dir_recursively(from, to_parent_dir);
    }

    /// Gets the str path of the parent dir
    /// If to is a file path, gets is dir where its allocated
    /// If to is a dir, gets its parent dir
    fn parent_dir(to: &str) -> &str{
        let to_parent_dir = std::path::Path::new(to).parent().expect(&format!("Could not get parent dir of {}", to));
        return to_parent_dir.to_str().expect(&format!("Could not get string from {} parent", to));
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
        repo_base: parsed_contents["repo_base"]
            .as_str()
            .expect("repo_base: <path> is not specified well")
            .to_string(),
        dir_blocks: vec![],
    };

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

pub fn handle_upload(yaml_file: &str) {
    println!("==> Uploading files from your system to the repo");

    // Get directives from yaml file
    let dir_descr = parse_yaml_directories(yaml_file);

    // Upload
    dir_descr.upload_from_system_to_repo();
}
