use crate::DirSync::dir_file_type::DirFileType;

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

    pub fn repo_path(&self) -> &String{
        return &self.repo_path;
    }

    pub fn system_path(&self) -> &String{
        return &self.system_path;
    }

    pub fn sync_type(&self) -> &DirFileType{
        return &self.sync_type;
    }
}

