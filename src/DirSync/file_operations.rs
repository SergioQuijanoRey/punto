/// Module to implement file operations such as copy files, copy dirs, create dirs, ...

use crate::SingleCommand;

#[derive(Debug)]
pub enum FileOperationError{

    // Error ocurred while copying one dir to another dir
    DirCopyError(String),
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
/// `ignore_paths` can be both file and dir paths
/// `ignore_paths` must be relative paths based on `to` path
///
/// If `remove_files` is true, files and dirs that are not present in `from` path but are present
/// in `to` path will be removed
///
// TODO -- we're using rsync to do this, move that to native rust code
pub fn copy_dir_recursively(from: &str, to: &str, ignore_paths: &Vec<String>, remove_files: bool) -> Result<(), FileOperationError>{

    // For using rsync, last char in the paths must be /
    // So make some checks and do the conversion if they fail
    // i.e. some/path -> some/path/
    let from = add_last_slash_to_path(&from);
    let to = add_last_slash_to_path(&to);

    // Build a bash command based on rsync to perform the operation
    // This has to be done in four steps due to ignore_files and remove_files nature

    // Step 1: create the base of the command string
    let mut command_content = format!("rsync -zaP ");

    // Step 2: check if we want to remove files
    if remove_files == true{
        command_content.push_str("--delete ");
    }

    // Step 2: add the ignored files
    if ignore_paths.is_empty() == false{
        for excluded_file in ignore_paths{
            command_content.push_str(&format!("--exclude {excluded_file} "));
        }
    }

    // Step 3: specify source and destination
    command_content.push_str(&format!("{from} {to}"));

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

    use crate::DirSync::file_operations::{
        add_last_slash_to_path,
        join_two_paths,
        copy_dir_recursively
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
        let remove_files = false;
        copy_dir_recursively(from, to, &ignore_files, remove_files).expect("Copy operation failed to run");

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
        let remove_files = false;
        copy_dir_recursively(from, to, &ignore_files, remove_files).expect("Copy operation failed to run");

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

    // TODO -- TEST -- test remove_files behaviour
}
