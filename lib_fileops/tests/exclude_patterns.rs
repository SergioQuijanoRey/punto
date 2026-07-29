mod common;
use lib_fileops::file_operations;
use lib_fileops::sync_options::{SyncOptions, SyncOptionsBuilder};
use std::fs;

#[test]
fn test_simple_ignore() {
    // Create the temp file structure
    let handler = common::setup_basic_scenario("test_simple_ignore");

    // Sync one dir into another ignoring the inner dir
    let from = handler.dir.path().join("origin");
    let to = handler.dir.path().join("destination");
    let sync_options = SyncOptionsBuilder::new()
        .exclude_patterns(vec!["dir1".into()])
        .build();
    file_operations::sync_dir(from, to, sync_options).expect("sync_dir failed");

    // Check the expected file structure
    assert!(handler.dir.path().join("destination").exists());
    for file in ["file1.txt", "file2.txt"] {
        assert!(handler.dir.path().join("destination").join(file).exists());
    }

    // Check that ignored files and dirs are not present in the destination
    assert!(!handler.dir.path().join("destination").join("dir1").exists());
    for file in ["a.txt", "b.txt", "c.txt"] {
        assert!(!handler
            .dir
            .path()
            .join("destination")
            .join("dir1")
            .join(file)
            .exists());
    }
}

#[test]
fn test_simple_ignore_with_trailing_slash() {
    let handler = common::setup_basic_scenario("test_simple_ignore_trailing_slash");

    let from = handler.dir.path().join("origin");
    let to = handler.dir.path().join("destination");
    let sync_options = SyncOptionsBuilder::new()
        .exclude_patterns(vec!["dir1/".into()])
        .build();
    file_operations::sync_dir(from, to, sync_options).expect("sync_dir failed");

    for file in ["file1.txt", "file2.txt"] {
        assert!(handler.dir.path().join("destination").join(file).exists());
    }

    assert!(!handler.dir.path().join("destination").join("dir1").exists());
}

#[test]
fn test_filetype_exclude() {
    // Create the temp file structure
    let handler = common::setup_basic_scenario("test_simple_ignore");

    // Add some rust files
    let file_path = &handler.dir.path().join("origin").join("src1.rs");
    fs::File::create(file_path).expect("Could not create temp rust file for testing");

    let file_path = &handler
        .dir
        .path()
        .join("origin")
        .join("dir1")
        .join("src2.rs");
    fs::File::create(file_path).expect("Could not create temp rust file for testing");

    // Sync one dir into another ignoring all rust files with a glob
    let from = handler.dir.path().join("origin");
    let to = handler.dir.path().join("destination");
    let sync_options = SyncOptionsBuilder::new()
        .exclude_patterns(vec!["**.rs".into()])
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

    // Check that ignored files and dirs are not present in the destination but they exist in the origin
    assert!(handler.dir.path().join("origin").join("src1.rs").exists());
    assert!(!handler
        .dir
        .path()
        .join("destination")
        .join("src1.rs")
        .exists());

    assert!(handler
        .dir
        .path()
        .join("origin")
        .join("dir1")
        .join("src2.rs")
        .exists());
    assert!(!handler
        .dir
        .path()
        .join("destination")
        .join("dir1")
        .join("src2.rs")
        .exists());
}
