use std::fs;

use punto::dir_sync::handle_download;

mod common;

#[test]
fn test_basic_case() {
    // Create the basic temp dir with the files
    let handler = common::setup_basic_scenario("test_basic_case");
    let tmp_path_str = handler
        .dir
        .path()
        .to_str()
        .expect("Could not convert path to string");

    // Read the toml, replace the placeholders with the temp path and write the config inside the
    // temp dir
    let toml_config_str = fs::read_to_string("./tests/configs/basic_config.toml")
        .expect("Could not read basic config toml")
        .replace("{{tmp_dir}}", tmp_path_str);
    let config_path = handler.dir.path().join("config.toml");
    fs::write(&config_path, toml_config_str).expect("Could not create toml config in tmp dir");

    // Launch the download
    handle_download(
        config_path
            .to_str()
            .expect("Could not convert config path to str"),
    );

    // Assert that file system ends up as expected
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
