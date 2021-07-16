use crate::DirSync::dir_file_type::DirFileType;

/// Represent a dir block inside yaml description
/// A Dir Block is represented by the path of the git repo, the path in the system and the type of
/// sync to be performed
#[derive(Debug)]
pub struct DirBlock {
    repo_path: String,
    system_path: String,
    sync_type: DirFileType,

    /// Files to ignore
    /// TODO -- BUG -- now we are deleting them, not ignoring them
    ignore_files: Vec<String>,
}

impl DirBlock {
    pub fn new(repo_path: String, system_path: String, sync_type: DirFileType, ignore_files: Vec<String>) -> Self {
        return DirBlock {
            repo_path,
            system_path,
            sync_type,
            ignore_files,
        };
    }

    pub fn repo_path(&self) -> &String{
        return &self.repo_path;
    }

    pub fn system_path(&self) -> &String{
        return &self.system_path;
    }

    pub fn sync_type(&self) -> &DirFileType{
        return &self.sync_type;
    }

    pub fn ignore_files(&self) -> Vec<String> {
        return self.ignore_files.clone();
    }
}

