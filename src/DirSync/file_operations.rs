use std::{fs, path::Path, process::exit};

/// Module to implement file operations such as copy files, copy dirs, create dirs, ...

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
