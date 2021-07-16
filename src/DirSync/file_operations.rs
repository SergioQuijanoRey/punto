/// Module to implement file operations such as copy files, copy dirs, create dirs, ...

use std::{fs, path::Path, process::exit, thread, time};

/// Creates recursively a dir if does not exist
// TODO -- not sure if it is creating dir recursively if depth is greater than 2
pub fn create_dir_if_not_exists(path: &str) {
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
// TODO -- BUG -- ignore_files behaviour is not correct
//      -- We are deleting ignore_files instead of actually ignoring them;
pub fn copy_dir_recursively(from: &str, to: &str, ignore_files: &Vec<String>) {
    // Options that we need to pass to copy_dir_advanced
    let overwrite_all = true;
    let overwrite_if_newer = true;
    let overwrite_if_size_differs = true;
    let include_filters : Vec<String> = Vec::new();

    let copy_result = dircpy::copy_dir_advanced(
        from,
        to,
        overwrite_all,
        overwrite_if_newer,
        overwrite_if_size_differs,
        ignore_files.to_vec(),
        include_filters
    );

    match copy_result{
        Ok(()) => (),
        Err(err) => {
            eprintln!("Error copying dir {} to {}", from, to);
            eprintln!("Err Code: {}", err);
        }
    }
}

/// Syncs two dirs.
/// If the destionation dir does not exists, it gets created
/// Sync means files and dirs not present in from path are deleted in to path if they are present
/// there
// TODO -- BUG -- ignore_files behaviour is not correct
//      -- As we are removing the destination dir, we are not ignoring the files, we are deleting
//         them
pub fn sync_dir(from: &str, to: &str, ignore_files: &Vec<String>) {

    // In order to have sync behaviour, delete all the contents of the destination dir
    remove_dir_and_contents(to);

    // Create the destination dir if does not exist (it should always not exists)
    create_dir_if_not_exists(to);

    // Copy contents recursively
    copy_dir_recursively(from, to, ignore_files);
}

/// Gets the str path of the parent dir
/// If to is a file path, gets is dir where its allocated
/// If to is a dir, gets its parent dir
pub fn parent_dir(to: &str) -> &str{
    let to_parent_dir = std::path::Path::new(to).parent().expect(&format!("Could not get parent dir of {}", to));
    return to_parent_dir.to_str().expect(&format!("Could not get string from {} parent", to));
}

/// Syncs two files.
/// If the destination file is inside a dir that does not exists, creates it
pub fn sync_file(from: &str, to: &str) {
    // Create parent dir if not exists
    let to_parent_dir = parent_dir(to);
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


/// Removes a given dir and all the contents
/// Dir does not have to be empty, all contents inside dir will be deleted
fn remove_dir_and_contents(dir_path: &str){

    // Check if the dir already exists
    // If it doesn't exist, do nothing
    if dir_exists(dir_path) == false{
        return;
    }

    let result = std::fs::remove_dir_all(dir_path);
    match result{
        Ok(()) => (),
        Err(err) => {
            eprintln!("Error trying to delete dir {} and all of its contents", dir_path);
            eprintln!("Error code was: {}", err);
            exit(-1);
        }
    }

}

/// Checks if a given dir exists
fn dir_exists(dir_path: &str) -> bool{
    return Path::new(dir_path).is_dir();
}

/// Joins two paths given in strings
///
/// # Examples
/// ```
/// let joined = join_two_paths("first_part", "second_part");
/// let expected = "first_part/second_part";
/// assert_eq!(expected, joined);
/// ```
pub fn join_two_paths(first: &str, second: &str) -> String{
    return std::path::Path::new(first).join(second).to_str().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::join_two_paths;

    #[test]
    fn test_join_two_paths_basic() {

        let computed = join_two_paths("testing", "this");
        let expected = "testing/this";
        assert_eq!(expected, computed);
    }

    #[test]
    fn test_join_two_paths_trailing_slashes(){

        let computed = join_two_paths("testing/", "this");
        let expected = "testing/this";
        assert_eq!(expected, computed);

        let computed = join_two_paths("testing", "this/");
        let expected = "testing/this/";
        assert_eq!(expected, computed);

        let computed = join_two_paths("testing/", "this/");
        let expected = "testing/this/";
        assert_eq!(expected, computed);
    }
}
