/// Module to implement file operations such as copy files, copy dirs, create dirs, ...

use std::{fs, path::Path, process::exit};

/// Creates recursively a dir if does not exist
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
    let from = vec![from];

    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;
    let copy_options = copy_options;

    match fs_extra::copy_items(&from, to, &copy_options) {
        Err(err) => {
            eprintln!("Error copying dir {} to dir {}", from[0], to);
            eprintln!("Error code was {}", err);
            exit(-1);
        }
        Ok(_) => (),
    };
}

// TODO -- BUG -- not removing deprecated files
// ie, when wallpaper is removed, sync_dir wont remove that wallpaper on to
// dir
/// Syncs two dirs.
/// If the destionation dir does not exists, it gets created
pub fn sync_dir(from: &str, to: &str) {
    let to_parent_dir = parent_dir(to);
    create_dir_if_not_exists(to_parent_dir);
    copy_dir_recursively(from, to_parent_dir);
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

