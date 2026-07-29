mod common;
use lib_fileops::file_operations;
use lib_fileops::sync_options::{SyncOptions, SyncOptionsBuilder};
use std::fs;

/// Test that we have implemented **expected behaviour** when exluding a pattern and setting delete
/// files at destination flag. That expected behaviour is that, if we are ignoring files and they
/// are present in destination, they should be removed
///
/// We start with the `common::setup_basic_but_messy_scenario` call that creates the following file
/// structure:
/// ```
/// `tmp_dir.path()`
/// |_ origin/
///      |__ file1.txt
///      |__ file2.txt
///      |____ dir1/
///          |__ a.txt
///          |__ b.txt
///          |__ c.txt
/// |_ destination/
///      |__ file1.txt
///      |__ file3.txt
///      |____ dir1/
///          |__ c.txt
///          |__ d.txt
/// ```
///
/// Then we ignore `dir1` and set `delete_files_destination`, so we expect:
///
/// ```
/// `tmp_dir.path()`
/// |_ origin/
///      |__ file1.txt
///      |__ file2.txt
///      |____ dir1/
///          |__ a.txt
///          |__ b.txt
///          |__ c.txt
/// |_ destination/
///      |__ file1.txt -
///      |__ file2.txt A
///      |__ file3.txt D
///      |____ dir1/   D
///          |__ c.txt D
///          |__ d.txt D
/// ```
#[test]
fn test_exclude_and_delete() {
    // Create the temp file structure
    let handler = common::setup_basic_but_messy_scenario("test_exclude_and_delete");

    // Sync one dir into another ignoring the inner dir
    // We put deletion at destination, so we expect it to be deleted
    let from = handler.dir.path().join("origin");
    let to = handler.dir.path().join("destination");
    let sync_options = SyncOptionsBuilder::new()
        .exclude_patterns(vec!["dir1".into()])
        .delete_files_destination(true)
        .build();
    file_operations::sync_dir(from, to, sync_options).expect("sync_dir failed");

    // Check the expected file structure that we have disclosed in the test comment
    assert!(handler.dir.path().join("destination").exists());
    for file in ["file1.txt", "file2.txt"] {
        assert!(handler.dir.path().join("destination").join(file).exists());
    }
    assert!(!handler
        .dir
        .path()
        .join("destination")
        .join("file3.txt")
        .exists());
    assert!(handler.dir.path().join("destination").join("dir1").exists());
}
