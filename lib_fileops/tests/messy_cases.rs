mod common;
use lib_fileops::file_operations;
use lib_fileops::sync_options::{SyncOptions, SyncOptionsBuilder};

#[test]
fn test_messy_without_deleting_at_destination() {
    // Create the temp file structure
    let handler =
        common::setup_basic_but_messy_scenario("test_messy_without_deleting_at_destination");

    // Sync one dir into another
    // Note that we are using default options which does not remove files at destination
    let from = handler.dir.path().join("origin");
    let to = handler.dir.path().join("destination");
    let sync_options = SyncOptions::default();
    file_operations::sync_dir(from, to, sync_options).expect("sync_dir failed");

    // Check the expected file structure
    assert!(handler.dir.path().join("destination").exists());
    for file in ["file1.txt", "file2.txt", "file3.txt"] {
        assert!(handler.dir.path().join("destination").join(file).exists());
    }

    assert!(handler.dir.path().join("destination").join("dir1").exists());
    for file in ["a.txt", "b.txt", "c.txt", "d.txt"] {
        assert!(handler
            .dir
            .path()
            .join("destination")
            .join("dir1")
            .join(file)
            .exists());
    }
}

#[test]
fn test_messy_with_deleting_at_destination() {
    // Create the temp file structure
    let handler =
        common::setup_basic_but_messy_scenario("test_messy_with_deleting_at_destination");

    // Sync one dir into another
    // We use the builder to have default values and set the deletion flag to true
    let from = handler.dir.path().join("origin");
    let to = handler.dir.path().join("destination");
    let sync_options = SyncOptionsBuilder::new()
        .delete_files_destination(true)
        .build();
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

    // Make sure that files that were in destination but not in origin get deleted
    assert!(!handler
        .dir
        .path()
        .join("destination")
        .join("file3.txt")
        .exists());

    assert!(!handler
        .dir
        .path()
        .join("destination")
        .join("dir1")
        .join("d.txt")
        .exists());
}
