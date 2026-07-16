mod common;
use lib_fileops::file_operations;
use lib_fileops::sync_options::SyncOptions;

#[test]
fn test_test() {
    // Create the temp file structure
    let handler = common::setup_basic_scenario("simple_test");

    // Sync one dir into another
    let from = handler.dir.path().join("origin");
    let to = handler.dir.path().join("destination");
    let sync_options = SyncOptions::default();
    file_operations::sync_dir(from, to, sync_options).expect("sync_dir failed");

    // Check the expected file structure
    assert!(handler.dir.path().join("destination").exists());
    for file in ["file1.txt", "file2.txt"] {
        assert!(handler.dir.path().join("destination").join(file).exists());
    }

    assert!(handler.dir.path().join("destination").join("dir1").exists());
    for file in ["a.txt", "b.txt", "c.txt"] {
        assert!(handler
            .dir
            .path()
            .join("destination")
            .join("dir1")
            .join(file)
            .exists());
    }
}
