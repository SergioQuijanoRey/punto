use crate::DirSync::file_operations::create_dir_if_not_exists;
use crate::DirSync::{dir_file_type::DirFileType, exit};
use crate::DirSync::DirBlock;

use super::file_operations::copy_dir_recursively;

/// Represent the directories yaml description
#[derive(Debug)]
pub struct DirectoriesDescr {
    repo_base: String,
    dir_blocks: Vec<DirBlock>,
}

impl DirectoriesDescr {

    /// Generates a new struct
    pub fn new(repo_base: String, dir_blocks: Vec<DirBlock>) -> Self {
        return Self{repo_base, dir_blocks};
    }

    /// Appends a new DirBlock to the struct
    pub fn push(&mut self, dir_block: DirBlock) {
        self.dir_blocks.push(dir_block);
    }

    /// Downloads files from repo to the system
    pub fn download_from_repo_to_system(&self) {
        for dir_block in &self.dir_blocks {
            // In order to manage trailing / in paths
            // TODO -- TEST -- Test if presence or absence of trailing / generates problems
            let path = std::path::Path::new(&self.repo_base).join(&dir_block.repo_path());
            let from = path.to_str().unwrap();

            let to = &dir_block.system_path();
            println!("==> Downloading {} to {}", from, to);

            match &dir_block.sync_type() {
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
            let to = std::path::Path::new(&self.repo_base).join(&dir_block.repo_path());
            let to = to.to_str().unwrap();

            let from = &dir_block.system_path();
            println!("==> Uploading {} to {}", from, to);

            match &dir_block.sync_type() {
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
