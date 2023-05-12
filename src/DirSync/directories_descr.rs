use std::path::Path;

use crate::DirSync::dir_block::{DirBlock, DirFileType};
use lib_fileops::{join_two_paths, sync_dir, sync_file, get_dir_diff};
use anyhow::Context;

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
        return Self{repo_base, system_base, dir_blocks};
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

            let ignore_files = &dir_block.ignore_files();

            // TODO -- DESIGN -- should this function return an error?
            match &dir_block.sync_type() {
                DirFileType::File => sync_file(from, to)
                    .context(format!("Could not sync file from {} to {}", from, to))
                    .unwrap(),
                DirFileType::Dir => sync_dir(from, to, ignore_files, false)
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

            let ignore_files = &dir_block.ignore_files();

            // TODO -- DESIGN -- should this function return an error?
            match &dir_block.sync_type() {
                DirFileType::File => sync_file(from, to)
                    .context(format!("Could not sync file from {} to {}", from, to))
                    .unwrap(),
                DirFileType::Dir => sync_dir(from, to, ignore_files, false)
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
        let only_dirs: Vec<&DirBlock> = self.dir_blocks.iter()
            .filter(|block| block.sync_type() == &DirFileType::Dir)
            .collect();

        // Iterate over the dir blocks and check for files present in one place
        // but not in the other
        for curr_dir_block in only_dirs{

            let absolute_repo_path = join_two_paths(
                self.repo_base.as_str(),
                curr_dir_block.repo_path().as_str()
            );
            let absolute_system_path = join_two_paths(
                self.system_base.as_str(),
                curr_dir_block.system_path().as_str()
            );

            // Check for files that are present in the repo but not in the system
            // These are the dangerous files
            let new_files = get_dir_diff(&absolute_system_path, &absolute_repo_path)
                .context(format!("Could not diff {} and {}", absolute_repo_path, absolute_system_path))
                .unwrap();

            // Warn the user if we found some files
            if new_files.len() > 0 {
                println!("🚨 Found files that are present in the repo but not in the system!");
                for file in new_files{
                    println!("\t- {file}");
                }
                println!("");
            }

            // Check for files that are present in the system but not in the repo
            let new_files = get_dir_diff(&absolute_repo_path, &absolute_system_path)
                .context(format!("Could not diff {} and {}", absolute_system_path, absolute_repo_path))
                .unwrap();

            // Warn the user if we found some files
            if new_files.len() > 0 {

                println!("🚨 Found files that are present in the system but not in the repo!");
                println!("😅 Don't worry too much, probably you want to update these files from system to your git repo");

                for file in new_files{
                    println!("\t- {file}");
                }
                println!("");
            }
        }
    }
}

#[cfg(test)]
mod tests{

    use std::{path::Path, fs};

    use super::DirectoriesDescr;
    use crate::DirSync::dir_block::{DirBlock, DirFileType};

    /// A lot of tests need to work in top a file hierarchy structure
    /// So with this function we can create a basic structure
    fn create_basic_file_structure(base_path: &str) -> Option<()>{
        fs::create_dir(Path::new(base_path)).ok()?;
        fs::create_dir(Path::new(base_path).join("src")).ok()?;
        fs::create_dir(Path::new(base_path).join("test")).ok()?;
        fs::File::create(Path::new(base_path).join("src/first.rs")).ok()?;
        fs::File::create(Path::new(base_path).join("src/second.rs")).ok()?;
        fs::File::create(Path::new(base_path).join("src/third.rs")).ok()?;
        fs::File::create(Path::new(base_path).join("test/first_test.rs")).ok()?;
        fs::File::create(Path::new(base_path).join("test/second_test.rs")).ok()?;

        return Some(());
    }

    /// Remove the basic file structure created with `create_basic_file_structure`
    fn remove_basic_file_structure(base_path: &str) -> Option<()>{
        fs::remove_dir_all(base_path).ok()?;

        return Some(());
    }

    /// Also, create a basic DirectoriesDescr to work with
    /// Instead of reading from a `.yaml` test file, we create that structure
    /// manually
    /// NOTE: do not share root folder, because some tests might run in parallel
    fn create_basic_dir_description(base_path: &str) -> DirectoriesDescr{

        let repo_base = base_path;

        // Binding to not mess with the lifetimes
        let binding = Path::new(repo_base)
            .join("system");
        let system_base = binding
            .to_str()
            .expect("Could not convert path object to string");

        // Create a bunch of DirBlocks
        // Put the parameters of each dir block in vectors, so creating more than one dir block
        // is easier
        let repo_paths = vec!["src", "test/first_test.rs"];
        let system_paths = vec!["alternative_src", "other_test_place/first_test___.rs"];
        let sync_types = vec![DirFileType::Dir, DirFileType::File];
        let ignored_files = vec![vec!["first.rs".to_string()], vec![]];

        // Use vector of parameters to construct the DirBlocks
        let mut dir_blocks = vec![];
        for i in 0..repo_paths.len(){

            let repo_path = repo_paths[i].to_string();
            let system_path = system_paths[i].to_string();
            let sync_type = sync_types[i].clone();
            let curr_ignored_files = ignored_files[i].clone();

            // Create the dir block with the current data
            let new_dir_block = DirBlock::new(repo_path, system_path, sync_type, curr_ignored_files);
            dir_blocks.push(new_dir_block);
        }

        return DirectoriesDescr::new(repo_base.to_string(), system_base.to_string(), dir_blocks);
    }

    #[test]
    fn test_download_basic_case(){
        // Start creating a basic file structure
        // If a test fails, this structure might be already created, so delete if first
        let base_path = "./test_download_basic_case";
        remove_basic_file_structure(base_path);
        create_basic_file_structure(base_path).expect("Could not create basic file structure for the test");

        // Now get the basic DirectoriesDescr
        let description = create_basic_dir_description(base_path);

        // Get the dir description
        description.download_from_repo_to_system();

        // Make some checks about directories
        assert!(Path::new(base_path).join("system").exists(), "Directories were not properly downloaded");
        assert!(Path::new(base_path).join("system/alternative_src").exists(), "Directories were not properly downloaded");
        assert!(Path::new(base_path).join("system/other_test_place").exists(), "Directories were not properly downloaded");

        // Now make some checks about files
        assert_eq!(Path::new(base_path).join("system/alternative_src/first.rs").exists(), false, "Ignored file was not ignored");
        assert!(Path::new(base_path).join("system/alternative_src/second.rs").exists(), "Dir sync failed to copy a file");
        assert!(Path::new(base_path).join("system/alternative_src/third.rs").exists(), "Dir sync failed to copy a file");
        assert!(Path::new(base_path).join("system/other_test_place/first_test___.rs").exists(), "File sync failed to make the copy");
    }
}
