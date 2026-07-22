use std::path::PathBuf;

use crate::dir_sync::dir_block::{DirBlock, DirFileType};
use anyhow::Context;
use lib_fileops::sync_options::SyncOptions;
use lib_fileops::{get_dir_diff, join_two_paths, sync_dir, sync_file};

/// Represent the dir structure that we want to manage
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
        return Self {
            repo_base,
            system_base,
            dir_blocks,
        };
    }

    /// Appends a new DirBlock to the struct
    pub fn push(&mut self, dir_block: DirBlock) {
        self.dir_blocks.push(dir_block);
    }

    /// Downloads files from repo to the system
    /// Download in sync mode: can delete files in system that are not present in repo
    // TODO -- test -- need to add some tests
    pub fn download_from_repo_to_system(&self) {
        for dir_block in &self.dir_blocks {
            // Get two absolute paths using base paths
            let from = &join_two_paths(&self.repo_base, &dir_block.repo_path());
            let to = &join_two_paths(&self.system_base, &dir_block.system_path());
            println!("==> Downloading {} to {}", from, to);

            let sync_options = SyncOptions::builder()
                .exclude_patterns(dir_block.ignore_files())
                .delete_files_destination(dir_block.delete_files_at_destination())
                .build();

            // TODO -- DESIGN -- should this function return an error?
            match &dir_block.sync_type() {
                DirFileType::File => sync_file(from, to)
                    .context(format!("Could not sync file from {} to {}", from, to))
                    .unwrap(),
                DirFileType::Dir => sync_dir(PathBuf::from(from), PathBuf::from(to), sync_options)
                    .context(format!("Could not sync dir from {} to {}", from, to))
                    .unwrap(),
            };
        }
    }

    /// Uploads files from system to the repo
    /// Upload in sync mode: can delete files in repo that are not present in system
    // TODO -- TEST -- need to add some tests
    pub fn upload_from_system_to_repo(&self) {
        for dir_block in &self.dir_blocks {
            // Get two absolute paths using base paths
            let to = &join_two_paths(&self.repo_base, &dir_block.repo_path());
            let from = &join_two_paths(&self.system_base, &dir_block.system_path());
            println!("==> Uploading {} to {}", from, to);

            let sync_options = SyncOptions::builder()
                .exclude_patterns(dir_block.ignore_files())
                .delete_files_destination(dir_block.delete_files_at_destination())
                .build();

            // TODO -- DESIGN -- should this function return an error?
            match &dir_block.sync_type() {
                DirFileType::File => sync_file(from, to)
                    .context(format!("Could not sync file from {} to {}", from, to))
                    .unwrap(),
                DirFileType::Dir => sync_dir(PathBuf::from(from), PathBuf::from(to), sync_options)
                    .context(format!("Could not sync dir from {} to {}", from, to))
                    .unwrap(),
            };
        }
    }

    /// Checks for dir sync problems
    /// That's to say, search for files that are present in repo (or system)
    /// but not in system (or repo)
    /// This happens when we delete a file, because dir sync does not delete files
    pub fn check(&self) {
        // Filter entries that are about files, that entries can't be checked
        let only_dirs: Vec<&DirBlock> = self
            .dir_blocks
            .iter()
            .filter(|block| block.sync_type() == &DirFileType::Dir)
            .collect();

        // Iterate over the dir blocks and check for files present in one place
        // but not in the other
        for curr_dir_block in only_dirs {
            let absolute_repo_path =
                join_two_paths(self.repo_base.as_str(), curr_dir_block.repo_path().as_str());
            let absolute_system_path = join_two_paths(
                self.system_base.as_str(),
                curr_dir_block.system_path().as_str(),
            );

            // Check for files that are present in the repo but not in the system
            // These are the dangerous files
            let new_files = get_dir_diff(&absolute_system_path, &absolute_repo_path)
                .context(format!(
                    "Could not diff {} and {}",
                    absolute_repo_path, absolute_system_path
                ))
                .unwrap();

            // Warn the user if we found some files
            if new_files.len() > 0 {
                println!("🚨 Found files that are present in the repo but not in the system!");
                for file in new_files {
                    println!("\t- {file}");
                }
                println!("");
            }

            // Check for files that are present in the system but not in the repo
            let new_files = get_dir_diff(&absolute_repo_path, &absolute_system_path)
                .context(format!(
                    "Could not diff {} and {}",
                    absolute_system_path, absolute_repo_path
                ))
                .unwrap();

            // Warn the user if we found some files
            if new_files.len() > 0 {
                println!("🚨 Found files that are present in the system but not in the repo!");
                println!("😅 Don't worry too much, probably you want to update these files from system to your git repo");

                for file in new_files {
                    println!("\t- {file}");
                }
                println!("");
            }
        }
    }
}
