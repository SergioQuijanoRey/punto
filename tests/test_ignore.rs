use std::fs;

use punto::dir_sync::handle_download;

mod common;

/// Verify that a whole directory listed in `ignore_paths` is not synced to destination.
#[test]
fn test_simple_ignore() {
    let handler = common::setup_basic_scenario("test_simple_ignore");
    let tmp_path_str = handler
        .dir
        .path()
        .to_str()
        .expect("Could not convert path to string");

    let toml_config_str = fs::read_to_string("./tests/configs/ignore_dir_config.toml")
        .expect("Could not read ignore_dir_config.toml")
        .replace("{{tmp_dir}}", tmp_path_str);
    let config_path = handler.dir.path().join("config.toml");
    fs::write(&config_path, toml_config_str).expect("Could not write config to tmp dir");

    handle_download(
        config_path
            .to_str()
            .expect("Could not convert config path to str"),
    );

    let dest = handler.dir.path().join("destination");

    // Top-level files not in the ignore list must be synced
    for file in ["file1.txt", "file2.txt"] {
        assert!(dest.join(file).exists(), "{} should exist", file);
    }

    // The ignored directory and its contents must not appear at destination
    assert!(!dest.join("dir1").exists(), "dir1 should not be synced");
}

/// Verify that a glob pattern in `ignore_paths` suppresses matching files across
/// all subdirectories while leaving non-matching files untouched.
#[test]
fn test_filetype_ignore() {
    let handler = common::setup_basic_scenario("test_filetype_ignore");
    let tmp_path_str = handler
        .dir
        .path()
        .to_str()
        .expect("Could not convert path to string");

    // Add extra .rs files to origin so the glob pattern has something to exclude
    let origin = handler.dir.path().join("origin");
    fs::File::create(origin.join("src1.rs")).expect("Could not create src1.rs");
    fs::File::create(origin.join("dir1").join("src2.rs"))
        .expect("Could not create dir1/src2.rs");

    let toml_config_str = fs::read_to_string("./tests/configs/ignore_filetype_config.toml")
        .expect("Could not read ignore_filetype_config.toml")
        .replace("{{tmp_dir}}", tmp_path_str);
    let config_path = handler.dir.path().join("config.toml");
    fs::write(&config_path, toml_config_str).expect("Could not write config to tmp dir");

    handle_download(
        config_path
            .to_str()
            .expect("Could not convert config path to str"),
    );

    let dest = handler.dir.path().join("destination");

    // Non-.rs files must be synced normally
    for file in ["file1.txt", "file2.txt"] {
        assert!(dest.join(file).exists(), "{} should exist", file);
    }
    for file in ["a.txt", "b.txt", "c.txt"] {
        assert!(dest.join("dir1").join(file).exists(), "dir1/{} should exist", file);
    }

    // .rs files matched by the glob must be absent from destination
    assert!(!dest.join("src1.rs").exists(), "src1.rs should be ignored");
    assert!(!dest.join("dir1").join("src2.rs").exists(), "dir1/src2.rs should be ignored");
}
