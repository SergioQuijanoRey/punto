use std::fs;

use punto::dir_sync::handle_download;

mod common;

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
/// Then, the toml config ignores `dir1` and set `delete_files_destination=true`, so we expect:
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
fn test_ignore_and_delete_at_destination() {
    let handler = common::setup_basic_but_messy_scenario("test_ignore_and_delete");
    let tmp_path_str = handler
        .dir
        .path()
        .to_str()
        .expect("Could not convert path to string");

    let toml_config_str = fs::read_to_string("./tests/configs/messy_config_ignore_and_delete.toml")
        .expect("Could not read messy_config_ignore_and_delete.toml")
        .replace("{{tmp_dir}}", tmp_path_str);
    let config_path = handler.dir.path().join("config.toml");
    fs::write(&config_path, toml_config_str).expect("Could not write config to tmp dir");

    handle_download(
        config_path
            .to_str()
            .expect("Could not convert config path to str"),
    );

    let dest = handler.dir.path().join("destination");

    // Check the expected file structure that we have disclosed in the test comment
    for file in ["file1.txt", "file2.txt"] {
        assert!(dest.join(file).exists(), "{} should exist", file);
    }
    assert!(
        !dest.join("file3.txt").exists(),
        "file3.txt should be deleted"
    );
    assert!(
        dest.join("dir1").exists(),
        "dir1 should be preserved (it is ignored)"
    );
}
