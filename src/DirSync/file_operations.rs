/// Module to implement file operations such as copy files, copy dirs, create dirs, ...

use std::{fs, path::Path, process::exit};

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
pub fn copy_dir_recursively(from: &str, to: &str) {

    // copy_items expects a vector of files and dirs to copy from
    let from = vec![from];

    // Options to the copy operation
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;

    // Append a trailing / if not present in to
    let to = append_trailing_slash_if_not_present(to);

    match fs_extra::copy_items(&from, to.clone(), &copy_options) {
        Err(err) => {
            eprintln!("Error copying dir {} to dir {}", from[0], to.clone());
            eprintln!("Error code was {}", err);
            exit(-1);
        }
        Ok(_) => (),
    };
}

fn append_trailing_slash_if_not_present(to: &str) -> String{
    let mut new_to = to.to_string();

    let last_char = *to.as_bytes().last().unwrap() as char;

    if last_char as char != '/'{
        new_to.push('/');
    }

    return new_to;
}

/// Syncs two dirs.
/// If the destionation dir does not exists, it gets created
/// Sync means files and dirs not present in from path are deleted in to path if they are present
/// there
pub fn sync_dir(from: &str, to: &str) {

    // In order to have sync behaviour, delete all the contents of the destination dir
    remove_dir_and_contents(to);

    // Create the destination dir if does not exist (it should always not exists)
    create_dir_if_not_exists(to);

    // Copy contents recursively
    copy_dir_recursively(from, to);
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
