use std::{fs, path::Path};
use anyhow::Context;
use thiserror::Error;
use folder_compare::FolderCompare;

/// Module to implement basic file operations such as copy files, copy dirs,
/// create dirs, ...

use lib_commands::SingleCommand;

/// Gets a path and adds a last "/" if it is not present
/// This is needed for the rsync command
///
/// For example:
/// some/path -> some/path/
/// other/path/ -> do nothing
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
pub fn sync_dir(from: &str, to: &str, ignore_paths: &Vec<String>, remove_files: bool) -> anyhow::Result<()>{

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

    let quiet = false;
    let sudo = false;
    let command = SingleCommand::new(
        command_content, quiet, sudo,
    ).context("Could not create the command to use rsync")?;

    // Run the command
    command.run().context("Rsync command failed at runtime")?;
    return Ok(());
}

/// Copies one file to another location
/// Creates the `to` folder if it does not exist
pub fn sync_file(from: &str, to: &str) -> anyhow::Result<()> {

    // Get the path to the parent dir of `to` file
    let parent_dir = Path::new(to).parent()
        .with_context(|| format!("Could not get the path of the parent dir of dest. file {}", to))?
        .to_str().with_context(|| format!("Could not get the string of the parent dir of dest file {}", to))?;

    // Create the dir for the new file
    fs::create_dir_all(parent_dir)
        .with_context(|| format!("Could not create dir {} to store new file", parent_dir))?;

    // Copy the file to the new dir
    fs::copy(from, to).context(format!("Failed to copy file from {} to {}", from, to))?;

    return Ok(());
}

/// Joins two paths given in strings
///
/// # Examples
/// ```
/// use lib_fileops::join_two_paths;
/// let joined = join_two_paths("first_part", "second_part");
/// let expected = "first_part/second_part";
/// assert_eq!(expected, joined, "Join two paths func did not work properly");
/// ```
pub fn join_two_paths(first: &str, second: &str) -> String{
    let second_sanitized = sanitize_relative_path(second);
    let joined_path = std::path::Path::new(first).join(second_sanitized).to_str().unwrap().to_string();
    return joined_path;
}

/// Relative paths that are stored in a string in the form of "./something" are
/// dangerous. Some functions fail when passing relative paths like that
/// So here we sanitize that relative paths
///
/// Also, joining two paths in the form of "/something" "/other" is problematic,
/// so we also handle that case
///
/// # Examples
///
/// ```
/// use lib_fileops::sanitize_relative_path;
/// let computed = sanitize_relative_path("./some/rel/path");
/// let expected = "some/rel/path";
/// assert_eq!(expected, computed, "Relative path sanitizer did not work well");
///
/// let computed = sanitize_relative_path("/some/rel/path");
/// let expected = "some/rel/path";
/// assert_eq!(expected, computed, "Relative path sanitizer did not work well");
/// ```
pub fn sanitize_relative_path(rel_path: &str) -> String {

    if &rel_path[0..1] == "/" {
        let sanitized: &str = &rel_path[1..rel_path.len()];
        return sanitized.to_string();
    }

    if &rel_path[0..2] == "./" {
        let sanitized: &str = &rel_path[2..rel_path.len()];
        return sanitized.to_string();

    }

    return rel_path.to_string();
}

/// `folder_compare::Error` does not implement `Error` trait, which is needed
/// for using anyhow. So this enum takes a `folder_compare::Error` and implements
/// the traits we need
#[derive(Error, Debug)]
enum DiffError{
    #[error("Error while computing the diff between two dirs, reason: {inner_error:?}")]
    DiffError {
        inner_error: folder_compare::Error,
    }
}

/// Given two folders, defined by paths `first_path` and `second_path`, returns
/// the list of files that are present in the second dir but not present in the
/// first dir
pub fn get_dir_diff(first_path: &str, second_path: &str) -> anyhow::Result<Vec<String>> {

    let excluded = vec![];
    let new_files = FolderCompare::new(
        Path::new(second_path),
        Path::new(first_path),
        &excluded
    )
    // Use our custom error type so we can use anyhow
    .map_err(|inner| DiffError::DiffError{inner_error: inner})
    .context(format!("An error ocurred while diffing {first_path} and {second_path}"))?
    .new_files;

    // We want the strings out of the `PathBuf` objects
    let new_files: Vec<String> = new_files.iter().map(|pathbuf| pathbuf.to_str().unwrap().to_string()).collect();
    return Ok(new_files);
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::{
        add_last_slash_to_path,
        join_two_paths,
        sync_dir,
        sync_file,
        sanitize_relative_path,
        get_dir_diff,
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
    fn test_join_two_paths_with_relative_paths(){
        let computed = join_two_paths("some/path/", "./relative/path");
        let expected = "some/path/relative/path";
        assert_eq!(expected, computed, "Relative paths are not joined properly");
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

    #[test]
    fn test_sanitizer_works() {

        let computed = sanitize_relative_path("./some/rel/path");
        let expected = "some/rel/path";
        assert_eq!(expected, computed, "Relative path sanitizer did not work well");

        let computed = sanitize_relative_path("/some/rel/path");
        let expected = "some/rel/path";
        assert_eq!(expected, computed, "Relative path sanitizer did not work well");
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

    #[test]
    fn test_sync_file_base_case(){

        let base_path = "test_sync_file_base_case";

        // Start creating a basic file structure
        // If a test fails, this structure might be already created, so delete if first
        remove_basic_file_structure(base_path);
        create_basic_file_structure(base_path)
            .expect("Could not create basic file structure for the test");

        // Sync just a single file
        let from = Path::new(base_path).join("src").join("first.rs");
        let to = Path::new(base_path).join("pruebas/code").join("first.rs");
        sync_file(from.to_str().unwrap(), to.to_str().unwrap()).expect("Copy operation failed to run");

        // Check that the dir for the file was created
        assert!(Path::new(base_path).join("pruebas/code").exists(), "Dir for the new file was not created");

        // Now check that the file itself exists
        assert!(to.exists(), "File was not properly copyed");

        // Now, remove the file hierarchy created
        remove_basic_file_structure(base_path);
    }

    #[test]
    fn test_get_diff_dir_basic_case(){
        let base_path = "./test_get_diff_dir_basic_case";
        let other_path = "./test_get_diff_dir_basic_case_other_path";

        // Start creating a basic file structure
        // If a test fails, this structure might be already created, so delete if first
        remove_basic_file_structure(base_path);
        create_basic_file_structure(base_path)
            .expect("Could not create basic file structure for the test");

        // Use the same hierarchy in other place
        create_basic_file_structure(other_path)
            .expect("Could not create basic file structure for the test");

        // Create a file that is in one place but not in the other
        let new_file_path = Path::new(other_path).join("test/this_file_is_new.rs");
        fs::File::create(&new_file_path).unwrap();

        // Compute one diff and check the result
        // A single new file should be detected
        let new_files = get_dir_diff(base_path, other_path).unwrap();
        let expected_new_files = vec![new_file_path.to_str().unwrap().to_string()];
        assert_eq!(new_files, expected_new_files, "Diff dir did not found a new file");

        // Compute the other diff and check the result
        // This time no new files should be detected
        let new_files = get_dir_diff(other_path, base_path).unwrap();
        let expected_new_files: Vec<String> = vec![];
        assert_eq!(new_files, expected_new_files, "Diff dir found new files when no one should be found");
    }
}
