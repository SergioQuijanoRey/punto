use std::{fs, path::Path};

/// Module to implement basic file operations such as copy files, copy dirs,
/// create dirs, ...

use lib_commands::SingleCommand;

#[derive(Debug)]
pub enum FileOperationError{

    // Error ocurred while copying one dir to another dir
    SyncDirError(String),

    /// Copying one file to another place failed
    FileCopyError(String),
}

/// Gets a path and adds a last "/" if it is not present
/// This is needed for the rsync command
///
/// # Examples
/// ```
/// # some/path -> some/path/
/// let original_path = "some/path";
/// let transformted_path = add_last_slash_to_path(original_path);
/// assert_eq!(transformted_path, "some/path/", "add_last_slash_to_path did not added last slash");
///
/// # other/path/ -> do nothing
/// let original_path = "some/path/";
/// let transformted_path = add_last_slash_to_path(original_path);
/// assert_eq!(transformted_path, "some/path/", "add_last_slash_to_path changed a path that was correct at first");
/// ```
fn add_last_slash_to_path(path: &str) -> String{
    let last_char = path.chars().last().unwrap();

    if last_char == '/' {
        return path.to_string();
    }


    let mut transformted_path = path.to_string();
    transformted_path.push('/');

    return transformted_path;
}

/// Syncs two paths
/// `ignore_paths` can be both file and dir paths
/// `ignore_paths` must be relative paths based on `from` path
///
/// If `remove_files` is true, files and dirs that are not present in `from` path but are present
/// in `to` path will be removed
///
// TODO -- we're using rsync to do this, move that to native rust code
pub fn sync_dir(from: &str, to: &str, ignore_paths: &Vec<String>, remove_files: bool) -> Result<(), FileOperationError>{

    // For using rsync, last char in the paths must be /
    // So make some checks and do the conversion if they fail
    // i.e. some/path -> some/path/
    let from = add_last_slash_to_path(&from);
    let to = add_last_slash_to_path(&to);

    // Build a bash command based on rsync to perform the operation
    // This has to be done in four steps due to ignore_files and remove_files nature

    // Step 1: create the base of the command string
    let mut command_content = format!("rsync -zaP ");

    // Step 2: add this flag, so rsync creates the necessary dirs if they don't
    // exist
    command_content.push_str("--mkpath ");

    // Step 3: check if we want to remove files
    if remove_files == true{
        command_content.push_str("--delete ");
    }

    // Step 4: add the ignored files
    if ignore_paths.is_empty() == false{
        for excluded_file in ignore_paths{
            command_content.push_str(&format!("--exclude {excluded_file} "));
        }
    }

    // Step 5: specify source and destination
    command_content.push_str(&format!("{from} {to}"));

    println!("TODO -- remove me -- command was {command_content}");

    let quiet = false;
    let sudo = false;
    let command = SingleCommand::new(
        command_content, quiet, sudo,
    );

    // Check that the command is valid
    let command = match command{
        Ok(result) => result,
        Err(err) => return Err(FileOperationError::SyncDirError(format!("rsync command creation failed: {err:?}"))),
    };

    // Run the command
    let command_result = command.run();

    // Check for errors
    // Translate SingleCommandError to a higher level of abstraction error
    let operation_result = match command_result {
        Ok(_) => Ok(()),
        Err(err) => Err(FileOperationError::SyncDirError(format!("Rsync error was {err:?}"))),
    };

    return operation_result;
}

