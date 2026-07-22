use std::fs;

use punto::dir_sync::handle_download;

mod common;

/// Verify that extra files already present at the destination are kept when
/// `delete_files_destination` is not set (defaults to false).
#[test]
fn test_messy_without_deleting_at_destination() {
    let handler = common::setup_basic_but_messy_scenario("test_messy_no_delete");
    let tmp_path_str = handler
        .dir
        .path()
        .to_str()
        .expect("Could not convert path to string");

    let toml_config_str = fs::read_to_string("./tests/configs/messy_config_no_delete.toml")
        .expect("Could not read messy_config_no_delete.toml")
        .replace("{{tmp_dir}}", tmp_path_str);
    let config_path = handler.dir.path().join("config.toml");
    fs::write(&config_path, toml_config_str).expect("Could not write config to tmp dir");

    handle_download(
        config_path
            .to_str()
            .expect("Could not convert config path to str"),
    );

    let dest = handler.dir.path().join("destination");

    // Synced files from origin must be present
    for file in ["file1.txt", "file2.txt"] {
        assert!(dest.join(file).exists(), "{} should exist", file);
    }
    for file in ["a.txt", "b.txt", "c.txt"] {
        assert!(dest.join("dir1").join(file).exists(), "dir1/{} should exist", file);
    }

    // Pre-existing extra files must be preserved because deletion is disabled
    assert!(dest.join("file3.txt").exists(), "file3.txt should be kept");
    assert!(dest.join("dir1").join("d.txt").exists(), "dir1/d.txt should be kept");
}

/// Verify that extra files at the destination are removed when
/// `delete_files_destination = true`, leaving only what is in origin.
#[test]
fn test_messy_with_deleting_at_destination() {
    let handler = common::setup_basic_but_messy_scenario("test_messy_with_delete");
    let tmp_path_str = handler
        .dir
        .path()
        .to_str()
        .expect("Could not convert path to string");

    let toml_config_str = fs::read_to_string("./tests/configs/messy_config_with_delete.toml")
        .expect("Could not read messy_config_with_delete.toml")
        .replace("{{tmp_dir}}", tmp_path_str);
    let config_path = handler.dir.path().join("config.toml");
    fs::write(&config_path, toml_config_str).expect("Could not write config to tmp dir");

    handle_download(
        config_path
            .to_str()
            .expect("Could not convert config path to str"),
    );

    let dest = handler.dir.path().join("destination");

    // Only files mirrored from origin should remain
    for file in ["file1.txt", "file2.txt"] {
        assert!(dest.join(file).exists(), "{} should exist", file);
    }
    for file in ["a.txt", "b.txt", "c.txt"] {
        assert!(dest.join("dir1").join(file).exists(), "dir1/{} should exist", file);
    }

    // Extra files that were only in destination must have been deleted
    assert!(!dest.join("file3.txt").exists(), "file3.txt should be deleted");
    assert!(!dest.join("dir1").join("d.txt").exists(), "dir1/d.txt should be deleted");
}
