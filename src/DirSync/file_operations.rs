/// Module to implement file operations such as copy files, copy dirs, create dirs, ...

use std::{fs, path::Path, process::exit};

use crate::SingleCommand;

#[derive(Debug)]
pub enum FileOperationError{

    // Error ocurred while copying one dir to another dir
    DirCopyError(String),
}

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

/// Gets a path and adds a last "/" if it is not present
/// i.e. some/path -> some/path/
/// i.e other/path/ -> do nothing
fn add_last_slash_to_path(path: &str) -> String{
    let last_char = path.chars().last().unwrap();

    if last_char == '/' {
        return path.to_string();
    }


    let mut transformted_path = path.to_string();
    transformted_path.push('/');

    return transformted_path;
}

/// Copies one dir to other recursively
/// Files in the `to` dir that are not present in the `from` dir are preserved
// TODO -- we're using rsync to do this, move that to native rust code
// TODO -- needs a lots of testing
// TODO -- BUG -- we are not ignoring files
pub fn copy_dir_recursively(from: &str, to: &str, ignore_files: &Vec<String>) -> Result<(), FileOperationError>{

    // For using rsync, last char in the paths must be /
    // So make some checks and do the conversion if they fail
    // i.e. some/path -> some/path/
    let from = add_last_slash_to_path(&from);
    let to = add_last_slash_to_path(&to);

    // Build a bash command based on rsync to perform the operation
    // This has to be done in three steps due to ignore_files nature

    // Step 1: create the base of the command string
    let mut command_content = format!("rsync -zaP ");

    // Step 2: add the ignored files
    if ignore_files.is_empty() == false{
        for excluded_file in ignore_files{
            command_content.push_str(&format!("--exclude {excluded_file} "));
        }
    }

    // Step 3: specify source and destination
    command_content.push_str(&format!("{from} {to}"));

    println!("TODO -- remove me -- command is {command_content}");


    let quiet = false;
    let sudo = false;
    let command = SingleCommand::SingleCommand::new(
        command_content, quiet, sudo,
    );

    // Check that the command is valid
    let command = match command{
        Ok(result) => result,
        Err(err) => return Err(FileOperationError::DirCopyError(format!("rsync command creation failed: {err:?}"))),
    };

    // Run the command
    let command_result = command.run();

    // Check for errors
    // Translate SingleCommandError to a higher level of abstraction error
    let operation_result = match command_result {
        Ok(_) => Ok(()),
        Err(err) => Err(FileOperationError::DirCopyError(format!("Rsync error was {err:?}"))),
    };

    return operation_result;
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
    use std::fs;
    use std::path::Path;

    use crate::DirSync::file_operations::add_last_slash_to_path;

    use super::{join_two_paths, copy_dir_recursively};

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

    #[test]
    fn test_add_last_slash_to_path(){
        let original_path = "some/path";
        let transformted_path = add_last_slash_to_path(original_path);
        assert_eq!(transformted_path, "some/path/", "add_last_slash_to_path did not added last slash");


        let original_path = "some/path/";
        let transformted_path = add_last_slash_to_path(original_path);
        assert_eq!(transformted_path, "some/path/", "add_last_slash_to_path changed a path that was correct at first");
    }

    /// A lot of tests need to work in top a file hierarchy structure
    /// So with this function we can create a basic structure
    fn create_basic_file_structure() -> Option<()>{
        fs::create_dir("./dir_tests").ok()?;
        fs::create_dir("./dir_tests/src").ok()?;
        fs::create_dir("./dir_tests/test").ok()?;
        fs::File::create("./dir_tests/src/first.rs").ok()?;
        fs::File::create("./dir_tests/src/second.rs").ok()?;
        fs::File::create("./dir_tests/src/third.rs").ok()?;
        fs::File::create("./dir_tests/test/first_test.rs").ok()?;
        fs::File::create("./dir_tests/test/second_test.rs").ok()?;

        return Some(());
    }

    /// Remove the basic file structure created with `create_basic_file_structure`
    fn remove_basic_file_structure() -> Option<()>{
        fs::remove_dir_all("./dir_tests").ok()?;

        return Some(());
    }

    #[test]
    fn test_copy_dir_recursively_base_case(){

        // Start creating a basic file structure
        // If a test fails, this structure might be already created, so delete if first
        remove_basic_file_structure();
        create_basic_file_structure().expect("Could not create basic file structure for the test");

        // Copy now to another path
        let from = "./dir_tests";
        let to = "./dir_tests/pruebas";
        let ignore_files = vec![];
        copy_dir_recursively(from, to, &ignore_files).expect("Copy operation failed to run");

        // Make some checks about the dirs
        assert!(Path::new("./dir_tests/pruebas/").exists(), "New dir hierarchy was not created properly");
        assert!(Path::new("./dir_tests/pruebas/src").exists(), "New dir hierarchy was not created properly");
        assert!(Path::new("./dir_tests/pruebas/test").exists(), "New dir hierarchy was not created properly");

        // Now check the paths
        assert!(Path::new("./dir_tests/pruebas/src/first.rs").exists(), "New dir hierarchy was not created properly");
        assert!(Path::new("./dir_tests/pruebas/src/second.rs").exists(), "New dir hierarchy was not created properly");
        assert!(Path::new("./dir_tests/pruebas/src/third.rs").exists(), "New dir hierarchy was not created properly");

        assert!(Path::new("./dir_tests/pruebas/test/first_test.rs").exists(), "New dir hierarchy was not created properly");
        assert!(Path::new("./dir_tests/pruebas/test/second_test.rs").exists(), "New dir hierarchy was not created properly");

        // Now, remove the file hierarchy created
        remove_basic_file_structure();
    }

    #[test]
    fn test_copy_dir_recursively_ignore_files(){

        // Start creating a basic file structure
        // If a test fails, this structure might be already created, so delete if first
        remove_basic_file_structure();
        create_basic_file_structure().expect("Could not create basic file structure for the test");

        // Copy now to another path
        let from = "./dir_tests";
        let to = "./dir_tests/pruebas";
        let ignore_files = vec!["src/first.rs".to_string(), "src/second.rs".to_string()];
        copy_dir_recursively(from, to, &ignore_files).expect("Copy operation failed to run");

        // Make some checks about the dirs
        assert!(Path::new("./dir_tests/pruebas/").exists(), "New dir hierarchy was not created properly");
        assert!(Path::new("./dir_tests/pruebas/src").exists(), "New dir hierarchy was not created properly");
        assert!(Path::new("./dir_tests/pruebas/test").exists(), "New dir hierarchy was not created properly");

        // Now check the paths
        assert_eq!(Path::new("./dir_tests/pruebas/src/first.rs").exists(), false, "Ignored file is present");
        assert_eq!(Path::new("./dir_tests/pruebas/src/second.rs").exists(), false, "Ignored file is present");
        assert!(Path::new("./dir_tests/pruebas/src/third.rs").exists(), "New dir hierarchy was not created properly");


        assert!(Path::new("./dir_tests/pruebas/test/first_test.rs").exists(), "New dir hierarchy was not created properly");
        assert!(Path::new("./dir_tests/pruebas/test/second_test.rs").exists(), "New dir hierarchy was not created properly");

        // Now, remove the file hierarchy created
        remove_basic_file_structure();
    }
}
