use lib_fileops::sanitize_relative_path;

/// Indicate if a path refers to a directory or a file
#[derive(Debug, Clone)]
pub enum DirFileType {
    File,
    Dir,
}

/// Represent a dir block inside a `DirectoriesDescr`
/// A Dir Block represents:
///     1. The relative path of the file or dir inside the git repo
///     2. The relative path of the file or dir inside the system
///     3. Wether we are working with a file or with a dir
///     4. If we are working with dirs, relative paths that we want to
///        exclude
#[derive(Debug)]
pub struct DirBlock {
    /// Path relative to DirDescr::repo_base
    repo_path: String,

    /// Path relative to DirDescr::system_base
    system_path: String,

    /// Type of sync mechanism
    /// Wether we are working with files or directories
    sync_type: DirFileType,

    /// Files to ignore
    /// Should be relative to `repo_path`
    /// For example, 'file.txt' instead of '/path/to/repo/ignore_files'
    ignore_files: Vec<String>,
}

impl DirBlock {
    pub fn new(repo_path: String, system_path: String, sync_type: DirFileType, ignore_files: Vec<String>) -> Self {
        return DirBlock {
            repo_path: sanitize_relative_path(&repo_path),
            system_path: sanitize_relative_path(&system_path),
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