/// Copies one file to another location
/// Creates the `to` folder if it does not exist
pub fn sync_file(from: &str, to: &str) -> Result<(), FileOperationError>{

    // Create the path to store the file if it does not exist
    let parent_dir = match Path::new(to).parent() {
        Some(path) => path,
        None => return Err(FileOperationError::FileCopyError("Could not retreive parent of dest file".to_string())),
    };

    // Now we get the string from the parent object
    let parent_dir = match parent_dir.to_str(){
        Some(path) => path,
        None => return Err(FileOperationError::FileCopyError("Could not get the string from the Path object".to_string())),
    };

    let create_dir_result = fs::create_dir_all(parent_dir);
    match create_dir_result{
        Ok(_) => (),
        Err(err) => return Err(FileOperationError::FileCopyError(format!("Could not create dirs for the new file, err was: {err}"))),
    }

    // Copy the file
    let result = fs::copy(from, to);

    // Return the result of the copy operation
    return match result{
        Ok(_) => return Ok(()),
        Err(err) => return Err(FileOperationError::FileCopyError(format!("File copy got following error: {err}"))),
    };
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

    use super::{
        add_last_slash_to_path,
        join_two_paths,
        sync_dir,
        sync_file
    };

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
    /// NOTE: do not share root folder, because tests might run in parallel
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

    // TODO -- BUG -- this test is failing
    #[test]
    fn test_sync_base_case_dirs(){

        let base_path = "test_sync_base_case_dirs";

        // Start creating a basic file structure
        // If a test fails, this structure might be already created, so delete if first
        remove_basic_file_structure(base_path);
        create_basic_file_structure(base_path)
            .expect("Could not create basic file structure for the test");

        // Copy now to another path
        let from = base_path;
        let to = Path::new(base_path).join("pruebas");
        let ignore_files = vec![];
        let remove_files = false;
        sync_dir(from, to.to_str().unwrap(), &ignore_files, remove_files).expect("Copy operation failed to run");

        // Make some checks about the dirs
        assert!(Path::new(base_path).join("pruebas/").exists(), "New dir hierarchy was not created properly");
        assert!(Path::new(base_path).join("pruebas/src").exists(), "New dir hierarchy was not created properly");
        assert!(Path::new(base_path).join("pruebas/test").exists(), "New dir hierarchy was not created properly");

        // Now check the paths
        assert!(Path::new(base_path).join("pruebas/src/first.rs").exists(), "New dir hierarchy was not created properly");
        assert!(Path::new(base_path).join("pruebas/src/second.rs").exists(), "New dir hierarchy was not created properly");
        assert!(Path::new(base_path).join("pruebas/src/third.rs").exists(), "New dir hierarchy was not created properly");

        assert!(Path::new(base_path).join("pruebas/test/first_test.rs").exists(), "New dir hierarchy was not created properly");
        assert!(Path::new(base_path).join("pruebas/test/second_test.rs").exists(), "New dir hierarchy was not created properly");

        // Now, remove the file hierarchy created
        remove_basic_file_structure(base_path);
    }

    #[test]
    fn test_sync_dir_ignore_files(){

        let base_path = "test_sync_dir_ignore_files";

        // Start creating a basic file structure
        // If a test fails, this structure might be already created, so delete if first
        remove_basic_file_structure(base_path);
        create_basic_file_structure(base_path)
            .expect("Could not create basic file structure for the test");

        // Copy now to another path
        let from = base_path;

        let binding = Path::new(base_path).join("pruebas");
        let to = binding.to_str().unwrap();

        let ignore_files = vec!["src/first.rs".to_string(), "src/second.rs".to_string()];
        let remove_files = false;
        sync_dir(from, to, &ignore_files, remove_files).expect("Copy operation failed to run");

        // Make some checks about the dirs
        assert!(Path::new(base_path).join("pruebas/").exists(), "New dir hierarchy was not created properly");
        assert!(Path::new(base_path).join("pruebas/src").exists(), "New dir hierarchy was not created properly");
        assert!(Path::new(base_path).join("pruebas/test").exists(), "New dir hierarchy was not created properly");

        // Now check the paths
        assert_eq!(Path::new(base_path).join("pruebas/src/first.rs").exists(), false, "Ignored file is present");
        assert_eq!(Path::new(base_path).join("pruebas/src/second.rs").exists(), false, "Ignored file is present");
        assert!(Path::new(base_path).join("pruebas/src/third.rs").exists(), "New dir hierarchy was not created properly");


        assert!(Path::new(base_path).join("pruebas/test/first_test.rs").exists(), "New dir hierarchy was not created properly");
        assert!(Path::new(base_path).join("pruebas/test/second_test.rs").exists(), "New dir hierarchy was not created properly");

        // Now, remove the file hierarchy created
        remove_basic_file_structure(base_path);
    }


    // TODO -- BUG -- This is failing
    #[test]
    fn test_sync_file_base_case(){

        let base_path = "test_sync_file_base_case";

        // Start creating a basic file structure
        // If a test fails, this structure might be already created, so delete if first
        remove_basic_file_structure(base_path);
        create_basic_file_structure(base_path)
            .expect("Could not create basic file structure for the test");

        // Sync just a single file
        let from = "./dir_tests/src/first.rs";
        let to = "./dir_tests/pruebas/code/first.rs";
        sync_file(from, to).expect("Copy operation failed to run");

        // Check that the dir for the file was created
        assert!(Path::new("./dir_tests/pruebas/code").exists(), "Dir for the new file was not created");

        // Now check that the file itself exists
        assert!(Path::new("./dir_tests/pruebas/code/first.rs").exists(), "File was not properly copyed");

        // Now, remove the file hierarchy created
        remove_basic_file_structure(base_path);
    }
}
