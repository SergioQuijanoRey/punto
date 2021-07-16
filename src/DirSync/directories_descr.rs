use crate::DirSync::file_operations::{create_dir_if_not_exists, sync_dir, sync_file};
use crate::DirSync::{dir_file_type::DirFileType, exit};
use crate::DirSync::DirBlock;

/// Represent the directories yaml description
/// This representation is based on a set of dirblocks
#[derive(Debug)]
pub struct DirectoriesDescr {
    /// All the dir blocks have the same repo_base path
    repo_base: String,

    /// The dir blocks that make up the DirectoriesDescr
    dir_blocks: Vec<DirBlock>,
}

impl DirectoriesDescr {

    /// Generates a new struct
    pub fn new(repo_base: String, dir_blocks: Vec<DirBlock>) -> Self {
        // Create and check the new struct
        let new_struct = Self{repo_base, dir_blocks};
        if new_struct.is_valid() == false{
            panic!("New DirectoriesDescr struct is not valid");
        }

        return new_struct;
    }

    /// Checks if a DirectoriesDescr is valid
    /// DirectoriesDescr is not valid when repo_base does not match all dir_blocks repo_base
    fn is_valid(&self) -> bool{
        for dir_block in &self.dir_blocks{
            if dir_block.repo_path() != &self.repo_base{
                return false;
            }
        }

        return true;
    }

    /// Appends a new DirBlock to the struct
    pub fn push(&mut self, dir_block: DirBlock) {
        self.dir_blocks.push(dir_block);
    }

    /// Downloads files from repo to the system
    /// Download in sync mode: can delete files in system that are not present in repo
    pub fn download_from_repo_to_system(&self) {
        for dir_block in &self.dir_blocks {
            // In order to manage trailing / in paths
            // TODO -- TEST -- Test if presence or absence of trailing / generates problems
            let from = std::path::Path::new(&self.repo_base).join(&dir_block.repo_path());
            let from = from.to_str().unwrap();

            let to = &dir_block.system_path();
            println!("==> Downloading {} to {}", from, to);

            let ignore_files = &dir_block.ignore_files();

            match &dir_block.sync_type() {
                DirFileType::File => sync_file(from, to),
                DirFileType::Dir => sync_dir(from, to, ignore_files),
            }
        }
    }

    /// Uploads files from system to the repo
    /// Upload in sync mode: can delete files in repo that are not present in system
    pub fn upload_from_system_to_repo(&self) {
        for dir_block in &self.dir_blocks {

            // In order to manage trailing / in paths
            let to = &super::file_operations::join_two_paths(&self.repo_base, &dir_block.repo_path());

            let from = &dir_block.system_path();
            println!("==> Uploading {} to {}", from, to);

            let ignore_files = &dir_block.ignore_files();

            match &dir_block.sync_type() {
                DirFileType::File => sync_file(from, to),
                DirFileType::Dir => sync_dir(from, to, ignore_files),
            }
        }
    }
}
