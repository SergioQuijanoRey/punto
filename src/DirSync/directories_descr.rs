use crate::DirSync::file_operations::{join_two_paths, sync};
use crate::DirSync::dir_file_type::DirFileType;
use crate::DirSync::DirBlock;

/// Represent the directories yaml description
/// This representation is based on a set of dirblocks
#[derive(Debug)]
pub struct DirectoriesDescr {
    /// All the dir blocks have the same repo_base path
    repo_base: String,

    /// All the dir blocks have the same system_base path
    system_base: String,

    /// The dir blocks that make up the DirectoriesDescr
    dir_blocks: Vec<DirBlock>,
}

impl DirectoriesDescr {

    /// Generates a new struct
    pub fn new(repo_base: String, system_base: String, dir_blocks: Vec<DirBlock>) -> Self {
        return Self{repo_base, system_base, dir_blocks};
    }

    /// Appends a new DirBlock to the struct
    pub fn push(&mut self, dir_block: DirBlock) {
        self.dir_blocks.push(dir_block);
    }

    /// Downloads files from repo to the system
    /// Download in sync mode: can delete files in system that are not present in repo
    pub fn download_from_repo_to_system(&self) {
        for dir_block in &self.dir_blocks {

            // Get two absolute paths using base paths
            let from = &join_two_paths(&self.repo_base, &dir_block.repo_path());
            let to = &join_two_paths(&self.system_base, &dir_block.system_path());
            println!("==> Downloading {} to {}", from, to);

            let ignore_files = &dir_block.ignore_files();

            // TODO -- BUG -- this might be badly implemented
            // TODO -- BUG -- copy_dir_recursively result must be handled
            match &dir_block.sync_type() {
                DirFileType::File => sync(from, to, &vec![], false).expect("Failed to sync file"),
                DirFileType::Dir => sync(from, to, ignore_files, false).expect("Failed to sync dir"),
            };
        }
    }

    /// Uploads files from system to the repo
    /// Upload in sync mode: can delete files in repo that are not present in system
    pub fn upload_from_system_to_repo(&self) {
        for dir_block in &self.dir_blocks {

            // Get two absolute paths using base paths
            let to = &join_two_paths(&self.repo_base, &dir_block.repo_path());
            let from = &join_two_paths(&self.system_base, &dir_block.system_path());
            println!("==> Uploading {} to {}", from, to);

            let ignore_files = &dir_block.ignore_files();

            // TODO -- BUG -- this might be badly implemented
            // TODO -- BUG -- copy_dir_recursively result must be handled
            match &dir_block.sync_type() {
                DirFileType::File => sync(from, to, &vec![], false).expect("Failed to sync file"),
                DirFileType::Dir => sync(from, to, ignore_files, false).expect("Failed to sync dir"),
            };
        }
    }
}
